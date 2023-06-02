#![allow(clippy::missing_errors_doc)]

mod ci;
mod config_hook;
mod declare_hook;
mod error;
mod local;
mod meta;
mod pre_commit;
mod pre_commit_config;
mod pre_commit_hooks;
mod remote;
mod repo;

pub use ci::*;
pub use config_hook::*;
pub use declare_hook::*;
pub use error::*;
pub use local::*;
pub use meta::*;
pub use pre_commit::*;
pub use pre_commit_config::*;
pub use pre_commit_hooks::*;
pub use remote::*;
pub use repo::*;
