use serde::{Deserialize, Serialize};

use crate::models::{CommonEventFields, Label};

//------------------------------------        Label Event       --------------------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LabelEvent {
    pub action: LabelEventAction,
    /// The label that was added.
    pub label: Label,
    #[serde(flatten)]
    pub info: CommonEventFields,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LabelEventAction {
    /// label created event
    #[serde(rename = "created")]
    Created,
    /// label deleted event
    #[serde(rename = "deleted")]
    Deleted,
    /// label edited event
    #[serde(rename = "edited")]
    Edited(Box<LabelEditedChanges>),
}

impl ToString for LabelEventAction {
    fn to_string(&self) -> String {
        match self {
            Self::Created => "created".into(),
            Self::Deleted => "deleted".into(),
            Self::Edited(_) => "edited".into(),
        }
    }
}

/// The changes to the label if the action was `edited`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LabelEditedChanges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<LabelEditedChangesColor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<LabelEditedChangesName>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LabelEditedChangesColor {
    /// The previous version of the color if the action was `edited`.
    pub from: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LabelEditedChangesName {
    /// The previous version of the name if the action was `edited`.
    pub from: String,
}
