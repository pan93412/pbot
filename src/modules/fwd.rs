//! PBot: Modules: `!fwd`

use grammers_client::{Client, types::Message};
use log::info;

use super::base::Module;

#[derive(Clone, Default)]
pub struct FwdModule;

#[async_trait::async_trait]
impl Module for FwdModule {
    fn name(&self) -> &'static str {
        "fwd_module"
    }

    async fn handle_updates(self, handle: Client, message: Message) -> anyhow::Result<()> {
        handle_updates(handle, message).await
    }
}

async fn handle_updates(handle: Client, message: Message) -> anyhow::Result<()> {
    info!("recv: {} [id={}, sender={}]", message.text(), message.id(), message.sender().map(|c| c.name().to_string()).unwrap_or_else(|| "None".to_string()));

    Ok(())
}
