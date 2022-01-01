//! PBot: Modules: GetInfoModule
//! 
//! Get the information of the message.
//! For debugging purpose.

use actix::prelude::*;
use log::info;

use super::base::{ActivatedModuleInfo, ModuleActivator, ModuleMessage, ModuleMeta};

/// The GetInfoModule module that is for debugging.
///
/// We don't recommended you enabling this without a reasonable reason,
/// since it is useless while noising.
#[derive(Clone)]
pub struct GetInfoModuleActor;

impl Actor for GetInfoModuleActor {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        info!("🌟 {} started!", self.name());
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        info!("👋 {} stopped!", self.name());
    }
}

impl Handler<ModuleMessage> for GetInfoModuleActor {
    type Result = ResponseActFuture<Self, anyhow::Result<()>>;

    fn handle(&mut self, msg: ModuleMessage, _: &mut Self::Context) -> Self::Result {
        // Destruct msg and get `handle` and `message`.
        let ModuleMessage { handle: _, message } = msg;

        async move {
            // Show the text, sender and chat of this message.
            info!(
                "MSG={:#?}; BY={:#?}; CHAT_ID={:#?}",
                message.read().await.text(),
                message.read().await.sender(),
                message.read().await.chat()
            );

            Ok(())
        }
        .into_actor(self)
        .boxed_local()
    }
}

impl ModuleMeta for GetInfoModuleActor {
    fn name(&self) -> &'static str {
        "GetInfoModule"
    }
}

impl ModuleActivator for GetInfoModuleActor {
    type Config = ();

    fn activate_module(_: Self::Config) -> ActivatedModuleInfo {
        // Create the instance.
        let actor = Self;
        // Get the actor name before consumed.
        let name = actor.name();
        // Start this instance and retrieve its address.
        let addr = actor.start();

        ActivatedModuleInfo {
            name,
            recipient: addr.recipient(),
        }
    }
}
