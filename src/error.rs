use serde::{Serialize, Serializer};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Call open_in_memory or open_in_path first to establish a connection.")]
    Connection(),
    #[error("Database error: {0}")]
    Database(String),
    #[error("Migration error: {0}")]
    Migration(String),
    #[error("Opening connection error: {0}")]
    OpeningConnection(String),
    #[error("Closing connection error: {0}")]
    ClosingConnection(String),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
