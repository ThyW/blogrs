#[derive(Debug)]
pub enum Error {
    Custom(String),
    Io(std::io::Error),
    Rocket(rocket::error::Error),
    ArgParse(crate::commands::ArgsError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Custom(e) => write!(f, "{}", e),
            Self::Io(e) => write!(f, "{}", e),
            Self::Rocket(e) => write!(f, "{}", e),
            Self::ArgParse(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}
impl From<rocket::error::Error> for Error {
    fn from(e: rocket::error::Error) -> Self {
        Self::Rocket(e)
    }
}

impl From<crate::commands::ArgsError> for Error {
    fn from(e: crate::commands::ArgsError) -> Self {
        Self::ArgParse(e)
    }
}

pub type BlogrsResult<T = ()> = Result<T, Error>;
