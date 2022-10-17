use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum PubSubError {
    #[error("An error occurred when trying to replace the server rules. {0}")]
    ReplaceRulesError(String),
    #[error("An error occurred when trying to add a server rules. {0}")]
    AddRuleError(String),
    #[error("Could not dispatch message to the relevant executor. {0}")]
    DispatchError(String),
    #[error("Could not complete an action due to missing information. {0}")]
    CouldNotCompleteAction(String),
}
