//! PBot: Modules: FwdModule

use std::sync::Arc;

use actix::{fut::WrapFuture, Actor, ActorFutureExt, Context, Handler, ResponseActFuture};
use grammers_client::{types::Chat, InputMessage};
use log::{error, info, warn};

use crate::telegram::{client::commands::ForwardSingleMessageCommand, user::is_root_user};

use super::base::{ActivatedModuleInfo, ModuleActivator, ModuleMessage, ModuleMeta};

const CMD: &str = "!cufwd";

/// The FwdModule actor.
#[derive(Clone)]
pub struct FwdModuleActor {
    /// Where the message will be forwarded to.
    pub target: Arc<Chat>,
}

/// The configuration of FwdModule.
pub struct FwdModuleConfig {
    /// Where the message will be forwarded to.
    /// 
    /// It will be used to initiate [`FwdModuleActor`].
    pub target: Arc<Chat>,
}

impl Actor for FwdModuleActor {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        info!("🌟 {} started! You can use it with {}.", self.name(), CMD);
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        info!("👋 {} ({}) stopped.", self.name(), CMD);
    }
}

impl Handler<ModuleMessage> for FwdModuleActor {
    type Result = ResponseActFuture<Self, anyhow::Result<()>>;

    fn handle(&mut self, msg: ModuleMessage, _: &mut Self::Context) -> Self::Result {
        // Clone self.target to move into the following block.
        let target = self.target.clone();

        async move {
            // Destruct msg and get `handle` and `message`.
            let ModuleMessage { handle, message } = msg;

            // It will only respond when:
            //   * The message text is the text specified in CMD.
            //   * The message is sent by the account operator.
            if message.text() == CMD && is_root_user(&message) {
                // Get the ID of the chat where the message is sent.
                // It is Option here. We will check if replied anyone later.
                let reply_message_id = message.reply_to_message_id();

                // Check if this message has been replied anyone.
                if let Some(reply_message_id) = reply_message_id {
                    // Yes - Get the chat with this replied message.
                    // Since the chat of replied message and the chat of this message are the same,
                    // we can use the chat of the command message to
                    // represent the chat of the replied message.
                    let reply_message_src = Arc::new(message.chat());

                    // Forward the message.
                    let forward_result = handle
                        .send(ForwardSingleMessageCommand {
                            forward_to: target,
                            message_id: reply_message_id,
                            message_chat: reply_message_src,
                        })
                        .await?;

                    // Check if the message has been forwarded successfully.
                    match forward_result {
                        // 👏 Great! Let's notify the sender of replied message.
                        Ok(_) => {
                            info!("💬 Message forwarded!");

                            // .edit() requires the message to be mutable,
                            // but this message is a Arc pointer and thus immutable.
                            //
                            // We need to get the original message instance itself
                            // and clone it.
                            //
                            // FIXME: It is pretty costly.
                            (*message)
                                .clone()
                                .edit(InputMessage::text(
                                    "[PBOT] 💬 訊息已轉錄至個人群組。若要撤下請回覆告知。",
                                ))
                                .await?;
                        }
                        // Show the error rather than panic!() it.
                        Err(e) => error!("Failed to forward message: {:?}", e),
                    }
                } else {
                    // No - Let user know how to use it correctly.
                    warn!("No reply message found");

                    // .edit() requires the message to be mutable,
                    // but this message is a Arc pointer and thus immutable.
                    //
                    // We need to get the original message instance itself
                    // and clone it.
                    //
                    // FIXME: It is pretty costly.
                    (*message)
                        .clone()
                        .edit(InputMessage::text(
                            "[PBOT] ⚠️ 請回覆訊息。",
                        ))
                        .await?;
                }
            }

            // It worked with no fault errors! 👌
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
        // Create the instance with the config.
        let actor = Self {
            target: config.target,
        };
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
