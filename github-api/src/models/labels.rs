use serde::{Deserialize, Serialize};

use crate::models::common::Url;

#[derive(Clone, Debug, Deserialize, Serialize)]
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
    use crate::models::labels::Label;

    #[test]
    fn deserialize_labels() {
        let data = include_str!("../test_data/labels.json");
        let labels: Vec<Label> = serde_json::from_str(data).unwrap();
        assert_eq!(labels.len(), 5);
        assert_eq!(labels[0].id, 1189345686);
        assert_eq!(labels[4].color, "bfdadc");
    }
}
