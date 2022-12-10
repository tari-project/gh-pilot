use actix::Addr;
use log::*;
use notify::{
    event::{DataChange, ModifyKind},
    Config,
    Event,
    EventHandler,
    EventKind,
    RecommendedWatcher,
    Watcher,
};
use tokio::sync::mpsc::{channel, Sender};

use crate::{load_rules::load_rules, pub_sub::PubSubActor};

pub struct FileChangeHandler(Sender<Event>);

impl EventHandler for FileChangeHandler {
    fn handle_event(&mut self, res: notify::Result<Event>) {
        match res {
            Ok(event) => {
                trace!("Sending watcher event: {event:?}");
                if let Err(e) = self.0.blocking_send(event) {
                    warn!("Error sending event. {e}")
                }
            },
            Err(e) => warn!("Watcher error. {e}"),
        }
    }
}

pub fn async_watch(pubsub: Addr<PubSubActor>, path: &str) -> Option<RecommendedWatcher> {
    let (tx, mut rx) = channel(1);
    let watcher = RecommendedWatcher::new(FileChangeHandler(tx), Config::default())
        .map_err(|e| {
            warn!(
                "Could not create notifier for {path}. If you change rules, the server will have to be restarted in \
                 order for them to take effect. {e}"
            );
        })
        .ok();
    let path_copy = path.to_string();
    tokio::spawn(async move {
        debug!("Waiting for file change events");
        while let Some(event) = rx.recv().await {
            trace!("Received Rules file change event: {:?}", event.kind);
            if matches!(event.kind, EventKind::Modify(ModifyKind::Data(DataChange::Content))) {
                info!("Ruleset change detected. Resetting rules...");
                match load_rules(pubsub.clone(), path_copy.as_str()).await {
                    Ok(num_rules) => info!("📄 {num_rules} Rules loaded"),
                    Err(e) => error!(
                        "Could not update rules. GithubPilot Server will be running without rules until this is \
                         resolved :(. {e}"
                    ),
                };
            }
        }
    });
    info!("Watching, {path} for changes. Rules will be reconfigured automatically");
    watcher
}
