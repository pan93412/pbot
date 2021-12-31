mod modules;
mod telegram;

use std::{env, sync::Arc};

use actix::Actor;
use dotenv::dotenv;
use modules::enabled_modules;
use simple_logger::SimpleLogger;
use telegram::{
    update::client_handler,
    user::{login, LoginConfig},
};

use crate::telegram::update::ClientModuleExecutor;

const SESSION_PATH: &str = "./telegram.session.dat";

macro_rules! getenv {
    ($envvar:expr) => {
        env::var($envvar).expect(concat!("should specify `", $envvar, "` in .env file"))
    };
    ($envvar:expr, $type:ty) => {
        getenv!($envvar)
            .parse::<$type>()
            .expect(concat!($envvar, " should be ", stringify!($type)))
    };
}

#[actix::main]
async fn main() {
    SimpleLogger::new()
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
