use actix::{Addr, MailboxError};

use crate::{
    actions::Actions,
    error::ServerError,
    events::{Event, ProgressConstraint, SubscriptionBuilder},
    pub_sub::{PubSubActor, ReplaceRulesMessage, ReplaceSubscriptionsMessage},
    rule_set::RuleSet,
};

pub async fn load_rules(pubsub: Addr<PubSubActor>, rules_path: &str) -> Result<usize, ServerError> {
    let rules = RuleSet::from_yaml(rules_path).map_err(|e| ServerError::RuleConfigurationError(e.to_string()))?;

    let msg = ReplaceRulesMessage {
        new_rules: rules.to_rules(),
    };

    let result = pubsub.send(msg).await?;
    Ok(result)
}

pub async fn load_subscriptions(pubsub: Addr<PubSubActor>) -> Result<usize, MailboxError> {
    let subscriptions = vec![
        SubscriptionBuilder::on("Ask for ACKs", Event::AcksNeeded)
            .and(ProgressConstraint::new().max_progress(99))
            .then(Actions::github().add_label("P-acks_required").build())
            .submit(),
        SubscriptionBuilder::on("Ask for reviews", Event::ReviewsNeeded)
            .and(ProgressConstraint::new().max_progress(99))
            .then(Actions::github().add_label("P-reviews_required").build())
            .submit(),
        SubscriptionBuilder::on("Acks achieved", Event::AcksThresholdReached)
            .then(Actions::github().remove_label("P-acks_required").build())
            .submit(),
        SubscriptionBuilder::on("Acks achieved", Event::ReviewsThresholdReached)
            .then(Actions::github().remove_label("P-reviews_required").build())
            .submit(),
    ];
    let msg = ReplaceSubscriptionsMessage {
        new_subscriptions: subscriptions,
    };
    pubsub.send(msg).await
}
