#[warn(missing_docs)]
mod modules;
mod telegram;
mod utils;

use std::sync::Arc;

use actix::Actor;
use dotenv::dotenv;
use log::error;
use modules::{
    base::ModuleActivator,
    fwd::{FwdModuleActor, FwdModuleConfig},
};
use simple_logger::SimpleLogger;
use telegram::{
    client::{
        commands::{LoginCommand, ResolveChatCommand, UnpackChatCommand},
        ClientActor,
    },
    update::{ClientModuleExecutor, ClientModuleMessage},
    user::LoginConfig,
};

use crate::telegram::client::commands::NextUpdatesCommand;

const SESSION_PATH: &str = "./.telegram.session.dat";

#[actix::main]
async fn main() {
    SimpleLogger::new()
        .with_utc_timestamps()
        .with_level(log::LevelFilter::Info)
        .init()
        .expect("failed to configure logger");
    dotenv().expect("a .env file should be existed in the current working directory");

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

    let fwd_mod = {
        let pack_chat = client
            .send(ResolveChatCommand(getenv!("TG_FWD_TO", i32)))
            .await
            .unwrap()
            .expect("failed to get the chat forward to");
        let fwd_chat = client
            .send(UnpackChatCommand(pack_chat))
            .await
            .unwrap()
            .expect("failed to unpack the chat");

        FwdModuleActor::activate_module(FwdModuleConfig {
            target: Arc::new(fwd_chat),
        })
    };
    // let getinfo_mod = GetInfoModuleActor::activate_module(());

    let executor = ClientModuleExecutor {
        client: client.clone(),
        modules: vec![fwd_mod],
    }
    .start();

    while let Some(updates) = tokio::select! {
        _ = tokio::signal::ctrl_c() => Ok(Ok(None)),
        result = client.send(NextUpdatesCommand) => result,
    }
    .unwrap()
    .expect("failed to retrieve updates")
    {
        for update in updates {
            let result = executor
                .send(ClientModuleMessage { update })
                .await
                .expect("Mailbox is full.");

            if let Err(e) = result {
                error!("error in exectutors: {:?}", e);
            }
        }
    }
}
