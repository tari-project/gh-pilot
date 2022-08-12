pub mod comments {
    pub const COMMENT: &str = include_str!("./data/comment.json");
}

pub mod labels {
    pub const LABELS: &str = include_str!("./data/labels.json");
}

pub mod pull_requests {
    pub const TARI_PR_1K: &str = include_str!("./data/pr_1000.json");
}

pub mod repositories {
    pub const TARI_REPO: &str = include_str!("./data/tari_repo.json");
}

pub mod users {
    pub const CJS77: &str = include_str!("./data/users_cjs77.json");
    pub const STRINGHANDLER: &str = include_str!("./data/users_stringhandler.json");
}

pub mod issues {
    pub const ISSUE: &str = include_str!("./data/issue.json");
}

pub mod events {
    pub const PR_REVIEW_COMMENT: &str = include_str!("./data/pr_review_comment_event.json");
    pub const PUSH_EVENT: &str = include_str!("./data/push_event.json");
}
