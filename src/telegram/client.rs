//! PBot: Telegram: Client Actor

pub mod commands;

use std::sync::{Arc, Mutex};

use actix::prelude::*;
use grammers_client::types::iter_buffer::InvocationError;
use grammers_client::UpdateIter;
use grammers_client::{
    types::{chat::PackedChat, Chat},
    Client,
};

use self::commands::{
    ForwardSingleMessageCommand, LoginCommand, NextUpdatesCommand, ResolveChatCommand,
    UnpackChatCommand,
};

use super::user::login;

use log::{info, debug};

/// The Telegram client actor.
#[derive(Default)]
pub struct ClientActor {
    client: Option<Arc<Mutex<Client>>>,
}

impl ClientActor {
    /// Get the client instance.
    pub fn get_client(&mut self) -> Arc<Mutex<Client>> {
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

    fn handle(&mut self, msg: LoginCommand, _: &mut Context<Self>) -> Self::Result {
        async { login(msg.0).await.expect("failed to login") }
            .into_actor(self)
            .map(|value, act, _ctx| {
                // Set the field `client` to our value.
                act.client = Some(Arc::new(Mutex::new(value)));
            })
            .boxed_local()
    }
}

impl Handler<ForwardSingleMessageCommand> for ClientActor {
    type Result = ResponseActFuture<
        Self,
        Result<Vec<Option<grammers_client::types::Message>>, InvocationError>,
    >;

    fn handle(
        &mut self,
        msg: ForwardSingleMessageCommand,
        _ctx: &mut Context<Self>,
    ) -> Self::Result {
        let client = self.get_client();

        async move {
            let mut client = client.lock().unwrap();
            client
                .forward_messages(&msg.forward_to, &[msg.message_id], &msg.message_chat)
                .await
        }
        .into_actor(self)
        .boxed_local()
    }
}

impl Handler<ResolveChatCommand> for ClientActor {
    type Result = ResponseActFuture<Self, anyhow::Result<PackedChat>>;

    fn handle(&mut self, msg: ResolveChatCommand, _: &mut Context<Self>) -> Self::Result {
        let client = self.get_client();

        async move {
            let client = client.lock().unwrap();
            let mut dialogs = client.iter_dialogs();

            while let Some(dialog) = dialogs.next().await? {
                let chat = dialog.chat();
                debug!("comparing: {:#?}", chat);
                if chat.id() == msg.0 {
                    let packed_chat = chat.pack();
                    return Ok(packed_chat);
                }
            }

            Err(anyhow::anyhow!("No such a group."))
        }
        .into_actor(self)
        .boxed_local()
    }
}

impl Handler<UnpackChatCommand> for ClientActor {
    type Result = ResponseActFuture<Self, Result<Chat, InvocationError>>;

    fn handle(&mut self, msg: UnpackChatCommand, _: &mut Context<Self>) -> Self::Result {
        let client = self.get_client();

        async move {
            let mut client = client.lock().unwrap();

            client.unpack_chat(&msg.0).await
        }
        .into_actor(self)
        .boxed_local()
    }
}

impl Handler<NextUpdatesCommand> for ClientActor {
    type Result = ResponseActFuture<Self, Result<Option<UpdateIter>, InvocationError>>;

    fn handle(&mut self, _: NextUpdatesCommand, _: &mut Context<Self>) -> Self::Result {
        let client = self.get_client();

        async move {
            let client = client.lock().unwrap();
            client.next_updates().await
        }
        .into_actor(self)
        .boxed_local()
    }
}
