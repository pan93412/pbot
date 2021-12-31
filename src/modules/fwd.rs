//! PBot: Modules: `!fwd`

use std::sync::Arc;

use actix::{fut::WrapFuture, Actor, Context, ContextFutureSpawner, Handler};
use grammers_client::types::Chat;
use log::{error, info, warn};

use crate::telegram::{client::commands::ForwardSingleMessageCommand, user::is_root_user};

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
            let reply_message_src = Arc::new(message.chat());

            if let Some(reply_message_id) = reply_message_id {
                let target = self.target.clone();
                async move {
                    let client_result = handle
                        .send(ForwardSingleMessageCommand {
                            forward_to: target,
                            message_id: reply_message_id,
                            message_chat: reply_message_src,
                        })
                        .await;
                    match client_result {
                        Ok(forward_result) => match forward_result {
                            Ok(_) => info!("💬 Message forwarded!"),
                            Err(e) => error!("Failed to forward message: {:?}", e),
                        },
                        Err(e) => {
                            error!("Failed to request client to forward message: {:?}", e);
                        }
                    };
                }
                .into_actor(self)
                .spawn(ctx)
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
        let actor = Self {
            target: config.target,
        };
        let name = actor.name();
        let addr = actor.start();

        ActivatedModuleInfo {
            name,
            recipient: addr.recipient(),
        }
    }
}
