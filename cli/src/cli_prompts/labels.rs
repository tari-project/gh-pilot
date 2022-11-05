

use github_pilot_api::provider_traits::RepoProvider;
use prompts::{text::TextPrompt, Prompt};

use crate::{
    cli_def::{LabelArg, LabelCommand},
    cli_prompts::convert::must_be_u64,
};

impl LabelArg {
    pub async fn prompt(&mut self) {
        todo!()
    }
}

impl LabelCommand {
    pub async fn prompt(&mut self, _provider: &dyn RepoProvider) -> Result<(), String> {
        match self {
            LabelCommand::List { per_page, page, .. } => Self::prompt_pagination(page, per_page).await,
            LabelCommand::Create {
                name,
                color,
                description,
            } => Self::prompt_new_label(name, color, description).await,
            LabelCommand::Delete { .. } => Ok(()), // TODO
            LabelCommand::Assign { .. } => Ok(()), // TODO
            LabelCommand::Edit { .. } => Ok(()),   // TODO
        }
    }

    async fn prompt_pagination(page: &mut Option<usize>, per_page: &mut Option<usize>) -> Result<(), String> {
        if per_page.is_none() {
            let v = TextPrompt::new("How many labels to fetch [100]?")
                .with_validator(must_be_u64)
                .run()
                .await
                .map_err(|e| format!("Could not set 'per page' parameter. {e}"))?;
            if v.is_some() {
                let new_val = v.unwrap().parse::<usize>().expect("u64 validator failed");
                per_page.replace(new_val);
            }
        }
        if page.is_none() {
            let v = TextPrompt::new("Which page number to fetch [0]?")
                .with_validator(must_be_u64)
                .run()
                .await
                .map_err(|e| format!("Could not set 'page' parameter. {e}"))?;
            if v.is_some() {
                let new_val = v.unwrap().parse::<usize>().expect("u64 validator failed");
                page.replace(new_val);
            }
        }
        Ok(())
    }

    async fn prompt_new_label(
        name: &mut Option<String>,
        color: &mut Option<String>,
        description: &mut Option<String>,
    ) -> Result<(), String> {
        if name.is_none() {
            let v = TextPrompt::new("🏷 New label name:")
                .run()
                .await
                .map_err(|e| format!("Could not set label name. {e}"))?;
            if v.is_some() {
                name.replace(v.unwrap());
            }
        }
        if color.is_none() {
            let v = TextPrompt::new("🏷 New label color [#000000]:")
                .with_validator(color_validator)
                .run()
                .await
                .map_err(|e| format!("Could not set label color. {e}"))?;
            if v.is_some() {
                color.replace(v.unwrap());
            }
        }
        if description.is_none() {
            let v = TextPrompt::new("🏷 New label description:")
                .run()
                .await
                .map_err(|e| format!("Could not set label description. {e}"))?;
            if v.is_some() {
                description.replace(v.unwrap());
            }
        }
        Ok(())
    }
}

fn color_validator(input: &str) -> Result<(), String> {
    if input.chars().all(|c| c.is_ascii_hexdigit()) {
        Ok(())
    } else {
        Err("Color contained a non-hex character".to_string())
    }
}
