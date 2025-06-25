#[derive(Debug, thiserror::Error)]
pub enum PesitError {
    #[error("IO error")]
    Io(#[from] std::io::Error),

    #[error("Parsing error")]
    Parse,

    #[error("Protocol violation")]
    Protocol,
}
