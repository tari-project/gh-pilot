use std::{fs::File, io::BufReader};

use comfy_table::{presets::UTF8_BORDERS_ONLY, Cell, ContentArrangement, Row, Table};
use github_pilot_api::{
    models::Label,
    provider_traits::RepoProvider,
    wrappers::{NewLabel, RepoId},
    GithubProvider,
};
use log::{debug, info};

use crate::{cli_def::OutputFormat, pretty_print::add_labels};

#[derive(Debug)]
pub enum LabelCmd {
    /// List labels
    List {
        repo: RepoId,
        format: OutputFormat,
        page: Option<usize>,
        per_page: Option<usize>,
    },
    /// Create a label
    Create(RepoId, NewLabel),
    /// Delete a label
    Delete(RepoId, String),
    /// Edit an existing label
    Edit {
        repo: RepoId,
        name: String,
        edits: NewLabel,
    },
}

impl LabelCmd {
    pub async fn execute(self, provider: &dyn RepoProvider) -> Result<(), String> {
        match self {
            LabelCmd::List {
                ref repo,
                format,
                page,
                per_page,
            } => fetch_labels(provider, repo, page, per_page, format).await,
            LabelCmd::Delete(repo, label) => delete_label(provider, &repo, label.as_str()).await,
            LabelCmd::Create(repo, label) => create_label(provider, &repo, label).await,
            LabelCmd::Edit {
                ref repo,
                ref name,
                ref edits,
            } => edit_label(provider, repo, name.as_str(), edits).await,
        }
    }
}

pub async fn assign_labels(provider: &GithubProvider, repo: &RepoId, label_file: &str) -> Result<(), String> {
    let labels = load_labels_from_file(label_file)?;
    debug!("🏷 Assigning {} labels to {repo}", labels.len());
    match provider.assign_labels(repo.owner(), repo.repo(), &labels[..]).await {
        Ok(_) => {
            info!("🏷 {} labels assigned to {repo}", labels.len());
            Ok(())
        },
        Err(e) => Err(format!("🏷 Error assigning labels to {repo}: {e}")),
    }
}

fn load_labels_from_file(labels: &str) -> Result<Vec<NewLabel>, String> {
    let file = File::open(labels).map_err(|e| format!("🏷 Could not open file '{labels}': {e}"))?;
    let reader = BufReader::new(file);
    let labels: Vec<NewLabel> =
        serde_yaml::from_reader(reader).map_err(|e| format!("🏷 Could not parse labels file '{labels}': {e}"))?;
    Ok(labels)
}

async fn edit_label(provider: &dyn RepoProvider, repo: &RepoId, label: &str, new: &NewLabel) -> Result<(), String> {
    match provider.edit_label(repo.owner(), repo.repo(), label, new).await {
        Ok(edited) => {
            if edited {
                info!("🏷 Label '{label}' edited");
            } else {
                info!("🏷 Label '{label}' not found");
            }
            Ok(())
        },
        Err(e) => Err(format!("🏷 Error editing label '{label}': {e}")),
    }
}

async fn delete_label(provider: &dyn RepoProvider, repo: &RepoId, label: &str) -> Result<(), String> {
    match provider.delete_label(repo.owner(), repo.repo(), label).await {
        Ok(true) => {
            info!("🏷 Label {label} deleted");
            Ok(())
        },
        Ok(false) => {
            info!("🏷 Label {label} not found, so was not deleted");
            Ok(())
        },
        Err(e) => Err(format!("🏷 Error deleting label: {e}")),
    }
}

async fn fetch_labels(
    provider: &dyn RepoProvider,
    id: &RepoId,
    page: Option<usize>,
    per_page: Option<usize>,
    format: OutputFormat,
) -> Result<(), String> {
    match provider.fetch_labels(id.owner(), id.repo(), page, per_page).await {
        Ok(labels) => {
            match format {
                OutputFormat::Text => pretty_print(&labels),
                OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&labels).unwrap()),
                OutputFormat::Yaml => println!("{}", serde_yaml::to_string(&labels).unwrap()),
            }
            Ok(())
        },
        Err(e) => Err(format!("🏷 Error fetching labels: {e}")),
    }
}

async fn create_label(provider: &dyn RepoProvider, repo: &RepoId, label: NewLabel) -> Result<(), String> {
    let name = label.name.clone();
    match provider.assign_labels(repo.owner(), repo.repo(), &[label]).await {
        Ok(()) => {
            info!("🏷 Label {name} created");
            Ok(())
        },
        Err(e) => Err(format!("🏷 Error creating label '{name}': {e}")),
    }
}

fn pretty_print(labels: &[Label]) {
    let mut table = Table::new();
    let mut header = Row::new();
    header
        .add_cell(Cell::new("Label"))
        .add_cell(Cell::new("Color"))
        .add_cell(Cell::new("Description"));
    table
        .load_preset(UTF8_BORDERS_ONLY)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .add_row(header);
    add_labels(&mut table, labels);
    println!("{table}");
}
