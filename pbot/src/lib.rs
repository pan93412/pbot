//! PBot Library
//!
//! It includes the PBot modules, PBot Telegram clients encapsulation,
//! and the utils such as [`getenv`].

#![warn(missing_docs)]
pub mod modules;
pub mod telegram;
pub mod utils;

/// The path to store the Telegram session.
pub const SESSION_PATH: &str = "./.telegram.session.dat";
