pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    CtxFail,
    XValueNotOfType(&'static str),
    XPropertyNotFound(String),
    StoreFailToCreate(String),
    Sqlx(sqlx::Error),
    JsonSerde(serde_json::Error),
    Modql(modql::Error),
    ModqlOperatorNotSupported(String),
    IO(std::io::Error),
}

// region: ---- Froms.

impl From<modql::Error> for Error {
    fn from(value: modql::Error) -> Self {
        Error::Modql(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::JsonSerde(value)
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Error::Sqlx(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IO(value)
    }
}

// endregion: ---- Froms.

// region: ---- Error boilerplate

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

// endregion: ---- Error boilerplate
