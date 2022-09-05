use std::sync::Arc;
use log::*;
use crate::actions::Action;
use crate::pub_sub::GithubEventMessage;
use async_trait::async_trait;

type ActionFn = Arc<dyn Fn(GithubEventMessage) + Send + Sync>;

/// An action implementation that wraps a closure
pub struct ClosureAction {
    function: ActionFn
}

impl ClosureAction {
    pub fn with<F: Fn(GithubEventMessage) + Send + Sync + 'static>(f: F) -> Self {
        Self { function: Arc::new(f) }
    }
}

#[async_trait]
impl Action for ClosureAction {
    async fn run(&self, msg: GithubEventMessage) {
        debug!("Running closure action");
        let f = Arc::clone(&self.function);
        let result = tokio::task::spawn_blocking(move || {
            f(msg);
        }).await;
        match result {
            Ok(()) => debug!("Closure Task completely happily."),
            Err(e) => debug!("Closure task wasn't happy. {}", e.to_string())
        }
    }
}