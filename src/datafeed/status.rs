use rand::Rng;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct StatusData {
    v3: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct VatsimStatus {
    data: StatusData,
}

const VATSIM_STATUS_URL: &'static str = "https://status.vatsim.net/status.json";
const VATSIM_DEFAULT_DATAFEED_URL: &'static str = "https://data.vatsim.net/v3/vatsim-data.json";

impl VatsimStatus {
    /// Retrieves a datafeed url to use from VATSIM's status page.
    ///
    /// # Returns
    /// The selected datafeed url, or the default url in case there was an issue fetching the
    /// newest status.json file.
    pub(crate) async fn get_datafeed_url() -> String {
        let response = reqwest::get(VATSIM_STATUS_URL).await;
        let status: VatsimStatus = match response {
            Ok(res) => res.json::<VatsimStatus>().await.unwrap_or_default(),
            Err(_) => Self::default(),
        };

        Self::select_random_url(&status.data.v3)
    }

    /// Selects a random string from a vector of strings. This is to be done as per VATSIM's
    /// documentation in order to balance the load on the datafeed if this should become an issue
    /// in the future
    ///
    /// # Returns
    /// An owned version (cloned) of the selected String within the vector.
    fn select_random_url(urls: &Vec<String>) -> String {
        let idx = rand::rng().random_range(0..urls.len());
        urls[idx].clone()
    }
}

impl Default for VatsimStatus {
    fn default() -> Self {
        Self {
            data: StatusData {
                v3: vec![VATSIM_DEFAULT_DATAFEED_URL.into()],
            },
        }
    }
}
