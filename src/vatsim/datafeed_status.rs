use crate::vatsim::datafeed::Datafeed;
use serde::{Deserialize, Serialize};

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
