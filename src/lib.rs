//! This module contains the implementation of the PESIT protocol.
//!
//! The PESIT protocol is a custom protocol designed for secure communication in file transfers.
//!
//! This library has been implemented without access to proprietary documentation.
//! Use it at your own risk. I do not guarantee its correctness or security.
//! Feel free to contribute to its development.
//!
//! server module expose the PesitServer struct, which is the main entry point for the server.
//! client module expose the PesitClient struct, which is the main entry point for the client.
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

/// API module for the PESIT protocol.
pub mod api;
pub(crate) mod client;
pub(crate) mod connection;
/// Error handling module for the PESIT protocol.
pub mod error;
#[cfg(feature = "pool-bb8")]
pub mod pool;
pub(crate) mod protocol;
/// Server module for the PESIT protocol.
pub mod server;
pub(crate) mod state;

pub use api::PesitSession;

enum PesitMode {
    Server,
    Client,
}
