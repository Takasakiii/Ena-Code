use std::fmt::Display;

pub type GenericError = Box<dyn std::error::Error + Send + Sync>;
pub type EResult<T> = Result<T, EnaError>;

#[derive(Debug)]
pub enum EnaError {
    UnknowError(GenericError),
    HomeDirNotExists,
}

impl Display for EnaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<GenericError> for EnaError {
    fn from(g_error: GenericError) -> Self {
        EnaError::UnknowError(g_error)
    }
}

impl From<EnaError> for GenericError {
    fn from(e_error: EnaError) -> Self {
        e_error.to_string().into()
    }
}
