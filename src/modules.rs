//! PBot: The module of PBot.

pub mod base;
#[cfg(feature = "fwdmod")]
pub mod fwd;
#[cfg(feature = "getinfomod")]
pub mod getinfo;
#[cfg(feature = "addrankmod")]
pub mod addrank;
