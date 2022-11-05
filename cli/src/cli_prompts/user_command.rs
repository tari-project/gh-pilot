use github_pilot_api::wrappers::GithubHandle;
use prompts::{text::TextPrompt, Prompt};

use crate::pilot_command::PilotCommand;

pub async fn extract_github_handle(non_interactive: bool, profile: Option<&String>) -> Result<PilotCommand, String> {
    match (non_interactive, profile) {
        (_, Some(h)) => Ok(PilotCommand::User(GithubHandle::from(h))),
        (true, None) => Err("No profile name provided".to_string()),
        (false, None) => match TextPrompt::new("👨 Github handle:").run().await {
            Ok(Some(h)) => Ok(PilotCommand::User(GithubHandle::from(h))),
            Ok(None) => Err("A github handle is required".to_string()),
            Err(e) => Err(format!("Failed to get github handle: {e}")),
        },
    }
}
