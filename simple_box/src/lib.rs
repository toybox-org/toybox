#[cfg(feature = "client")]
pub mod client;

#[cfg(feature = "server")]
pub mod server;

#[cfg(feature = "gui")]
pub mod renderer;

pub use shared::plugin;
pub use shared::protocol;
