mod api;
mod auth;
mod config;
mod error;
mod models;
mod products;
mod utils;

pub use api::ApiClient;
pub use auth::Auth;
pub use config::Config;
pub use error::{Error, Result};
pub use models::*;
pub use products::*;
pub use utils::*;

pub mod prelude {
    pub use crate::api::ApiClient;
    pub use crate::auth::Auth;
    pub use crate::config::Config;
    pub use crate::error::{Error, Result};
    pub use crate::products::*;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
