//! Commands for the client actor.

use std::sync::Arc;

use super::super::user::LoginConfig;
use actix::prelude::*;
use grammers_client::types::iter_buffer::InvocationError;
use grammers_client::types::{chat::PackedChat, Chat};
use grammers_client::{InputMessage, UpdateIter};

/// Logging in to Telegram.
#[derive(Message)]
#[rtype(result = "()")]
pub struct LoginCommand(pub LoginConfig);

/// Forward a single message to the specified chat.
#[derive(Message)]
#[rtype(result = "Result<Vec<Option<grammers_client::types::Message>>, InvocationError>")]
pub struct ForwardSingleMessageCommand {
    /// The chat to forward to.
    pub forward_to: Arc<Chat>,
    /// The message to forward.
    pub message_id: i32,
    /// The chat which the message was sent in.
    pub message_chat: Arc<Chat>,
}

/// Resolve the chat according to the specified chat_id.
#[derive(Message)]
#[rtype(result = "anyhow::Result<PackedChat>")]
pub struct ResolveChatCommand(pub i32);

/// Resolve the chat according to the specified chat_id.
#[derive(Message)]
#[rtype(result = "Result<Chat, InvocationError>")]
pub struct UnpackChatCommand(pub PackedChat);

/// Get the next updates.
#[derive(Message)]
#[rtype(result = "Result<Option<UpdateIter>, InvocationError>")]
pub struct NextUpdatesCommand;

/// Send message to the specified Chat.
#[derive(Message)]
#[rtype(result = "Result<grammers_client::types::Message, InvocationError>")]
pub struct SendMessageCommand(pub Chat, pub InputMessage);
