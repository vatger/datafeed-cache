use crate::datafeed::datafeed_status::DatafeedStatus;
use std::sync::Arc;
use tokio::sync::RwLock;

pub(crate) mod datafeed;
pub(crate) mod datafeed_status;
pub(crate) mod status;
pub(crate) mod types;

pub(crate) type DatafeedSharedState = Arc<RwLock<DatafeedStatus>>;

impl From<DatafeedStatus> for DatafeedSharedState {
    fn from(value: DatafeedStatus) -> Self {
        Arc::new(RwLock::new(value))
    }
}
