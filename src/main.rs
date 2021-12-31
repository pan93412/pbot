use actix::prelude::*;

use dotenv::dotenv;
use simple_logger::SimpleLogger;

use pbot::getenv;
use pbot::SESSION_PATH;

use log::error;
use std::sync::Arc;

use pbot::modules::{
    base::ModuleActivator,
    fwd::{FwdModuleActor, FwdModuleConfig},
};
use pbot::telegram::{
    client::{
        commands::{LoginCommand, NextUpdatesCommand, ResolveChatCommand, UnpackChatCommand},
        ClientActor,
    },
    update::{ClientModuleExecutor, ClientModuleMessage},
    user::LoginConfig,
};

#[actix::main]
async fn main() {
    /* Phase I: Initiate loggers and dotenv */
    SimpleLogger::new()
        .with_utc_timestamps()
        .with_level(log::LevelFilter::Info)
        .init()
        .expect("failed to configure logger");
    dotenv().expect("a .env file should be existed in the current working directory");

    /* Phase II: Start Telegram Client */
    let client = ClientActor::default().start();
    client
        .send(LoginCommand(LoginConfig {
            api_id: getenv!("TG_ID", usize),
            api_hash: getenv!("TG_HASH"),
            mobile_number: getenv!("TG_MOBILE_NUMBER"),
            session_path: SESSION_PATH,
        }))
        .await
        .expect("failed to login");

    /* Phase III: Initiate FwdModule  */
    let fwd_chat = Arc::new({
        // Resolve the chat ID from the environment variable `TG_FWD_TO`.
        //
        // For details, see the implementation of Handler<ResolveChatCommand> in ClientActor.
        let pack_chat = client
            .send(ResolveChatCommand(getenv!("TG_FWD_TO", i32)))
            .await
            .unwrap()
            .expect("failed to get the chat forward to");

        // Unpack the Chat object from the PackedChat.
        //
        // For details, see the implementation of Handler<UnpackChatCommand> in ClientActor.
        client
            .send(UnpackChatCommand(pack_chat))
            .await
            .unwrap()
            .expect("failed to unpack the chat")
    });

    // We initiate the FwdModule with the Chat object.
    let fwd_mod = 
        FwdModuleActor::activate_module(FwdModuleConfig {
            target: fwd_chat.clone(),
        });

    /* Phase IV: Initiate ClientModuleExecutor */
    let executor = ClientModuleExecutor {
        client: client.clone(),
        modules: Arc::new(vec![fwd_mod]),
    }
    .start();

    /* Phase V: Polling updates */
    while let Some(updates) = tokio::select! {
        _ = tokio::signal::ctrl_c() => Ok(Ok(None)),
        result = client.send(NextUpdatesCommand) => result,
    }
    .unwrap()
    .expect("failed to retrieve updates")
    {
        // Get the Update object by iterating UpdateIter.
        for update in updates {
            // Send request to ClientModuleExecutor, let it distribute Update to modules.
            let result = executor.send(ClientModuleMessage { update }).await.unwrap();

            // When the ClientModuleExecutor returns an error, we log it.
            if let Err(e) = result {
                error!("error in exectutors: {:?}", e);
            }
        }
    }
}
