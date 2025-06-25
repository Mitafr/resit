pub trait State {}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum ClientState {
    #[default]
    Idle,
    Disconnected,
    Connected,
    Synchronized,
    Transferring,
    Terminated,
}

impl State for ClientState {}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum ServerState {
    #[default]
    Idle,
    Disconnected,
    Connected,
    Synchronized,
    Transferring,
    Terminated,
}
impl State for ServerState {}
