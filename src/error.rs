/// PESIT error type.
#[derive(Debug, thiserror::Error)]
pub enum PesitError {
    #[error("IO error")]
    /// An error occurred while performing an IO operation.
    Io(#[from] std::io::Error),

    #[error("Parsing error")]
    /// An error occurred while parsing a frame.
    Parse,

    #[error("Protocol violation")]
    /// A protocol violation occurred.
    Protocol,

    #[error("Invalid frame")]
    /// The frame received from the client is invalid.
    InvalidFrame,
}
