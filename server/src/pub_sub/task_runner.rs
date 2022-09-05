use actix::{Actor, Context, Handler, ResponseFuture, Running};
use log::*;
use crate::pub_sub::GithubEventMessage;
use crate::pub_sub::task_message::TaskExecuteMessage;

#[derive(Default)]
pub struct TaskRunner {}

impl TaskRunner {}
impl Actor for TaskRunner {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        debug!("TaskRunner actor has started.");
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        debug!("TaskRunner actor is stopping.");
        Running::Stop
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        debug!("TaskRunner actor has stopped.");
    }
}

impl Handler<TaskExecuteMessage> for TaskRunner {
    type Result = ResponseFuture<()>;

    fn handle(&mut self, msg: TaskExecuteMessage, _ctx: &mut Self::Context) -> Self::Result {
        let (name, event_name, event, action) = msg.to_parts();
        debug!("Starting task {}", name);
        trace!("Triggered by {}", event_name);
        Box::pin(async move {
            action.run(GithubEventMessage::new(name.as_str(), event)).await;
            debug!("Completed execution of task {}", name);
        })
    }
}