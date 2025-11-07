use datafeed_cache_shared::datafeed::{
    DatafeedAtis, DatafeedController, DatafeedMilitaryRating, DatafeedPilot, DatafeedPilotRating,
    DatafeedServer,
};
use datafeed_cache_shared::response::{
    DatafeedGeneralResponse, DatafeedListResponse, DatafeedResponse,
};
use serde::de::DeserializeOwned;
use crate::DatafeedClient;

type Error = reqwest::Error;

impl DatafeedClient {

    fn make_request<T>(&self, path: &str) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let req = self.client.get(self.base_url.to_owned() + path).build()?;
        let response = self.client.execute(req)?;
        response.json::<T>()
    }

    pub fn get(&self) -> Result<DatafeedResponse, Error> {
        self.make_request("/datafeed")
    }

    pub fn get_general(&self) -> Result<DatafeedGeneralResponse, Error> {
        self.make_request("/datafeed/general")
    }

    pub fn get_controllers(&self) -> Result<DatafeedListResponse<DatafeedController>, Error> {
        self.make_request("/datafeed/controllers")
    }

    pub fn get_pilots(&self) -> Result<DatafeedListResponse<DatafeedPilot>, Error> {
        self.make_request("/datafeed/pilots")
    }

    pub fn get_atis(&self) -> Result<DatafeedListResponse<DatafeedAtis>, Error> {
        self.make_request("/datafeed/atis")
    }

    pub fn get_servers(&self) -> Result<DatafeedListResponse<DatafeedServer>, Error> {
        self.make_request("/datafeed/servers")
    }

    pub fn get_pilot_ratings(
        &self,
    ) -> Result<DatafeedListResponse<DatafeedPilotRating>, Error> {
        self.make_request("/datafeed/pilot_ratings")
    }

    pub fn get_military_ratings(
        &self,
    ) -> Result<DatafeedListResponse<DatafeedMilitaryRating>, Error> {
        self.make_request("/datafeed/military_ratings")
    }

    pub fn get_ger_controllers(
        &self,
    ) -> Result<DatafeedListResponse<DatafeedController>, Error> {
        self.make_request("/datafeed/controllers/ger")
    }

    pub fn get_ger_pilots(&self) -> Result<DatafeedListResponse<DatafeedPilot>, Error> {
        self.make_request("/datafeed/pilots/ger")
    }

    pub fn get_ger_atis(&self) -> Result<DatafeedListResponse<DatafeedAtis>, Error> {
        self.make_request("/datafeed/atis/ger")
    }
}
