use github_pilot_api::models_plus::{MergeMethod, MergeParameters};
use prompts::{select::SelectPrompt, text::TextPrompt, Prompt};

use crate::cli_def::MergeArgs;

impl MergeArgs {
    pub async fn prompt(&mut self) {
        if self.commit_title.is_none() {
            let mut prompt = TextPrompt::new("🔀 Pull request title [Can be blank]: ");
            match prompt.run().await {
                Ok(Some(t)) => {
                    self.commit_title.replace(t);
                },
                Ok(None) => {},
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        if self.commit_message.is_none() {
            let mut prompt = TextPrompt::new("🔀 Pull request body [Can be blank]: ");
            match prompt.run().await {
                Ok(Some(t)) => {
                    self.commit_message.replace(t);
                },
                Ok(None) => {},
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        if self.merge_method.is_none() {
            let mut prompt = SelectPrompt::new("🔀 Merge method [Enter for default]: ", vec![
                "merge", "rebase", "squash",
            ]);
            match prompt.run().await {
                Ok(Some(t)) => {
                    self.merge_method.replace(
                        t.parse::<MergeMethod>()
                            .expect("Merge method should have been restricted"),
                    );
                },
                Ok(None) => {},
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }
}

impl From<&MergeArgs> for MergeParameters {
    fn from(a: &MergeArgs) -> Self {
        MergeParameters {
            commit_title: a.commit_title.clone(),
            commit_message: a.commit_message.clone(),
            sha: a.sha.clone(),
            merge_method: a.merge_method.clone().unwrap_or_default(),
        }
    }
}
