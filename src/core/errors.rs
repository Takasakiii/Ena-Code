use std::fmt::Display;

pub type GenericError = Box<dyn std::error::Error + Send + Sync>;
pub type EResult<T> = Result<T, EnaError>;

#[derive(Debug)]
pub enum EnaError {
    UnknowError(GenericError),
    #[cfg(feature = "config_yml")]
    SerializeDeserializeConfigError(serde_yaml::Error),
    #[cfg(feature = "config_json")]
    SerializeDeserializeConfigError(serde_json::Error),
    HomeDirNotExists,
    VsCodeNotFound,
    PathToStrNone,
    IoError(std::io::Error),
    NotImplemented,
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

#[cfg(feature = "config_yml")]
impl From<serde_yaml::Error> for EnaError {
    fn from(err: serde_yaml::Error) -> Self {
        Self::SerializeDeserializeConfigError(err)
    }
}

#[cfg(feature = "config_json")]
impl From<serde_json::Error> for EnaError {
    fn from(err: serde_json::Error) -> Self {
        Self::SerializeDeserializeConfigError(err)
    }
}

impl From<std::io::Error> for EnaError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}
