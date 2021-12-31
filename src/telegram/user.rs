use std::io;

use grammers_client::{Client, Config};
use grammers_session::Session;
use log::{debug, info};

/// The login configuration.
pub struct LoginConfig {
    pub api_id: usize,
    pub api_hash: String,
    pub mobile_number: String,
    pub session_path: &'static str,
}

/// Login to Telegram with our own account.
pub async fn login(conf: LoginConfig) -> anyhow::Result<Client> {
    /* Phase 1: Connect to Telegram */
    info!("user::login(): 😶 Connecting to Telegram...");
    let config = Config {
        session: Session::load_file_or_create(conf.session_path)?,
        api_id: conf.api_id as i32,
        api_hash: conf.api_hash.clone(),
        params: Default::default(),
    };
    let mut client = Client::connect(config).await?;
    info!("user::login(): ✅ Connected to Telegram.");

    /* Phase 2: Authorize */
    if !client.is_authorized().await? {
        /* Phase 2-1: Request login code */
        info!("user::login(): 😶 Authorizing...");
        let token = client
            .request_login_code(&conf.mobile_number, conf.api_id as i32, &conf.api_hash)
            .await?;

        /* Phase 2-2: Prompt user to input their login code  */
        info!("user::login(): ⭐️ You would have gotten a login code. Copy that into here, then press Enter! ❤️");
        let mut login_code = String::new();
        io::stdin().read_line(&mut login_code)?;

        /* Phase 2-3: Authorize with the login code */
        info!("user::login(): ❤️ Perfect! Now authorizing with the login code...");
        client.sign_in(&token, &login_code).await?;
        info!("user::login(): ✅ Authorized successfully!");

        /* Phase 3: Store this loggin session. */
        client.session().save_to_file(&conf.session_path)?;
    } else {
        debug!("user::login(): ✅ Already authorized.");
    }

    debug!("user::login(): 👋 Welcome! You are now logged in.");
    Ok(client)
}
