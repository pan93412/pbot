//! PBot: Utils
use grammers_client::types::Message;

/// Get the environment value.
#[macro_export]
macro_rules! getenv {
    ($envvar:expr) => {
        std::env::var($envvar).expect(concat!("should specify `", $envvar, "` in .env file"))
    };
    ($envvar:expr, $type:ty) => {
        getenv!($envvar)
            .parse::<$type>()
            .expect(concat!($envvar, " should be ", stringify!($type)))
    };
}


/// Get the sender's ID of this message.
/// 
/// If we can't get the sender's ID,
/// we returns `-1145141919810`.
fn get_sender(message: &Message) -> String {
    message
        .sender()
        .map(|c| c.id().to_string())
        .unwrap_or_else(|| "-1145141919810".to_string())
}

pub fn is_root_user(message: &Message) -> bool {
    get_sender(message) == getenv!("TG_ROOT_USER")
}
