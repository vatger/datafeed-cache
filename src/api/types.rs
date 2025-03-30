use crate::datafeed::datafeed::Datafeed;
use crate::datafeed::types::DatafeedGeneral;
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct DatafeedResponse<'a> {
    pub(crate) data: &'a Option<Datafeed>,
    pub(crate) failed: bool,
}

#[derive(Serialize)]
pub(crate) struct DatafeedGeneralResponse<'a> {
    pub(crate) data: Option<&'a DatafeedGeneral>,
    pub(crate) controller_length: usize,
    pub(crate) pilots_length: usize,
    pub(crate) atis_length: usize,
    pub(crate) failed: bool,
}

#[derive(Serialize)]
pub(crate) struct DatafeedListResponse<'a, T> {
    pub(crate) data: &'a [T],
    pub(crate) length: usize,
    pub(crate) failed: bool,
}

#[derive(Serialize)]
pub(crate) struct DatafeedGerListResponse<'a, T> {
    pub(crate) data: &'a Vec<T>,
    pub(crate) length: usize,
    pub(crate) failed: bool,
}
