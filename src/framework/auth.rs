use std::fmt;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Credentials {
    pub(crate) key: String,
}

impl Credentials {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl fmt::Display for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Api Key: *****nouwhy*****")
    }
}
