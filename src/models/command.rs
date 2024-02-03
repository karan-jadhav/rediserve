use serde::Deserialize;

use super::Argument;

#[derive(Debug, Deserialize)]
pub struct Command {
    pub name: String,
    pub args: Vec<Argument>,
}

impl AsRef<str> for Command {
    fn as_ref(&self) -> &str {
        &self.name
    }
}
