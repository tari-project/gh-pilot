use serde::Deserialize;

use crate::models::common::Url;

#[derive(Debug, Deserialize)]
pub struct Label {
    pub id: u64,
    pub node_id: String,
    pub url: Url,
    pub name: String,
    pub description: Option<String>,
    pub color: String,
    pub default: bool,
}

#[cfg(test)]
mod test {
    use crate::models::{labels::Label, static_data::labels::LABELS};

    #[test]
    fn deserialize_labels() {
        let labels: Vec<Label> = serde_json::from_str(LABELS).unwrap();
        assert_eq!(labels.len(), 5);
        assert_eq!(labels[0].id, 1189345686);
        assert_eq!(labels[4].color, "bfdadc");
    }
}
