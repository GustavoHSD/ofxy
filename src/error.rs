#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("parse error: {0}")]
    ParseError(String),

    #[error("at {location}: {msg}")]
    Located {
        location: Location,
        msg: String,
    },

    #[error(transparent)]
    Sgmlish(#[from] sgmlish::Error),

    #[error(transparent)]
    Normalization(#[from] sgmlish::transforms::NormalizationError),

    #[error(transparent)]
    SgmlishDe(#[from] sgmlish::de::DeserializationError),

    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
}
