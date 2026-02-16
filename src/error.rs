use std::fmt;

#[derive(Debug, Clone)]
pub struct BooplError {
    pub message: String,
    pub line: i32,
    pub file: Option<String>,
}

pub type Result<T> = std::result::Result<T, BooplError>;

impl BooplError {
    pub fn new(msg: impl Into<String>, line: i32) -> Self {
        Self {
            message: msg.into(),
            line,
            file: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_file(mut self, file: impl Into<String>) -> Self {
        self.file = Some(file.into());
        self
    }
}

impl fmt::Display for BooplError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.file {
            Some(file) => write!(f, "\n   >>  ! {}  ({}:{})\n\n", self.message, file, self.line),
            None => write!(f, "\n   >>  ! {}  ({})\n\n", self.message, self.line),
        }
    }
}

impl std::error::Error for BooplError {}