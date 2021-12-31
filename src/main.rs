mod modules;
mod telegram;
mod utils;

use std::sync::Arc;

use actix::Actor;
use dotenv::dotenv;
use modules::enabled_modules;
use simple_logger::SimpleLogger;
use telegram::{
    update::client_handler,
    user::{login, LoginConfig},
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

    let client = Arc::new(login(login_config).await.expect("failed to login"));

    let executor = ClientModuleExecutor {
        client: client.clone(),
        modules: enabled_modules(),
    };

    let executor_recipient = executor.start().recipient();
    client_handler(client.clone(), executor_recipient)
        .await
        .expect("failed to handle updates");
}
