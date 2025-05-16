use crate::vatsim::datafeed::DatafeedStatus;
use std::sync::Arc;
use tokio::sync::RwLock;

pub(crate) mod datafeed;
pub(crate) mod status;

pub(crate) type DatafeedSharedState = Arc<RwLock<DatafeedStatus>>;

impl From<DatafeedStatus> for DatafeedSharedState {
    fn from(value: DatafeedStatus) -> Self {
        Arc::new(RwLock::new(value))
    }
}
