pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
    InvalidSubqueryType,
    Surreal,
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self { kind }
    }
}

impl From<surrealdb::err::Error> for Error {
    fn from(_: surrealdb::err::Error) -> Self {
        Self {
            kind: ErrorKind::Surreal,
        }
    }
}
