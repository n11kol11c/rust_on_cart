#[derive(Debug)]
pub enum CartError {
    IoError(String),
    DbError(String),
    NetworkError(String),
    ResourceError(String),
    Exception(String),
}
