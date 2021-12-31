use core::fmt;
use std::sync::Arc;

use actix::{Actor, Context, Handler, Message, Recipient};
use grammers_client::Client;
use grammers_client::Update::NewMessage;
use log::{debug, error, info};

use crate::modules::base::{ActivatedModuleInfo, ModuleMessage};
#[derive(Message)]
#[rtype(result = "anyhow::Result<()>")]
pub struct ClientModuleMessage {
    pub update: grammers_client::Update,
}

/// The executor that will distribute messages to modules..
pub struct ClientModuleExecutor {
    /// The client that will be used to handle updates.
    pub client: Arc<Client>,
    /// The modules that will be executed.
    ///
    /// The first element is the module name;
    /// the second element is the recipient of [`ModuleMessage`].
    pub modules: Vec<ActivatedModuleInfo>,
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
    type Result = anyhow::Result<()>;

    fn handle(&mut self, msg: ClientModuleMessage, _: &mut Self::Context) -> Self::Result {
        debug!("ClientModuleExector: start processing ClientModuleMessage");
        let message = match &msg.update {
            NewMessage(message) => Ok(Arc::new(message.clone())),
            _ => Err(UnhandledMessage),
        }?;

        for module in self.modules.iter() {
            let handle = self.client.clone();
            let message = message.clone();

            let recv = module.recipient.try_send(ModuleMessage { handle, message });

            if let Err(e) = recv {
                error!("failed to broadcast message to {}: {:?}", module.name, e);
            }
        }

        Ok(())
    }
}

/// The error that will be returned when the received message was not handled.
#[derive(Debug)]
struct UnhandledMessage;
impl fmt::Display for UnhandledMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "got a unhandled message")
    }
}
impl std::error::Error for UnhandledMessage {}

/// The root handler of client.
///
/// It includes the Ctrl-C handler.
pub async fn client_handler(
    client: &Client,
    executors: Recipient<ClientModuleMessage>,
) -> anyhow::Result<()> {
    while let Some(updates) = tokio::select! {
        _ = tokio::signal::ctrl_c() => Ok(None),
        result = client.next_updates() => result,
    }? {
        for update in updates {
            let result = executors.send(ClientModuleMessage { update }).await?;

            if let Err(e) = result {
                error!("client_handler(): error in exectutors: {:?}", e);
            }
        }
    }

    Ok(())
}
