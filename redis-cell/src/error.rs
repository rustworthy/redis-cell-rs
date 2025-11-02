#[derive(Clone, Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("redis cell reponse decoding failed: {0}")]
    Protocol(String),
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::Protocol(value)
    }
}
