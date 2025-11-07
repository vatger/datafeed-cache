use log::info;

#[cfg(feature = "blocking")]
pub mod client;

#[cfg(feature = "blocking")]
use reqwest::blocking::Client;

#[cfg(not(feature = "blocking"))]
pub mod client_async;

#[cfg(not(feature = "blocking"))]
use reqwest::Client;

pub mod shared {
    pub use datafeed_cache_shared::*;
}

pub struct DatafeedClient {
    client: Client,
    base_url: String,
}

const BASE_DEFAULT: &'static str = "https://df.vatsim-germany.org";

impl DatafeedClient {
    pub fn new() -> Self {
        let _ = env_logger::try_init();
        let _ = dotenv::dotenv();

        let base_url: String = dotenv::var("BASE_URL").unwrap_or(BASE_DEFAULT.to_string());
        info!("Selected BASE_URL: {}", base_url);

        DatafeedClient {
            client: Client::default(),
            base_url,
        }
    }
}