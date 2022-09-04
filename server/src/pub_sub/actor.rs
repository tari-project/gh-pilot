use actix::{Actor, Context, Handler, Running};
use log::{debug, trace};

use crate::pub_sub::GithubEventMessage;

#[derive(Default, Clone)]
pub struct PubSubActor;

impl Actor for PubSubActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        debug!("PubSub actor has started.");
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        debug!("PubSub actor is stopping.");
        Running::Stop
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        debug!("PubSub actor has stopped.");
    }
}

impl Handler<GithubEventMessage> for PubSubActor {
    type Result = ();

    fn handle(&mut self, msg: GithubEventMessage, ctx: &mut Self::Context) -> Self::Result {
        trace!("PubSub received github event message: {}", msg.name());
    }
}
