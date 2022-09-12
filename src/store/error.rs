use std::fmt;

pub struct StoreError {
    message: String,
}

impl StoreError {
    pub fn new<M: Into<String>>(message: M) -> Self {
        return Self {
            message: message.into(),
        };
    }
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl fmt::Debug for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StoreError")
            .field("message", &self.message)
            .finish()
    }
}
