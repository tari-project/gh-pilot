use std::{fmt::Display, num::ParseIntError, str::FromStr};

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IssueId {
    pub owner: String,
    pub repo: String,
    pub number: u64,
}

impl IssueId {
    pub fn new<S: Into<String>, S2: Into<String>>(owner: S, repo: S2, number: u64) -> Self {
        Self {
            owner: owner.into(),
            repo: repo.into(),
            number,
        }
    }

    pub fn owner(&self) -> &str {
        self.owner.as_str()
    }

    pub fn repo(&self) -> &str {
        self.repo.as_str()
    }

    pub fn number(&self) -> u64 {
        self.number
    }
}

impl Display for IssueId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}#{}", self.owner, self.repo, self.number)
    }
}

#[derive(Clone, Debug, Error)]
pub enum IssueIdParseError {
    #[error("The issue or pr number was missing. {0}")]
    MissingNumber(#[from] ParseIntError),
    #[error("Could not extract repository name from the string.")]
    MissingRepoSeparator,
    #[error("The issue or pr string is in the wrong format. {0}")]
    FormatError(String),
}

impl FromStr for IssueId {
    type Err = IssueIdParseError;

    /// Parses a string of the format `{owner}/{repo}#{number}` into an `IssueId`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('#');
        let repo = split
            .next()
            .ok_or_else(|| IssueIdParseError::FormatError("The string was empty".into()))?;
        let number = split.next().ok_or_else(|| {
            IssueIdParseError::FormatError("The `#{number}` portion of the string was missing or incomplete".into())
        })?;
        let number = number.parse::<u64>()?;
        let mut split = repo.split('/');
        let owner = split
            .next()
            .ok_or_else(|| IssueIdParseError::FormatError("`{owner}/{repo}` portion missing from string".into()))?;
        let repo = split.next().ok_or(IssueIdParseError::MissingRepoSeparator)?;
        Ok(Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
            number,
        })
    }
}

#[cfg(test)]
mod test {
    use std::num::IntErrorKind;

    use crate::wrappers::{issue_id::IssueIdParseError, IssueId};

    #[test]
    fn parse_id() {
        let id = "owner/repo#123".parse::<IssueId>().unwrap();
        assert_eq!(id.owner, "owner");
        assert_eq!(id.repo, "repo");
        assert_eq!(id.number, 123);
    }

    #[test]
    fn missing_number() {
        let id = "owner/repo#".parse::<IssueId>();
        assert!(
            matches!(id, Err(IssueIdParseError::MissingNumber(ref e)) if e.kind() == &IntErrorKind::Empty),
            "Expected a MissingNumber, got {:?}",
            id
        );
        let id = "owner/repo#x".parse::<IssueId>();
        assert!(
            matches!(id, Err(IssueIdParseError::MissingNumber(ref e)) if e.kind() == &IntErrorKind::InvalidDigit),
            "Expected a MissingNumber, got {:?}",
            id
        );
    }

    #[test]
    fn missing_repo() {
        let id = "#123".parse::<IssueId>();
        assert!(
            matches!(id, Err(IssueIdParseError::MissingRepoSeparator)),
            "Got {:?}",
            id
        );
        let id = "owner#123".parse::<IssueId>();
        assert!(
            matches!(id, Err(IssueIdParseError::MissingRepoSeparator)),
            "Got {:?}",
            id
        );
        let id = "".parse::<IssueId>();
        assert!(
            matches!(id, Err(IssueIdParseError::FormatError(ref s))
                if s == "The `#{number}` portion of the string was missing or incomplete"
            ),
            "Got {:?}",
            id
        );
    }
}
