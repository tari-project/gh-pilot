//! The rule_set module is a wrapper around a Vec of [`Rules`] with added functionality to (eventually) read in rules
//! from JSON, YAML, etc. so that the server can be configured without needing to recompile it every time.
//!
//! We maintain a vector of rules because order is important. Rules are run in the order that they are defined.

use std::{io::ErrorKind, path::Path};

use serde::{Deserialize, Serialize};

use crate::rules::Rule;

#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct RuleSet {
    rules: Vec<Rule>,
}

impl RuleSet {
    #[cfg(feature = "json")]
    pub fn from_json<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let json_file = std::fs::read_to_string(path)?;
        let rs = serde_json::from_str(json_file.as_str())?;
        Ok(rs)
    }

    #[cfg(feature = "yaml")]
    pub fn from_yaml<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let yaml_file = std::fs::read_to_string(path)?;
        let rs =
            serde_yaml::from_str(yaml_file.as_str()).map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e))?;
        Ok(rs)
    }

    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    pub fn to_rules(self) -> Vec<Rule> {
        self.rules
    }
}

impl From<RuleSet> for Vec<Rule> {
    fn from(rs: RuleSet) -> Self {
        rs.rules
    }
}

impl From<Vec<Rule>> for RuleSet {
    fn from(rules: Vec<Rule>) -> Self {
        Self { rules }
    }
}
