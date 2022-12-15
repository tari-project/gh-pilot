#[derive(Clone, Default)]
pub struct Page {
    since: Option<String>,
    per_page: Option<usize>,
    page: Option<usize>,
}

impl Page {
    pub fn nth(page: usize, per_page: usize) -> Self {
        Self {
            since: None,
            per_page: Some(per_page),
            page: Some(page),
        }
    }

    pub fn length(per_page: usize) -> Self {
        Self {
            since: None,
            per_page: Some(per_page),
            page: None,
        }
    }

    pub fn since<S: Into<String>>(since: S) -> Self {
        Self {
            since: Some(since.into()),
            per_page: None,
            page: None,
        }
    }

    pub fn to_query(self) -> String {
        let mut qs = Vec::with_capacity(3);
        if let Some(since) = self.since {
            qs.push(format!("since={since}"));
        }
        if let Some(page) = self.page {
            qs.push(format!("page={page}"));
        }
        if let Some(pp) = self.per_page {
            qs.push(format!("per_page={pp}"));
        }
        qs.join("&")
    }
}
