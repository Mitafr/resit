pub mod api;
pub mod client;
pub mod connection;
pub mod error;
#[cfg(feature = "pool-bb8")]
pub mod pool;
pub mod protocol;
pub mod server;
pub mod state;

pub use api::PesitSession;

enum PesitMode {
    Server,
    Client,
}
