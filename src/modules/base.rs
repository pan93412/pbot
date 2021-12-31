//! PBot: Modules: Base Structure.

use grammers_client::{Client, types::Message};

/// The base structure of a module.
#[async_trait::async_trait]
pub trait Module {
    /// The identifier of this module.
    fn name(&self) -> &'static str;

    /// Handle the updates from [`client_handler`].
    async fn handle_updates(self, handle: Client, message: Message) -> anyhow::Result<()>;
}
