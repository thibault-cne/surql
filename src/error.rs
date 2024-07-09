pub struct Error {
    pub kind: ErrorKind,
}

pub enum ErrorKind {
    InvalidSubqueryType,
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self { kind }
    }
}
