//! PBot: Modules: `!fwd`

use std::sync::Arc;

use actix::{Actor, Context, Handler, fut::WrapFuture, AsyncContext, ActorFutureExt, ContextFutureSpawner};
use grammers_client::{Client, types::chat::PackedChat};
use log::{info, error, warn};
use tokio::sync::OnceCell;

use crate::{getenv, utils::is_root_user};

use super::base::{ActivatedModuleInfo, ModuleActivator, ModuleMessage, ModuleMeta};

/// The `!fwd` module.
#[derive(Clone)]
pub struct FwdModuleActor {
    pub target: PackedChat,
}

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

    fn handle(&mut self, msg: ModuleMessage, ctx: &mut Self::Context) -> Self::Result {
        let ModuleMessage { handle, message } = msg;
        
        if message.text() == "!cufwd" && is_root_user(&*message) {
            let reply_message_id = message.reply_to_message_id();
            let reply_message_src = message.chat();

            if let Some(reply_message_id) = reply_message_id {
                async {
                    let cugroup = handle.unpack_chat(&self.target).await;
                    let forward_message = handle.forward_messages(&cugroup, &reply_message_id, &reply_message_src);
    
                    if let Err(e) = forward_message {
                        error!("Failed to forward message: {}", e);
                    }
                }.into_actor(self).wait(ctx);
            } else {
                warn!("!cufwd: no reply message found");
            }
        }
        let sender = message
            .sender()
            .map(|c| c.id().to_string())
            .unwrap_or_else(|| "None".to_string());

        info!(
            "recv: {} [id={}, sender={}, is_root={}]",
            message.text(),
            message.id(),
            sender,
            sender == getenv!("TG_ROOT_USER")
        );

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
