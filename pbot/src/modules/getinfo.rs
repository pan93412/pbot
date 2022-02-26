//! PBot: Modules: GetInfoModule
//!
//! Get the information of the message.
//! For debugging purpose.

use actix::prelude::*;
use log::info;
use pbot_modules_derive::{ModuleActivator, ModuleActor, ModuleMeta};

use super::base::ModuleMessage;

/// The GetInfoModule module that is for debugging.
///
/// We don't recommended you enabling this without a reasonable reason,
/// since it is useless while noising.
#[derive(Clone, ModuleActor, ModuleActivator, ModuleMeta)]
#[name = "GetInfoModule"]
pub struct GetInfoModuleActor;

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
