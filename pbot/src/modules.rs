//! PBot: The module of PBot.

#[cfg(feature = "addrankmod")]
pub mod addrank;
pub mod base;
#[cfg(feature = "fwdmod")]
pub mod fwd;
#[cfg(feature = "getinfomod")]
pub mod getinfo;
