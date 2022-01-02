//! PBot: Telegram: The client module executors.

use actix::prelude::*;

use log::{error, info};
use std::sync::Arc;
use tokio::sync::RwLock;

use super::client::ClientActor;

use grammers_client::Update::NewMessage;

use crate::modules::base::{ActivatedModuleInfo, ModuleMessage};

/// The message for a ClientModule.
///
/// See main.rs > Phase V: Polling updates
#[derive(Message)]
#[rtype(result = "anyhow::Result<()>")]
pub struct ClientModuleMessage {
    /// The update event. See main.rs > Phase V: Polling updates
    pub update: grammers_client::Update,
}

/// The executor that will distribute messages to modules..
pub struct ClientModuleExecutor {
    /// The client that will be used to handle updates.
    pub client: Addr<ClientActor>,
    /// The modules that will be executed.
    ///
    /// The first element is the module name;
    /// the second element is the recipient of [`ModuleMessage`].
    pub modules: Arc<Vec<ActivatedModuleInfo>>,
}

impl Actor for ClientModuleExecutor {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        info!("🌟 Client Module Executor started!");
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        info!("👋 Client Module Executor stopped!");
    }
}

impl Handler<ClientModuleMessage> for ClientModuleExecutor {
    type Result = ResponseActFuture<Self, anyhow::Result<()>>;

    fn handle(&mut self, msg: ClientModuleMessage, _ctx: &mut Self::Context) -> Self::Result {
        // https://github.com/actix/actix/issues/308
        // We clone the variables from self to workaround this error.
        let modules = self.modules.clone();
        let handle = self.client.clone();
        let message = match msg.update {
            NewMessage(message) => Ok(message),
            _ => Err(anyhow::anyhow!("got a unhandled message")),
        };

        async move {
            // Make a Arc smart pointer to the message
            // so we can share it with those modules.
            let message = Arc::new(RwLock::new(message?));

            // Join all the .send() futures.
            futures::future::join_all(modules.iter().map(|module| async {
                // Forward our handle and message to the module.
                //
                // Note that we clone() twice - first to workaround the lifetime issue,
                // this to let the every modules consume.
                let error = module
                    .recipient
                    .send(ModuleMessage {
                        handle: handle.clone(),
                        message: message.clone(),
                    })
                    .await
                    .unwrap();

                // module.name is the module name;
                // e is the error from module.recipient.send().
                if let Err(e) = error {
                    error!("error in {}: {:?}", module.name, e);
                }
            }))
            .await;

            Ok(())
        }
        .into_actor(self)
        .boxed_local()
    }
}
