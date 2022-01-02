//! PBot: Modules: AddRankModule
//!
//! You can add rank for every member you administrated
//! without giving the actual permission.

use actix::prelude::*;
use grammers_client::InputMessage;
use log::{debug, info};

use crate::telegram::{client::commands::GetAdminRightsBuilderCommand, user::is_root_user};

use super::base::{ModuleActivator, ModuleMessage, ModuleMeta};

const CMD_PREFIX: &str = "!addrank";

/// The AddRank actor.
#[derive(Clone, Default)]
pub struct AddRankModuleActor;

impl Actor for AddRankModuleActor {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        info!("🌟 {} started!", self.name());
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        info!("👋 {} stopped.", self.name());
    }
}

impl Handler<ModuleMessage> for AddRankModuleActor {
    type Result = ResponseActFuture<Self, anyhow::Result<()>>;

    fn handle(&mut self, msg: ModuleMessage, _: &mut Self::Context) -> Self::Result {
        // https://github.com/actix/actix/issues/308
        // DEVEDIT: You can clone the variables from self here to workaround this error.

        async move {
            // Destruct msg and get `handle` and `message`.
            let ModuleMessage { handle, message } = msg;

            // Extract the rank to set from the command message.
            let rank = match extract_rank(&*message.read().await) {
                // Return rank if the rank extracted successfully.
                Some(rank) => rank,
                // Otherwise, we return early.
                None => {
                    debug!("Failed to extract rank.");
                    return Ok(());
                }
            };

            // Check if this message is replying to a message,
            // and the message has a sender.
            //
            // FOR DEVS: don't merge the following two lines,
            // otherwise it will deadlock the thread.
            let user_replied_to = get_user_replied_to(&mut *message.write().await).await?;
            let user_replied_to = match user_replied_to {
                Some(user_replied_to) => user_replied_to,
                None => {
                    message
                        .write()
                        .await
                        .edit(InputMessage::text("[PBOT] ⚠️ 請回覆訊息。"))
                        .await?;

                    return Ok(());
                }
            };

            // Get the full name of user repiled to.
            //
            // We get it before `admin_builder`
            // since `GetAdminRightsBuilderCommand` will own
            // `user_replied_to`.
            let repiled_user_name = user_replied_to.full_name();

            // Get the admin builder.
            // The "Rank" is one of the administrator privileges.
            let mut admin_builder = handle
                .send(GetAdminRightsBuilderCommand {
                    channel: message.read().await.chat(),
                    user: user_replied_to,
                })
                .await?;

            // Set the rank and send the request to Telegram.
            admin_builder.rank(&rank).invoke().await?;

            // Notify user that the operation is succeed.
            message
                .write()
                .await
                .edit(InputMessage::text(format!(
                    "[PBOT] ✅ 成功將 {user} 的頭銜設定為 {rank}。",
                    user = repiled_user_name
                )))
                .await?;

            // It worked with no fault errors! 👌
            Ok(())
        }
        .into_actor(self)
        .boxed_local()
    }
}

impl ModuleMeta for AddRankModuleActor {
    fn name(&self) -> &'static str {
        // DEVEDIT: Your module name for users.
        "AddRankModule"
    }
}

impl ModuleActivator for AddRankModuleActor {}

/// Extract the rank argument from the command message.
fn extract_rank(message: &grammers_client::types::Message) -> Option<String> {
    // Get the message text.
    let msg_text = message.text().to_string();
    // Get the sender.
    let msg_from_root_user = is_root_user(message);
    // Split the message text by whitespace.
    // For example: `!addrank idiot` -> `Iterator(!addrank, idiot)`
    let mut cmd_list = msg_text.split_whitespace();

    // Get the command for checking if the command is matched.
    let cmd = cmd_list.next();

    // Check if we extracted the command sucussfully,
    // the command is CMD_PREFIX and the command message
    // is from the account owner (root user).
    if matches!(cmd, Some(s) if s == CMD_PREFIX) && msg_from_root_user {
        cmd_list.next().map(|s| s.to_string())
    } else {
        None
    }
}

/// Get the user of the message replied to.
async fn get_user_replied_to(
    message: &mut grammers_client::types::Message,
) -> anyhow::Result<Option<grammers_client::types::User>> {
    Ok(message
        .get_reply()
        .await?
        .and_then(|message_replied_to| message_replied_to.sender())
        .and_then(|sender| match sender {
            grammers_client::types::Chat::User(user) => Some(user),
            _ => None,
        }))
}
