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
}
