use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExamReaderError {
    #[error("Redering error")]
    IOError(#[from] std::io::Error),
    #[error("Error parsing `{0}`")]
    TemplateError(String),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}
