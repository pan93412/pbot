//! PBot: Telegram: Client Actor
//!
//! This encapsulates the Telegram client as a Actor
//! so we can manage and track the instance well.

pub mod commands;

use actix::prelude::*;
use grammers_client::types::AdminRightsBuilder;

use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;

use grammers_client::types::iter_buffer::InvocationError;
use grammers_client::UpdateIter;
use grammers_client::{
    types::{chat::PackedChat, Chat},
    Client,
};

use self::commands::{
    ForwardSingleMessageCommand, GetAdminRightsBuilderCommand, LoginCommand, NextUpdatesCommand,
    ResolveChatCommand, SendMessageCommand, UnpackChatCommand, SaveSessionToFileCommand,
};

use super::user::login;

use log::{debug, info};

/// The Telegram client actor.
#[derive(Default)]
pub struct ClientActor {
    client: Option<Arc<RwLock<Client>>>,
}

impl ClientActor {
    /// Get the `Arc<RwLock<Client>>` instance.
    pub fn get_client(&mut self) -> Arc<RwLock<Client>> {
        self.client
            .clone()
            .expect("You must login your Telegram first.")
    }
}

impl Actor for ClientActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut <Self as actix::Actor>::Context) {
        info!("🌟 Telegram Client started!");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("👋 Telegram Client stopped!");
    }
}

impl Handler<LoginCommand> for ClientActor {
    type Result = ResponseActFuture<Self, ()>;

    /// Logging in to Telegram.
    fn handle(&mut self, msg: LoginCommand, _: &mut Context<Self>) -> Self::Result {
        // Call login() method to login your Telegram account.
        //
        // * It is a interactive method. You would need to enter your credentials.
        async { login(msg.0).await.expect("failed to login") }
            .into_actor(self)
            .map(|value, act, _ctx| {
                // Set the field `client` to the value returned from login().
                act.client = Some(Arc::new(RwLock::new(value)));
            })
            .boxed_local()
    }
}

impl Handler<ForwardSingleMessageCommand> for ClientActor {
    type Result = ResponseActFuture<
        Self,
        Result<Vec<Option<grammers_client::types::Message>>, InvocationError>,
    >;

    /// Forward a single message to the specified chat.
    fn handle(
        &mut self,
        msg: ForwardSingleMessageCommand,
        _ctx: &mut Context<Self>,
    ) -> Self::Result {
        // Get the unwrapped client.
        let client = self.get_client();

        async move {
            // Forward the message.
            client
                .write()
                .await
                .forward_messages(&msg.forward_to, &[msg.message_id], &msg.message_chat)
                .await
        }
        .into_actor(self)
        .boxed_local()
    }
}

impl Handler<ResolveChatCommand> for ClientActor {
    type Result = ResponseActFuture<Self, anyhow::Result<PackedChat>>;

    /// Resolve the chat according to the specified chat_id.
    fn handle(&mut self, msg: ResolveChatCommand, _: &mut Context<Self>) -> Self::Result {
        // Get the unwrapped client.
        let client = self.get_client();

        async move {
            // Get the dialogs iterator.
            //
            // We let the lock period as short as possible by
            // making a block for it.
            let mut dialogs = {
                // Get the dialogs iterator.
                //
                // "Dialogs" is full list of chats with messages and auxiliary data.
                // https://core.telegram.org/constructor/messages.dialogs
                client.read().await.iter_dialogs()
            };

            // Iterate over the dialogs.
            while let Some(dialog) = dialogs.next().await? {
                // Get the chat information of this dialog.
                let chat = dialog.chat();

                debug!("comparing: {:#?}", chat);

                // Check if the ID of this chat is same as the ID of the chat we are looking for.
                if chat.id() == msg.0 {
                    // Yes - pack the chat reference and return it.
                    let packed_chat = chat.pack();
                    return Ok(packed_chat);
                }
            }

            // If we reach here, it means the chat was not found.
            Err(anyhow::anyhow!("No such a group."))
        }
        .into_actor(self)
        .boxed_local()
    }
}

impl Handler<UnpackChatCommand> for ClientActor {
    type Result = ResponseActFuture<Self, Result<Chat, InvocationError>>;

    /// Resolve the chat according to the specified chat_id.
    fn handle(&mut self, msg: UnpackChatCommand, _: &mut Context<Self>) -> Self::Result {
        // Get the unwrapped client.
        let client = self.get_client();

        async move {
            // Unpack the chat.
            client.write().await.unpack_chat(&msg.0).await
        }
        .into_actor(self)
        .boxed_local()
    }
}

impl Handler<NextUpdatesCommand> for ClientActor {
    type Result = ResponseActFuture<Self, Result<Option<UpdateIter>, InvocationError>>;

    /// Get the next updates.
    fn handle(&mut self, _: NextUpdatesCommand, _: &mut Context<Self>) -> Self::Result {
        let client = self.get_client();

        async move {
            // Get the next round of updates.
            client.read().await.next_updates().await
        }
        .into_actor(self)
        .boxed_local()
    }
}

impl Handler<SendMessageCommand> for ClientActor {
    type Result = ResponseActFuture<Self, Result<grammers_client::types::Message, InvocationError>>;

    /// Send message to the specified Chat.
    fn handle(&mut self, cmd: SendMessageCommand, _: &mut Context<Self>) -> Self::Result {
        let client = self.get_client();
        let SendMessageCommand(chat, message) = cmd;

        async move {
            // Send message.
            client.write().await.send_message(&chat, message).await
        }
        .into_actor(self)
        .boxed_local()
    }
}

impl Handler<GetAdminRightsBuilderCommand> for ClientActor {
    type Result = ResponseActFuture<Self, AdminRightsBuilder>;

    /// Get the admin rights builder.
    fn handle(&mut self, cmd: GetAdminRightsBuilderCommand, _: &mut Context<Self>) -> Self::Result {
        let client = self.get_client();
        let GetAdminRightsBuilderCommand { channel, user } = cmd;

        async move {
            client.write().await.set_admin_rights(&channel, &user)
        }
        .into_actor(self)
        .boxed_local()
    }
}

impl<T> Handler<SaveSessionToFileCommand<T>> for ClientActor where T: 'static + AsRef<Path> {
    type Result = ResponseActFuture<Self, std::io::Result<()>>;

    /// Save the current session to file.
    /// 
    /// The first element is the file to save.
    fn handle(&mut self, cmd: SaveSessionToFileCommand<T>, _: &mut Context<Self>) -> Self::Result {
        let client = self.get_client();
        async move {
            client.write().await.session().save_to_file(cmd.0)
        }
        .into_actor(self)
        .boxed_local()
    }
}
