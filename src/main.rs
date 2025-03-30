use crate::api::init_api;
use crate::vatsim::DatafeedSharedState;
use crate::vatsim::datafeed::DatafeedStatus;
use env_logger;
use vatsim::status::VatsimStatus;

mod api;
mod vatsim;

#[tokio::main]
async fn main() {
    env_logger::init();

    let shared_datafeed: DatafeedSharedState = DatafeedStatus::new().into();
    let datafeed_url = VatsimStatus::get_datafeed_url().await;

    tokio::task::spawn({
        let shared_datafeed = shared_datafeed.clone();
        async move {
            vatsim::datafeed::update_datafeed_loop(datafeed_url, shared_datafeed).await;
        }
    });

    let _ = init_api(shared_datafeed).await;
}
