use regex::Regex;

use crate::{
    models::{IssueComment, IssueCommentAction, IssueCommentEvent},
    wrappers::IssueId,
};

pub const PR_REGEX: &str = r"github.com/[\w\-_]+/[\w\-_]+/pull/\d+";

impl IssueCommentEvent {
    /// Is this Issue comment actually a PR comment?
    ///
    /// TBG I don't know how robust the logic is here. It looks like the right way to do this would be if the
    /// `pull_request` field is not null, but Github is sometimes flaky with these things.
    ///
    /// The other approach is to look for the word "pull" in the html_url. It's dodgy af, but probably more reliable
    /// than the first option.
    ///
    /// Hell. why not both?
    pub fn is_on_pull_request(&self) -> bool {
        self.issue.is_pull_request() || self.comment.is_on_pull_request()
    }

    pub fn related_pull_request(&self) -> Option<IssueId> {
        if self.is_on_pull_request() {
            let owner = self.info.repository.owner.login.as_str();
            let repo = self.info.repository.name.as_str();
            let number = self.issue.number;
            Some(IssueId::new(owner, repo, number))
        } else {
            None
        }
    }
}

impl IssueComment {
    /// Is this comment made on a pull request?
    ///
    /// One would think that the `pull_request` field would say, but it's sometime null, so we need a backup
    pub fn is_on_pull_request(&self) -> bool {
        self.pull_request_url.is_some() || {
            // in an expression so we only eval the regex if we need to
            let re = Regex::new(PR_REGEX).unwrap();
            re.is_match(self.html_url.as_ref())
        }
    }
}

impl IssueCommentAction {}

#[cfg(test)]
mod test {
    use regex::Regex;

    use super::PR_REGEX;
    use crate::models::IssueCommentEvent;

    #[test]
    fn is_pull_request() {
        let data = include_str!("../test_data/issue_comment1.json");
        let comment: IssueCommentEvent = serde_json::from_str(data).unwrap();
        assert!(comment.is_on_pull_request());
        assert!(comment.issue.is_pull_request());
        assert!(comment.comment.is_on_pull_request());
    }

    #[test]
    fn related_pr_id() {
        let data = include_str!("../test_data/issue_comment1.json");
        let comment: IssueCommentEvent = serde_json::from_str(data).unwrap();
        let id = comment.related_pull_request().unwrap();
        assert_eq!(id.to_string(), "tari-project/gh-pilot#10");
    }

    #[test]
    fn pr_regex() {
        let re = Regex::new(PR_REGEX).unwrap();
        assert!(re.is_match("https://github.com/org/repo/pull/180"));
        assert!(re.is_match("https://github.com/org/repo/pull/180#foo"));
        assert!(re.is_match("https://github.com/tari-project/a_repo/pull/180#foo"));
        assert!(re.is_match("https://github.com/tari-2/repo/pull/180#foo"));
        assert!(re.is_match("http://github.com/tari/tari/pull/1"));

        assert!(!re.is_match("https://github.com/tari/repo/issues/180"));
        assert!(!re.is_match("https://gitland.com/tari/repo/issues/180"));
        assert!(!re.is_match("https://github.com/tari/repo/pulls"));
    }
}
