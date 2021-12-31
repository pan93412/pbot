use grammers_client::Client;
use grammers_client::Update::NewMessage;
use log::{error, debug};

use crate::modules::base::Module;

/// The main handler of client.
/// 
/// It includes the Ctrl-C handler.
pub async fn client_handler<M: 'static + Module + Clone + Send>(client: Client, mods: Vec<M>) -> anyhow::Result<()> {
    while let Some(updates) = tokio::select! {
        _ = tokio::signal::ctrl_c() => Ok(None),
        result = client.next_updates() => result,
    }? {
        for update in updates {
            for module in mods.iter() {
                debug!("Broadcast this module to: {}", module.name());

                let module = module.clone();
                let handle = client.clone();
                let message = match &update {
                    NewMessage(message) => Some(message.clone()),
                    _ => None,
                };

                tokio::task::spawn(async move {
                    let name = module.name();
                    let update = module.handle_updates(handle, message.unwrap()).await;

                    if let Err(error) = update {
                        error!("Error occurred in module: {}, error: {}", name, error);
                    }
                });
            }
        }
    }

    Ok(())
}
