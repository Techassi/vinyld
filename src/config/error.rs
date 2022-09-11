use std::fmt;

pub struct ConfigError {
    message: String,
}

impl ConfigError {
    pub fn new<M: Into<String>>(message: M) -> Self {
        return Self {
            message: message.into(),
        };
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl fmt::Debug for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ConfigError")
            .field("message", &self.message)
            .finish()
    }
}
