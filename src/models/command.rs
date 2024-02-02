use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Command(pub String);

impl AsRef<str> for Command {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
