use std::{fs::File, io::BufReader};

use github_pilot_api::{models::Label, provider_traits::RepoProvider, wrappers::NewLabel};
use log::{debug, error, info};

use crate::{
    cli_def::LabelCommand,
    pretty_print::{add_labels, pretty_table},
};

pub async fn run_label_cmd(provider: &dyn RepoProvider, owner: &str, repo: &str, cmd: LabelCommand) -> Result<(), ()> {
    match cmd {
        LabelCommand::List { page, per_page } => fetch_labels(provider, owner, repo, page, per_page).await,
        LabelCommand::Delete { label } => delete_label(provider, owner, repo, &label).await,
        LabelCommand::Create {
            name,
            color,
            description,
        } => {
            let label = NewLabel::new(name, color, description);
            create_label(provider, owner, repo, label).await
        },
        LabelCommand::Assign { labels_file } => assign_labels(provider, owner, repo, &labels_file).await,
        LabelCommand::Edit {
            label,
            name,
            color,
            description,
        } => {
            let new_name = name.unwrap_or(label.clone());
            let new_label = NewLabel::new(new_name, color, description);
            edit_label(provider, owner, repo, label, new_label).await
        },
    }
}

async fn assign_labels(provider: &dyn RepoProvider, owner: &str, repo: &str, labels: &String) -> Result<(), ()> {
    let labels = load_labels(labels).map_err(|_| {
        error!("🏷 Could not load labels");
    })?;
    debug!("🏷 Assigning {} labels to {owner}/{repo}", labels.len());
    match provider.assign_labels(owner, repo, &labels[..]).await {
        Ok(_) => {
            info!("🏷 {} labels assigned to {owner}/{repo}", labels.len());
            Ok(())
        },
        Err(e) => {
            error!("🏷 Error assigning labels to {owner}/{repo}: {e}");
            Err(())
        },
    }
}

fn load_labels(labels: &String) -> Result<Vec<NewLabel>, ()> {
    let file = File::open(labels).map_err(|e| {
        error!("🏷 Could not open file '{labels}': {e}");
    })?;
    let reader = BufReader::new(file);
    let labels: Vec<NewLabel> = serde_yaml::from_reader(reader).map_err(|e| {
        error!("🏷 Could not parse labels file '{labels}': {e}");
    })?;
    Ok(labels)
}

async fn edit_label(
    provider: &dyn RepoProvider,
    owner: &str,
    repo: &str,
    label: String,
    new: NewLabel,
) -> Result<(), ()> {
    match provider.edit_label(owner, repo, label.as_str(), &new).await {
        Ok(edited) => {
            if edited {
                info!("🏷 Label '{label}' edited");
            } else {
                info!("🏷 Label '{label}' not found");
            }
            Ok(())
        },
        Err(e) => {
            error!("🏷 Error editing label '{label}': {e}");
            Err(())
        },
    }
}

async fn delete_label(provider: &dyn RepoProvider, owner: &str, repo: &str, label: &String) -> Result<(), ()> {
    match provider.delete_label(owner, repo, label).await {
        Ok(true) => {
            info!("🏷 Label {label} deleted");
            Ok(())
        },
        Ok(false) => {
            info!("🏷 Label {label} not found, so was not deleted");
            Ok(())
        },
        Err(e) => {
            error!("🏷 Error deleting label: {e}");
            Err(())
        },
    }
}

async fn fetch_labels(
    provider: &dyn RepoProvider,
    owner: &str,
    repo: &str,
    page: Option<usize>,
    per_page: Option<usize>,
) -> Result<(), ()> {
    match provider.fetch_labels(owner, repo, page, per_page).await {
        Ok(labels) => {
            pretty_print(&labels);
            Ok(())
        },
        Err(e) => {
            error!("🏷 Error fetching labels: {e}");
            Err(())
        },
    }
}

async fn create_label(provider: &dyn RepoProvider, owner: &str, repo: &str, label: NewLabel) -> Result<(), ()> {
    let name = label.name.clone();
    match provider.assign_labels(owner, repo, &[label]).await {
        Ok(()) => {
            info!("🏷 Label {name} created");
            Ok(())
        },
        Err(e) => {
            error!("🏷 Error creating label '{name}': {e}");
            Err(())
        },
    }
}

fn pretty_print(labels: &[Label]) {
    let mut table = pretty_table("Label", "Description");
    add_labels(&mut table, labels);
    println!("{table}");
}
