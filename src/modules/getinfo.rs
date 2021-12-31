//! PBot: Modules: `getinfo`

use actix::{Actor, Context, Handler};
use log::info;

use super::base::{ActivatedModuleInfo, ModuleActivator, ModuleMessage, ModuleMeta};

/// The module for debugging.
#[derive(Clone)]
pub struct GetInfoModuleActor;

impl Actor for GetInfoModuleActor {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        info!("🌟 GetInfoModuleActor started!");
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        info!("👋 GetInfoModuleActor stopped!");
    }
}

impl Handler<ModuleMessage> for GetInfoModuleActor {
    type Result = anyhow::Result<()>;

    fn handle(&mut self, msg: ModuleMessage, _: &mut Self::Context) -> Self::Result {
        let ModuleMessage { handle: _, message } = msg;

        info!("MSG={:#?}; BY={:#?}; CHAT_ID={:#?}", message.text(), message.sender(), message.chat());
        Ok(())
    }
}

impl ModuleMeta for GetInfoModuleActor {
    fn name(&self) -> &'static str {
        "GetInfoModule"
    }
}

impl ModuleActivator for GetInfoModuleActor  {
    type Config = ();

    fn activate_module(_: Self::Config) -> ActivatedModuleInfo {
        let actor = Self;
        let name = actor.name();
        let addr = actor.start();

        ActivatedModuleInfo {
            name,
            recipient: addr.recipient(),
        }
    }
}
