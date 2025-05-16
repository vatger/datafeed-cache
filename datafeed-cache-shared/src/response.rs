use crate::datafeed::{Datafeed, DatafeedGeneral};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Serialize, Deserialize, Debug)]
pub struct DatafeedGeneralResponse<'a> {
    pub data: Option<Cow<'a, DatafeedGeneral>>,
    pub controller_length: usize,
    pub pilots_length: usize,
    pub atis_length: usize,
    pub failed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DatafeedResponse<'a> {
    pub data: Cow<'a, Option<Datafeed>>,
    pub failed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DatafeedListResponse<'a, T>
where
    T: Clone,
{
    pub data: Cow<'a, [T]>,
    pub length: usize,
    pub failed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DatafeedGerListResponse<'a, T>
where
    T: Clone,
{
    pub data: Cow<'a, Vec<T>>,
    pub length: usize,
    pub failed: bool,
}
