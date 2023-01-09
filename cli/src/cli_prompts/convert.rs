use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use chrono::{Datelike, TimeZone, Utc};
use github_pilot_api::{
    models::{DateTime, Label},
    provider_traits::{IssueProvider, RepoProvider},
    wrappers::{GithubHandle, IssueId, NewLabel, RepoId},
    GithubProvider,
};
use prompts::{autocomplete::AutocompletePrompt, text::TextPrompt, Prompt};

use crate::{
    cli_def::{ActivityReportOptions, Cli, Commands, IssueCommand, LabelArg, LabelCommand, PullRequestCommand},
    cli_prompts::user_command::extract_github_handle,
    pilot_command::{assign_labels, IssueCmd, LabelCmd, PilotCommand, PilotCommand::NoOp, PrCmd},
};

impl Cli {
    pub async fn as_pilot_command(&mut self, provider: &GithubProvider) -> Result<PilotCommand, String> {
        let command = match self.command.clone() {
            Commands::User { profile } => extract_github_handle(self.non_interactive, profile.as_ref()).await?,
            Commands::PullRequest { number, sub_command } => self.to_pr_cmd(provider, number, &sub_command).await?,
            Commands::Issue { number, sub_command } => self.to_issue_cmd(provider, number, &sub_command).await?,
            Commands::Labels {
                sub_command: LabelCommand::Assign { labels_file },
            } => {
                let id = self.prompt_repo_id().await?;
                assign_labels(provider, &id, labels_file.as_str()).await?;
                NoOp
            },
            Commands::Labels { sub_command } => self.to_label_cmd(provider, sub_command).await?,
            Commands::Contributors => self.to_contributors(provider).await?,
            Commands::ActivityReport(opts) => Self::to_activity_cmd(opts)?,
        };
        Ok(command)
    }

    async fn to_pr_cmd(
        &self,
        provider: &GithubProvider,
        number: Option<u64>,
        sub_command: &PullRequestCommand,
    ) -> Result<PilotCommand, String> {
        let id = self.prompt_issue_id(number, "PR").await?;
        let repo = RepoId::new(id.owner(), id.repo());
        let cmd = match sub_command {
            PullRequestCommand::Fetch => PrCmd::Fetch(id),
            PullRequestCommand::AddLabel(label) => {
                let label = self.get_or_prompt_for_label(provider, &repo, label).await?;
                PrCmd::AddLabel(id, label)
            },
            PullRequestCommand::RemoveLabel(label) => {
                let label = self.get_or_prompt_for_existing_label(provider, &id, label).await?;
                PrCmd::RemoveLabel(id, label)
            },
            PullRequestCommand::Comments => PrCmd::Comments(id),
            PullRequestCommand::Merge(m) => PrCmd::Merge(id, m.into()),
            PullRequestCommand::Reviews => PrCmd::Reviews(id),
            PullRequestCommand::Check => PrCmd::Check(id),
            PullRequestCommand::AddComment(arg) => {
                let comment = self.get_or_prompt_for_comment(&arg.comment).await?;
                PrCmd::AddComment(id, comment)
            },
        };
        Ok(PilotCommand::PullRequest(cmd))
    }

    async fn prompt_repo_id(&self) -> Result<RepoId, String> {
        let non_interactive = self.non_interactive;
        let owner = if self.owner.is_none() {
            if non_interactive {
                return Err("😥 Organisation/Owner is required".to_string());
            }
            match TextPrompt::new("🏢 Owner / Organisation: ").run().await {
                Ok(Some(owner)) => owner,
                Ok(None) => return Err("😥 Organisation/Owner can't be blank for this command".to_string()),
                Err(e) => return Err(format!("😥 Failed to get owner: {}", e)),
            }
        } else {
            self.owner.clone().unwrap()
        };
        let repo = if self.repo.is_none() {
            if non_interactive {
                return Err("😥 Repository is required".to_string());
            }
            match TextPrompt::new("📦 Repository: ").run().await {
                Ok(Some(repo)) => repo,
                Ok(None) => return Err("😥 Repository can't be blank for this command".to_string()),
                Err(e) => return Err(format!("😥 Failed to get repository: {}", e)),
            }
        } else {
            self.repo.clone().unwrap()
        };
        Ok(RepoId::new(owner, repo))
    }

    async fn prompt_issue_id(&self, number: Option<u64>, id_type: &str) -> Result<IssueId, String> {
        let non_interactive = self.non_interactive;
        let repo_id = self.prompt_repo_id().await?;
        let number = if number.is_none() {
            if non_interactive {
                return Err(format!("😥 {} number is required", id_type));
            }
            match TextPrompt::new(format!("{id_type} number: "))
                .with_validator(must_be_u64)
                .run()
                .await
            {
                Ok(Some(number)) => number
                    .parse::<u64>()
                    .expect("Input should have been validated. Expected an integer"),
                Ok(None) => return Err(format!("😥 {id_type} number can't be blank for this command")),
                Err(e) => return Err(format!("😥 Failed to get {id_type} number: {e}")),
            }
        } else {
            number.unwrap()
        };
        Ok(IssueId::new(repo_id.owner(), repo_id.repo(), number))
    }

    async fn to_issue_cmd(
        &self,
        provider: &GithubProvider,
        number: Option<u64>,
        sub_command: &IssueCommand,
    ) -> Result<PilotCommand, String> {
        let id = self.prompt_issue_id(number, "issue").await?;
        let repo = RepoId::new(id.owner(), id.repo());
        let cmd = match sub_command {
            IssueCommand::Fetch => IssueCmd::Fetch(id),
            IssueCommand::AddLabel(l) => {
                let label = self.get_or_prompt_for_label(provider, &repo, l).await?;
                IssueCmd::AddLabel(id, label)
            },
            IssueCommand::RemoveLabel(l) => {
                let label = self.get_or_prompt_for_existing_label(provider, &id, l).await?;
                IssueCmd::RemoveLabel(id, label)
            },
            IssueCommand::Comments => IssueCmd::Comments(id),
            IssueCommand::AddComment(arg) => {
                let comment = self.get_or_prompt_for_comment(&arg.comment).await?;
                IssueCmd::AddComment(id, comment)
            },
        };
        Ok(PilotCommand::Issue(cmd))
    }

    async fn get_or_prompt_for_label(
        &self,
        provider: &GithubProvider,
        id: &RepoId,
        label: &LabelArg,
    ) -> Result<String, String> {
        let label = match (self.non_interactive, &label.label) {
            (true, None) => return Err("😥 Label is required for this command".to_string()),
            (_, Some(l)) => l.clone(),
            (false, None) => {
                let labels = fetch_labels(provider, id.owner(), id.repo()).await?;
                let labels = labels.into_iter().map(|l| l.name).collect();
                let mut prompt = AutocompletePrompt::new("🏷 Label: ", labels);
                prompt
                    .run()
                    .await
                    .map_err(|e| format!("Failed to get label: {}", e))?
                    .ok_or_else(|| "😥 Label can't be blank for this command".to_string())?
            },
        };
        Ok(label)
    }

    /// Returns a *required* string by extracting it from the given value, failing that, prompting the user, failing
    /// that, returning an error.
    async fn get_or_prompt_for_string(
        &self,
        val: &Option<String>,
        prompt: &str,
        err_msg: &str,
    ) -> Result<String, String> {
        match (self.non_interactive, val) {
            (true, None) => Err(err_msg.to_string()),
            (_, Some(s)) => Ok(s.clone()),
            (false, None) => {
                let mut prompt = TextPrompt::new(prompt);
                let val = prompt
                    .run()
                    .await
                    .map_err(|e| format!("Failed to get label: {}", e))?
                    .ok_or_else(|| err_msg.to_string())?;
                Ok(val)
            },
        }
    }

    async fn get_or_prompt_for_comment(&self, comment: &Option<String>) -> Result<String, String> {
        self.get_or_prompt_for_string(
            comment,
            "📝 Provide the comment body:",
            "😥 A comment body is required for this command",
        )
        .await
    }

    async fn get_or_prompt_for_existing_label(
        &self,
        provider: &GithubProvider,
        id: &IssueId,
        label: &LabelArg,
    ) -> Result<String, String> {
        let label = match (self.non_interactive, &label.label) {
            (true, None) => return Err("😥 Label is required for this command".to_string()),
            (_, Some(l)) => l.clone(),
            (false, None) => {
                let labels = fetch_issue_labels(provider, id).await?;
                let labels = labels.into_iter().map(|l| l.name).collect();
                let mut prompt = AutocompletePrompt::new("🏷 Label: ", labels);
                prompt
                    .run()
                    .await
                    .map_err(|e| format!("Failed to get label: {}", e))?
                    .ok_or_else(|| "😥 Label can't be blank for this command".to_string())?
            },
        };
        Ok(label)
    }

    async fn to_label_cmd(&self, provider: &GithubProvider, sub_command: LabelCommand) -> Result<PilotCommand, String> {
        let id = self.prompt_repo_id().await?;
        let can_prompt = !self.non_interactive;
        let cmd = match sub_command {
            LabelCommand::List {
                mut page,
                mut per_page,
                format,
            } => {
                if can_prompt {
                    LabelCommand::prompt_pagination(&mut page, &mut per_page).await?;
                }
                LabelCmd::List {
                    repo: id,
                    format,
                    page,
                    per_page,
                }
            },
            LabelCommand::Create {
                mut name,
                mut description,
                mut color,
            } => {
                if can_prompt {
                    LabelCommand::prompt_new_label(&mut name, &mut color, &mut description).await?;
                }
                if name.is_none() {
                    return Err("🏷 A label name is required.".to_string());
                }
                let new_label = NewLabel::new(name.unwrap(), color, description);
                LabelCmd::Create(id, new_label)
            },
            LabelCommand::Delete(label) => {
                let label = self.get_or_prompt_for_label(provider, &id, &label).await?;
                LabelCmd::Delete(id, label)
            },
            LabelCommand::Assign { .. } => return Err("Labels::Assign should have been handled".to_string()),
            LabelCommand::Edit {
                label,
                name,
                description,
                color,
            } => {
                let arg = LabelArg { label };
                let label = self.get_or_prompt_for_label(provider, &id, &arg).await?;
                let edits = NewLabel::new(name.unwrap_or_else(|| label.clone()), color, description);
                LabelCmd::Edit {
                    repo: id,
                    name: label,
                    edits,
                }
            },
        };
        Ok(PilotCommand::Labels(cmd))
    }

    async fn to_contributors(&self, _provider: &dyn RepoProvider) -> Result<PilotCommand, String> {
        let id = self.prompt_repo_id().await?;
        Ok(PilotCommand::Contributors(id))
    }

    fn to_activity_cmd(opts: ActivityReportOptions) -> Result<PilotCommand, String> {
        let since = match opts.since {
            None => {
                let today = Utc::now();
                let start_of_month = Utc
                    .with_ymd_and_hms(today.year(), today.month(), 1, 0, 0, 0)
                    .earliest()
                    .unwrap_or(today);
                DateTime::new(start_of_month)
            },
            Some(since) => {
                let d = format!("\"{since}T0:0:0Z\"");
                serde_json::from_str(d.as_str()).map_err(|e| format!("Invalid value for 'since': {since}. {e}"))?
            },
        };
        match (opts.id, opts.user_file_path) {
            (None, None) => Err("Either ids or userfile must be specified".into()),
            (Some(_), Some(_)) => Err("You cannot specify both ids and userfile".into()),
            (Some(ids), None) => Ok(PilotCommand::ActivityReport(
                ids.into_iter().map(GithubHandle::from).collect(),
                since,
            )),
            (None, Some(f)) => {
                let ids = Self::load_ids(f)?;
                Ok(PilotCommand::ActivityReport(ids, since))
            },
        }
    }

    fn load_ids(id_file: String) -> Result<Vec<GithubHandle>, String> {
        let f = File::open(id_file).map_err(|e| format!("Invalid user id file. {e}"))?;
        let ids = BufReader::new(f)
            .lines()
            .filter_map(|l| {
                l.ok().and_then(|s| {
                    if s.starts_with('#') {
                        None
                    } else {
                        Some(GithubHandle::from(s))
                    }
                })
            })
            .collect();
        Ok(ids)
    }
}

pub fn must_be_u64(val: &str) -> Result<(), String> {
    match val.parse::<u64>() {
        Ok(_) => Ok(()),
        Err(_) => Err("Value is not an integer >= 0".to_string()),
    }
}

async fn fetch_labels(provider: &GithubProvider, owner: &str, repo: &str) -> Result<Vec<Label>, String> {
    let labels = provider
        .fetch_labels(owner, repo, None, None)
        .await
        .map_err(|e| format!("Failed to fetch labels: {}", e))?;
    Ok(labels)
}

async fn fetch_issue_labels(provider: &GithubProvider, id: &IssueId) -> Result<Vec<Label>, String> {
    let labels = provider
        .fetch_issue_labels(id)
        .await
        .map_err(|e| format!("Failed to fetch labels for {id}: {e}"))?;
    Ok(labels)
}
