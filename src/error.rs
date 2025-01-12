#[derive(Debug)]
pub enum RuntimeError {
    Io(std::io::Error),
    Curl(curl::Error),
}

pub type AppResult<T> = Result<T, RuntimeError>;

pub struct NotFound(std::io::Error);

impl NotFound {
    pub fn new(error: &str) -> Self {
        Self(std::io::Error::new(std::io::ErrorKind::NotFound, error))
    }
}

impl From<NotFound> for RuntimeError {
    fn from(err: NotFound) -> Self {
        Self::Io(err.0)
    }
}

pub struct InvalidData(std::io::Error);

impl InvalidData {
    pub fn new(error: &str) -> Self {
        Self(std::io::Error::new(std::io::ErrorKind::InvalidData, error))
    }
}

impl From<InvalidData> for RuntimeError {
    fn from(err: InvalidData) -> Self {
        Self::Io(err.0)
    }
}

pub struct AlreadyExists(std::io::Error);

#[allow(dead_code)]
impl AlreadyExists {
    pub fn new(error: &str) -> Self {
        Self(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            error,
        ))
    }
}

impl From<AlreadyExists> for RuntimeError {
    fn from(err: AlreadyExists) -> Self {
        Self::Io(err.0)
    }
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Io(err) => std::fmt::Display::fmt(err, f),
            Self::Curl(err) => std::fmt::Display::fmt(err, f),
        }
    }
}

impl From<std::io::Error> for RuntimeError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<curl::Error> for RuntimeError {
    fn from(err: curl::Error) -> Self {
        Self::Curl(err)
    }
}

pub trait NotFoundExt<T> {
    fn ok_or_not_found(self, error: &str) -> AppResult<T>;
}

impl<T> NotFoundExt<T> for Option<T> {
    fn ok_or_not_found(self, error: &str) -> AppResult<T> {
        self.ok_or_else(|| NotFound::new(error).into())
    }
}
