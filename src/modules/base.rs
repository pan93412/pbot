//! PBot: Modules: Base Structure and Traits
//!
//! The base structure and traits of the PBot modules.

use std::sync::Arc;
use tokio::sync::RwLock;

use actix::{Addr, Handler, Message, Recipient};

use crate::telegram::client::ClientActor;
use grammers_client::types;

/// The information of the module which has been initiated and activated.
pub struct ActivatedModuleInfo {
    /// The name of this module.
    pub name: &'static str,
    /// The module recipient.
    ///
    /// It'll be used by [`crate::telegram::update::ClientModuleExecutor`].
    pub recipient: Recipient<ModuleMessage>,
}

/// The message that a PBot Module would receive.
#[derive(Message)]
#[rtype(result = "anyhow::Result<()>")]
pub struct ModuleMessage {
    /// The address to a [`ClientActor`] instance.
    pub handle: Addr<ClientActor>,
    /// The message received.
    pub message: Arc<RwLock<types::Message>>,
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
    /// The configuration structure that'll pass to activate_module
    ///
    /// You can receive this structure in [`Self::activate_module`]
    /// for initizating your module.
    type Config;

    /// Activate this module and get [`ActivatedModuleInfo`] including
    /// the module name and the recipient to this module.
    fn activate_module(config: Self::Config) -> ActivatedModuleInfo;
}
