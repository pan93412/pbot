use actix::prelude::*;

use dotenv::dotenv;
use log::info;
use simple_logger::SimpleLogger;

use pbot::getenv;
use pbot::SESSION_PATH;

use std::sync::Arc;

use pbot::telegram::{
    client::{
        commands::{LoginCommand, NextUpdatesCommand},
        ClientActor,
    },
    update::{ClientModuleExecutor, ClientModuleMessage},
    user::LoginConfig,
};

#[cfg(feature = "fwdmod")]
async fn activate_fwd_mod(client: &Addr<ClientActor>) -> pbot::modules::base::ActivatedModuleInfo {
    use pbot::modules::{base::ModuleActivator, fwd::FwdModuleActor};
    use pbot::telegram::client::commands::{ResolveChatCommand, UnpackChatCommand};

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
    let fwd_chat = client
        .send(UnpackChatCommand(pack_chat))
        .await
        .unwrap()
        .expect("failed to unpack the chat");

    // We initiate the FwdModule with the Chat object.
    FwdModuleActor {
        target: Arc::new(fwd_chat),
    }
    .activate_module()
}

#[actix::main]
async fn main() {
    /* Phase I: Initiate loggers and dotenv */
    info!("Configurating loggers and dotenv...");
    SimpleLogger::new()
        .with_utc_timestamps()
        .with_level(log::LevelFilter::Info)
        .init()
        .expect("failed to configure logger");
    dotenv().expect("a .env file should be existed in the current working directory");

    /* Phase II: Start Telegram Client */
    info!("Starting Telegram client...");
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

    /* Phase III: Initiate Modules */
    info!("Initiating modules...");
    let mut modules = Vec::new();
    // Initiate FwdModule
    #[cfg(feature = "fwdmod")]
    {
        info!("  → Enabled: FwdModule");
        modules.push(activate_fwd_mod(&client).await);
    }
    // Initiate GetInfoModule
    #[cfg(feature = "getinfomod")]
    {
        info!("  → Enabled: GetInfoModule");
        use pbot::modules::base::ModuleActivator;
        modules.push(pbot::modules::getinfo::GetInfoModuleActor.activate_module());
    }
    #[cfg(feature = "addrankmod")]
    {
        info!("  → Enabled: AddRankModule");
        use pbot::modules::base::ModuleActivator;
        modules.push(pbot::modules::addrank::AddRankModuleActor.activate_module());
    }

    /* Phase IV: Initiate ClientModuleExecutor */
    info!("Initiating ClientModuleExecutor...");
    let executor = ClientModuleExecutor {
        client: client.clone(),
        modules: Arc::new(modules),
    }
    .start();

    /* Phase V: Polling updates */
    info!("Polling updates...");
    while let Some(updates) = tokio::select! {
        _ = tokio::signal::ctrl_c() => Ok(Ok(None)),
        result = client.send(NextUpdatesCommand) => result,
    }
    .unwrap()
    .expect("failed to retrieve updates")
    {
        // Join all the .send() futures.
        futures::future::join_all(
            updates
                .into_iter()
                // Send request to ClientModuleExecutor, let it distribute Update to modules.
                .map(|update| executor.send(ClientModuleMessage { update })),
        )
        .await;
    }
}
