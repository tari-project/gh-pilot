use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewLabel {
    pub name: String,
    pub color: Option<String>,
    pub description: Option<String>,
}

impl NewLabel {
    pub fn new<S1: Into<String>>(name: S1, color: Option<String>, description: Option<String>) -> Self {
        Self {
            name: name.into(),
            color,
            description,
        }
    }
}

impl<S: AsRef<str>> From<S> for NewLabel {
    fn from(s: S) -> Self {
        Self {
            name: s.as_ref().to_string(),
            color: None,
            description: None,
        }
    }
}
