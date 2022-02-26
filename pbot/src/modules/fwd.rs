//! PBot: Modules: FwdModule
//!
//! Simply forward the message to your specified chat
//! with `!cufwd`.

use std::sync::Arc;

use actix::{fut::WrapFuture, Actor, ActorFutureExt, Context, Handler, ResponseActFuture};
use grammers_client::{
    types::{Chat, Message},
    InputMessage,
};
use log::{error, info, warn};
use pbot_modules_derive::{ModuleActivator, ModuleActor, ModuleMeta};

use crate::telegram::{client::commands::ForwardSingleMessageCommand, user::is_root_user};

use super::base::ModuleMessage;

const CMD: &str = "!cufwd";

/// The FwdModule actor.
#[derive(Clone, ModuleActor, ModuleActivator, ModuleMeta)]
#[name = "FwdModule"]
pub struct FwdModuleActor {
    /// Where the message will be forwarded to.
    pub target: Arc<Chat>,
}

impl Handler<ModuleMessage> for FwdModuleActor {
    type Result = ResponseActFuture<Self, anyhow::Result<()>>;

    fn handle(&mut self, msg: ModuleMessage, _: &mut Self::Context) -> Self::Result {
        // Clone self.target to move into the following block.
        let target = self.target.clone();

        // It will only respond when:
        //   * The message text is the text specified in CMD.
        //   * The message is sent by the account operator.
        let trigger_condition = |message: &Message| message.text() == CMD && is_root_user(message);

        async move {
            // Destruct msg and get `handle` and `message`.
            let ModuleMessage { handle, message } = msg;

            if trigger_condition(&*message.read().await) {
                // Get the ID of the chat where the message is sent.
                // It is Option here. We will check if replied anyone later.
                let reply_message_id = {
                    let message = message.read().await;
                    message.reply_to_message_id()
                };

                // Check if this message has been replied anyone.
                if let Some(reply_message_id) = reply_message_id {
                    // Yes - Get the chat with this replied message.
                    // Since the chat of replied message and the chat of this message are the same,
                    // we can use the chat of the command message to
                    // represent the chat of the replied message.
                    let reply_message_src = Arc::new(message.read().await.chat());

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

                            message
                                .write()
                                .await
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

                    message
                        .write()
                        .await
                        .edit(InputMessage::text("[PBOT] ⚠️ 請回覆訊息。"))
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
