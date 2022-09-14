pub struct IssueId {
    pub owner: String,
    pub repo: String,
    pub number: u64,
}

impl IssueId {
    pub fn new<S: Into<String>>(owner: S, repo: S, number: u64) -> Self {
        Self {
            owner: owner.into(),
            repo: repo.into(),
            number,
        }
    }
}
