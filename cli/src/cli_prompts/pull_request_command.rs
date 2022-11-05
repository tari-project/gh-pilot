use crate::{cli_def::PullRequestCommand};

impl PullRequestCommand {
    pub async fn prompt(&mut self) {
        match self {
            PullRequestCommand::AddLabel(args) => args.prompt().await,
            PullRequestCommand::RemoveLabel(args) => args.prompt().await,
            PullRequestCommand::Merge(args) => args.prompt().await,
            PullRequestCommand::Fetch => {},
            PullRequestCommand::Comments => {},
            PullRequestCommand::Reviews => {},
            PullRequestCommand::Check => {},
        }
    }
}
