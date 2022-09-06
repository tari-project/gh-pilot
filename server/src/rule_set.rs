//! The rule_set module is a wrapper around a Vec of [`Rules`] with added functionality to (eventually) read in rules
//! from JSON, YAML, etc. so that the server can be configured without needing to recompile it every time.
//!
//! We maintain a vector of rules because order is important. Rules are run in the order that they are defined.

use crate::rules::Rule;

#[derive(Default)]
pub struct RuleSet {
    rules: Vec<Rule>,
}

impl RuleSet {
    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
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
