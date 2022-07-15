use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExamReaderError {
    #[error("Reading error")]
    IOError(#[from] std::io::Error),
    #[error("Your input file is badly formatted: `{0}`")]
    TemplateError(String),
    #[error("Your input file is badly `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown error")]
    Unknown,
}
