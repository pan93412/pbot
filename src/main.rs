mod telegram;
mod modules;

use std::env;

use dotenv::dotenv;
use modules::enabled_modules;
use simple_logger::SimpleLogger;
use telegram::{user::{LoginConfig, login}, update::client_handler};

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

#[tokio::main]
async fn main() {
    SimpleLogger::new().init().expect("failed to configure logger");
    dotenv().expect("a .env file should be existed in the current working directory");

    let login_config = LoginConfig {
        api_id: getenv!("TG_ID", usize),
        api_hash: getenv!("TG_HASH"),
        mobile_number: getenv!("TG_MOBILE_NUMBER"),
        session_path: SESSION_PATH
    };

    let client = login(login_config).await.expect("failed to login");
    client_handler(client, enabled_modules()).await.expect("failed to handle updates");
}
