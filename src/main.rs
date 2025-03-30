use crate::api::init_api;
use crate::datafeed::DatafeedSharedState;
use crate::datafeed::datafeed_status::DatafeedStatus;
use datafeed::status::VatsimStatus;
use env_logger;

mod api;
mod datafeed;

#[tokio::main]
async fn main() {
    env_logger::init();

    let shared_datafeed: DatafeedSharedState = DatafeedStatus::new().into();
    let datafeed_url = VatsimStatus::get_datafeed_url().await;

    tokio::task::spawn({
        let shared_datafeed = shared_datafeed.clone();
        async move {
            datafeed::datafeed::update_datafeed_loop(datafeed_url, shared_datafeed).await;
        }
    });

    let _ = init_api(shared_datafeed).await;
}
