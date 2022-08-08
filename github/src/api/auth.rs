use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct AuthToken(String);

impl Display for AuthToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl<S: AsRef<str>> From<S> for AuthToken {
    fn from(src: S) -> Self {
        AuthToken(src.as_ref().into())
    }
}
