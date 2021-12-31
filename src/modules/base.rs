//! PBot: Modules: Base Structure.

use std::sync::Arc;

use actix::{Message, Actor, Handler, Recipient};
use grammers_client::{Client, types};

pub struct ActivatedModuleInfo<A: Actor + Handler<ModuleMessage>> {
    pub name: &'static str,
    pub actor: A,
    pub recipient: Recipient<ModuleMessage>,
}

/// The message that a PBot Module should receive.
#[derive(Message)]
#[rtype(result = "anyhow::Result<()>")]
pub struct ModuleMessage {
    /// The [`Client`] instance.
    pub handle: Arc<Client>,
    /// The message received.
    pub message: Arc<types::Message>
}

/// The metadata that a PBot Module should have.
pub trait ModuleMeta {
    /// The name of this module.
    fn name(&self) -> &'static str;
}

/// The module activator.
pub trait ModuleActivator
    where Self: Handler<ModuleMessage> {
    /// Activate this module and get [`ActivatedModuleInfo`] with
    /// the module name, actor instance and the actor address included.
    fn activate_module() -> ActivatedModuleInfo<Self>;
}
