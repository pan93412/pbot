use grammers_client::types::Chat::User;
use grammers_client::{types::Message, Client, Config, SignInError};
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
        let login_code = rpassword::prompt_password_stderr("Login Code: ").unwrap();

        /* Phase 2-3: Authorize with the login code */
        info!("user::login(): ❤️  Perfect! Now authorizing with the login code...");
        let status = client.sign_in(&token, &login_code).await;

        /* Phase 2-4: Check if the authorization is successful */
        match status {
            Err(SignInError::PasswordRequired(password_token)) => {
                /* Phase 2-4-1 [PwdRequried]: Let user input their password. */
                info!("user::login(): ⚠️  You need to enter your password to authorize. Type your password to here, then press Enter!");
                info!(
                    "user::login(): hint: {}",
                    password_token
                        .hint()
                        .map(|v| v.as_str())
                        .unwrap_or_else(|| "None")
                );

                let password = rpassword::prompt_password_stderr("Password: ").unwrap();

                info!("user::login(): 😶 Checking password...");
                client
                    .check_password(password_token, password.trim())
                    .await
                    .expect("error singing in (2FA)");
            }
            Err(e) => {
                panic!("error signing in: {:?}", e);
            }
            _ => {}
        };

        info!("user::login(): ✅ Authorized successfully!");

        /* Phase 3: Store this loggin session. */
        client.session().save_to_file(&conf.session_path)?;
    } else {
        debug!("user::login(): ✅ Already authorized.");
    }

    debug!("user::login(): 👋 Welcome! You are now logged in.");
    Ok(client)
}

pub fn is_root_user(message: &Message) -> bool {
    message
        .sender()
        .map(|c| if let User(u) = c { u.is_self() } else { false })
        .unwrap_or(false)
}
