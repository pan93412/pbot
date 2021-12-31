use std::sync::Arc;

use actix::prelude::*;

use grammers_client::Update::NewMessage;
use log::{debug, error, info};

use crate::modules::base::{ActivatedModuleInfo, ModuleMessage};

use super::client::ClientActor;
#[derive(Message)]
#[rtype(result = "anyhow::Result<()>")]
pub struct ClientModuleMessage {
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
        let modules = self.modules.clone();
        let handle = self.client.clone();
        let message = match msg.update {
            NewMessage(message) => Ok(message),
            _ => Err(anyhow::anyhow!("got a unhandled message")),
        };

        async move {
            debug!("ClientModuleExector: start processing ClientModuleMessage");
            let message = Arc::new(message?);

            for module in modules.iter() {
                let recipient = module.recipient.clone();
    
                let recv = recipient.send(ModuleMessage {
                    handle: handle.clone(),
                    message: message.clone()
                }).await?;
    
                if let Err(e) = recv {
                    error!("failed to broadcast message to {}: {:?}", module.name, e);
                }
            }

            Ok(())
        }
            .into_actor(self)
            .boxed_local()
    }
}
