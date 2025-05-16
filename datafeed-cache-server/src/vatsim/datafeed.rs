use crate::vatsim::DatafeedSharedState;
use chrono::{TimeDelta, Utc};
use datafeed_cache_shared::datafeed::Datafeed;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::ops::Sub;

const UPDATE_DATAFEED_INTERVAL_SECS: u64 = 15;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct DatafeedStatus {
    pub data: Option<Datafeed>,
    pub failed: bool,
}

impl DatafeedStatus {
    pub(crate) fn new() -> Self {
        Self {
            data: None,
            failed: true,
        }
    }
}

pub(crate) trait DatafeedExt {
    async fn download_from_url(url: &str) -> Result<Datafeed, reqwest::Error>;
    fn is_failed(&self, same_timestamp_count: &mut u32, previous: &Option<Datafeed>) -> bool;
}

impl DatafeedExt for Datafeed {
    async fn download_from_url(url: &str) -> Result<Datafeed, reqwest::Error> {
        reqwest::get(url).await?.json().await
    }

    fn is_failed(&self, same_timestamp_count: &mut u32, previous: &Option<Datafeed>) -> bool {
        match previous {
            Some(previous) => {
                if self.general.update_timestamp == previous.general.update_timestamp {
                    *same_timestamp_count += 1;
                    info!("Same timestamp count: {}", same_timestamp_count);
                } else {
                    *same_timestamp_count = 0;
                }

                let now = Utc::now().sub(TimeDelta::minutes(2));
                if self.pilots.len() == 0
                    || self.pilots.len().abs_diff(previous.pilots.len()) > 500
                    || *same_timestamp_count > 5
                    || (*same_timestamp_count > 0 && previous.general.update_timestamp < now)
                {
                    return true;
                }

                false
            }
            None => false,
        }
    }
}

pub(crate) async fn update_datafeed_loop(datafeed_url: String, shared_state: DatafeedSharedState) {
    let mut same_timestamp_count: u32 = 0;
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(
        UPDATE_DATAFEED_INTERVAL_SECS,
    ));

    loop {
        interval.tick().await;

        match Datafeed::download_from_url(datafeed_url.as_str()).await {
            Ok(datafeed) => {
                let mut write_guard = shared_state.write().await;
                if datafeed.is_failed(&mut same_timestamp_count, &write_guard.data) {
                    error!("Failed to update vatsim");
                    write_guard.failed = true;
                    continue;
                }

                info!("{:?}", datafeed.general);
                write_guard.data = Some(datafeed);
                write_guard.failed = false;
            }
            Err(err) => {
                error!("Failed to download or parse response: {:?}", err.source());
                let mut write_guard = shared_state.write().await;
                write_guard.failed = true;
            }
        }
    }
}
