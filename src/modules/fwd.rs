//! PBot: Modules: `!fwd`

use actix::{Actor, Context, Handler};
use log::info;

use super::{base::{ModuleMessage, ModuleMeta, ActivatedModuleInfo, ModuleActivator}};

/// The `!fwd` module.
#[derive(Clone, Default)]
pub struct FwdModuleActor;

impl Actor for FwdModuleActor {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        info!("🌟 FwdModuleActor (!fwd) started!");
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        info!("👋 FwdModuleActor (!fwd) stopped!");
    }
}

impl Handler<ModuleMessage> for FwdModuleActor {
    type Result = anyhow::Result<()>;

    fn handle(&mut self, msg: ModuleMessage, _ctx: &mut Self::Context) -> Self::Result {
        let ModuleMessage { handle: _, message } = msg;
        info!("recv: {} [id={}, sender={}]", message.text(), message.id(), message.sender().map(|c| c.name().to_string()).unwrap_or_else(|| "None".to_string()));

        Ok(())
    }
}

impl ModuleMeta for FwdModuleActor {
    fn name(&self) -> &'static str {
        "FwdModule (!fwd)"
    }
}

impl ModuleActivator for FwdModuleActor {
    fn activate_module() -> ActivatedModuleInfo {
        let actor = Self::default();
        let name = actor.name();
        let addr = actor.start();
        
        ActivatedModuleInfo {
            name,
            recipient: addr.recipient(),
        }
    }
}
