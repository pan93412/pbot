//! PBot: Modules: `!fwd`

use std::sync::Arc;

use actix::{fut::WrapFuture, Actor, ActorFutureExt, Context, Handler, ResponseActFuture};
use grammers_client::{types::Chat, InputMessage};
use log::{error, info, warn};

use crate::telegram::{client::commands::ForwardSingleMessageCommand, user::is_root_user};

use super::base::{ActivatedModuleInfo, ModuleActivator, ModuleMessage, ModuleMeta};

const CMD: &str = "!cufwd";

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
        info!("🌟 {} ({}) started!", self.name(), CMD);
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        info!("👋 {} ({}) stopped!", self.name(), CMD);
    }
}

impl Handler<ModuleMessage> for FwdModuleActor {
    type Result = ResponseActFuture<Self, anyhow::Result<()>>;

    fn handle(&mut self, msg: ModuleMessage, _: &mut Self::Context) -> Self::Result {
        let target = self.target.clone();
        async move {
            let ModuleMessage { handle, message } = msg;

            if message.text() == "!cufwd" && is_root_user(&message) {
                let reply_message_id = message.reply_to_message_id();
                let reply_message_src = Arc::new(message.chat());

                if let Some(reply_message_id) = reply_message_id {
                    let forward_result = handle
                        .send(ForwardSingleMessageCommand {
                            forward_to: target,
                            message_id: reply_message_id,
                            message_chat: reply_message_src,
                        })
                        .await?;

                    match forward_result {
                        Ok(_) => {
                            info!("💬 Message forwarded!");
                            (*message)
                                .clone()
                                .edit(InputMessage::text(
                                    "[PBOT] 💬 訊息已轉錄至個人群組。若要撤下請回覆告知。",
                                ))
                                .await?;
                        }
                        Err(e) => error!("Failed to forward message: {:?}", e),
                    }
                } else {
                    warn!("!cufwd: no reply message found");
                }
            }

            Ok(())
        }
        .into_actor(self)
        .boxed_local()
    }
}

impl ModuleMeta for FwdModuleActor {
    fn name(&self) -> &'static str {
        "FwdModule"
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
