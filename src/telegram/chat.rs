use std::sync::Arc;

use grammers_client::{Client, types::chat::PackedChat};

/// Resolve the specified chat_id to a PackedChat.
/// 
/// You must join that chat before using this.
async fn resolve_chat(handle: Arc<Client>, chat_id: i32) -> anyhow::Result<PackedChat> {
    while let Some(dialog) = handle.iter_dialogs().next().await? {
        let chat = dialog.chat();
        if chat.id() == chat_id {
            let packed_chat = chat.pack();
            return Ok(packed_chat);
        }
    }

    Err(anyhow::anyhow!("No such a group."))
}
