/// Marker trait for representing different states in the client-server protocol.
pub trait State {}

/// Represents the current state of a `PesitClient`.
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum ClientState {
    #[default]
    /// The client is idle and not connected to any server.
    Idle,
    /// The client is disconnected from the server.
    Disconnected,
    /// The client is connected to the server.
    Connected,
    /// The client is in the process of file selection.
    FileSelection,
    /// The client is synchronized with the server.
    Synchronized,
    /// The client is currently transferring a file.
    Transferring,
    /// Indicates that the server has terminated the session.
    Terminated,
}

impl State for ClientState {}

/// Represents the current state of a `PesitServer`.
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum ServerState {
    #[default]
    /// The server is idle and not connected to any client.
    Idle,
    /// The server is disconnected from the client.
    Disconnected,
    /// The server is connected to the client.
    Connected,
    /// The server is in the process of file selection.
    FileSelection,
    /// The server is synchronized with the client.
    Synchronized,
    /// The server is currently transferring a file.
    Transferring,
    /// Indicates that the server has terminated the session.
    Terminated,
}
impl State for ServerState {}
