use std::{fmt::Display, str::FromStr};

use thiserror::Error;

use crate::wrappers::IssueId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepoId {
    pub owner: String,
    pub repo: String,
}

impl RepoId {
    pub fn new<S: Into<String>, S2: Into<String>>(owner: S, repo: S2) -> Self {
        Self {
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    pub fn owner(&self) -> &str {
        self.owner.as_str()
    }

    pub fn repo(&self) -> &str {
        self.repo.as_str()
    }
}

impl Display for RepoId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.owner, self.repo)
    }
}

#[derive(Clone, Debug, Error)]
pub enum RepoIdParseError {
    #[error("Could not extract repository name from the string.")]
    MissingRepoSeparator,
    #[error("The Repo or pr string is in the wrong format. {0}")]
    FormatError(String),
}

impl FromStr for RepoId {
    type Err = RepoIdParseError;

    /// Parses a string of the format `{owner}/{repo}` into a `RepoId`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('/');
        let owner = split
            .next()
            .ok_or_else(|| RepoIdParseError::FormatError("Repo id was not formatted as {owner}/{repo}".into()))?;
        let repo = split.next().ok_or(RepoIdParseError::MissingRepoSeparator)?;
        if owner.is_empty() {
            return Err(RepoIdParseError::FormatError("Owner cannot be empty".to_string()));
        }
        if repo.is_empty() {
            return Err(RepoIdParseError::FormatError("Repo cannot be empty".to_string()));
        }
        Ok(Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
        })
    }
}

impl From<IssueId> for RepoId {
    fn from(issue_id: IssueId) -> Self {
        Self {
            owner: issue_id.owner,
            repo: issue_id.repo,
        }
    }
}

#[cfg(test)]
mod test {
    

    use super::*;

    #[test]
    fn parse_id() {
        let id = "owner/repo".parse::<RepoId>().unwrap();
        assert_eq!(id.owner, "owner");
        assert_eq!(id.repo, "repo");
    }

    #[test]
    fn missing_repo() {
        let id = "owner".parse::<RepoId>();
        assert!(
            matches!(id, Err(RepoIdParseError::MissingRepoSeparator)),
            "Got {id:?}",
        );
        let id = "owner/".parse::<RepoId>();
        assert!(
            matches!(id, Err(RepoIdParseError::FormatError(ref s))if s == "Repo cannot be empty"),
            "Got {id:?}",
        );
    }
    #[test]
    fn missing_owner() {
        let id = "/".parse::<RepoId>();
        assert!(
            matches!(id, Err(RepoIdParseError::FormatError(ref s))if s == "Owner cannot be empty"),
            "Got {id:?}",
        );
        let id = "/repo".parse::<RepoId>();
        assert!(
            matches!(id, Err(RepoIdParseError::FormatError(ref s))if s == "Owner cannot be empty"),
            "Got {id:?}",
        );
    }
}
