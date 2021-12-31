//! PBot: Modules: `!fwd`

use std::sync::Arc;

use actix::{Actor, Context, Handler, fut::WrapFuture, ContextFutureSpawner};
use grammers_client::{types::{Chat}};
use log::{info, error, warn};

use crate::telegram::user::is_root_user;

use super::base::{ActivatedModuleInfo, ModuleActivator, ModuleMessage, ModuleMeta};

/// The `!fwd` module.
#[derive(Clone)]
pub struct FwdModuleActor {
    pub target: Arc<Chat>,
}

/// The configuration of FwdModule.
pub struct FwdModuleConfig {
    pub target: Arc<Chat>,
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
                let target = self.target.clone();
                async move {
                    let forward_message = handle.forward_messages(&target, [reply_message_id].as_ref(), &reply_message_src).await;

                    if let Err(e) = forward_message {
                        error!("!cufwd: failed to forward message: {}", e);
                    }
                }.into_actor(self).spawn(ctx)
            } else {
                warn!("!cufwd: no reply message found");
            }
        }
        Ok(())
    }
}

impl ModuleMeta for FwdModuleActor {
    fn name(&self) -> &'static str {
        "FwdModule (!fwd)"
    }
}

impl ModuleActivator for FwdModuleActor {
    type Config = FwdModuleConfig;

    fn activate_module(config: Self::Config) -> ActivatedModuleInfo {
        let actor = Self { target: config.target };
        let name = actor.name();
        let addr = actor.start();

        ActivatedModuleInfo {
            name,
            recipient: addr.recipient(),
        }
    }
}
