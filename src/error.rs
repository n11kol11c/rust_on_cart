use std::fmt;

#[derive(Debug)]
pub enum CartError {
    Message(String),
}

impl fmt::Display for CartError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CartError::Message(s) => write!(f, "{s}"),
        }
    }
}

impl<E: ToString> From<E> for CartError {
    fn from(e: E) -> Self {
        CartError::Message(e.to_string())
    }
}
