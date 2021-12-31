mod modules;
mod telegram;
mod utils;

use std::sync::Arc;

use actix::Actor;
use dotenv::dotenv;
use modules::{fwd::{FwdModuleActor, FwdModuleConfig}, base::ModuleActivator};
use simple_logger::SimpleLogger;
use telegram::{
    update::client_handler,
    user::{login, LoginConfig}, chat::resolve_chat,
};

use crate::telegram::update::ClientModuleExecutor;

const SESSION_PATH: &str = "./.telegram.session.dat";

#[actix::main]
async fn main() {
    SimpleLogger::new()
        .with_utc_timestamps()
        .with_level(log::LevelFilter::Info)
        .init()
        .expect("failed to configure logger");
    dotenv().expect("a .env file should be existed in the current working directory");

    let login_config = LoginConfig {
        api_id: getenv!("TG_ID", usize),
        api_hash: getenv!("TG_HASH"),
        mobile_number: getenv!("TG_MOBILE_NUMBER"),
        session_path: SESSION_PATH,
    };

    let mut client = login(login_config).await.expect("failed to login");

    let fwd_chat = client.unpack_chat(&resolve_chat(&client, getenv!("TG_FWD_TO", i32)).await.expect("failed to get the chat forward to")).await.expect("failed to unpack the chat");
    let fwd_mod = FwdModuleActor::activate_module(FwdModuleConfig {
        target: Arc::new(fwd_chat)
    });

    let client = Arc::new(client);

    let executor = ClientModuleExecutor {
        client: client.clone(),
        modules: vec![fwd_mod]
    };

    let executor_recipient = executor.start().recipient();
    client_handler(&client, executor_recipient)
        .await
        .expect("failed to handle updates");
}
