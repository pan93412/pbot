//! PBot: Modules: Base Structure.

use std::sync::Arc;

use actix::{Handler, Message, Recipient};
use grammers_client::{types, Client};

pub struct ActivatedModuleInfo {
    pub name: &'static str,
    pub recipient: Recipient<ModuleMessage>,
}

/// The message that a PBot Module should receive.
#[derive(Message)]
#[rtype(result = "anyhow::Result<()>")]
pub struct ModuleMessage {
    /// The [`Client`] instance.
    pub handle: Arc<Client>,
    /// The message received.
    pub message: Arc<types::Message>,
}

/// The metadata that a PBot Module should have.
pub trait ModuleMeta {
    /// The name of this module.
    fn name(&self) -> &'static str;
}

/// The module activator.
pub trait ModuleActivator
where
    Self: Handler<ModuleMessage>,
{
    type Config;

    /// Activate this module and get [`ActivatedModuleInfo`] including
    /// the module name and the recipient to this module.
    fn activate_module(config: Self::Config) -> ActivatedModuleInfo;
}
