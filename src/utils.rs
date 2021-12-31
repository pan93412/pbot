//! PBot: Utilities
//! 
//! This module contains some useful utilities, such as [`getenv`].

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
