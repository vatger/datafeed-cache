use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DatafeedGeneral {
    pub version: u32,
    pub reload: u32,
    pub update_timestamp: DateTime<Utc>,
    pub connected_clients: u32,
    pub unique_users: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DatafeedPilotFlightPlan {
    pub flight_rules: String,
    pub aircraft_short: String,
    pub departure: String,
    pub arrival: String,
    pub alternate: String,
    pub cruise_tas: String,
    pub altitude: String,
    pub deptime: String,
    pub enroute_time: String,
    pub fuel_time: String,
    pub remarks: String,
    pub route: String,
    pub assigned_transponder: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DatafeedPilot {
    pub cid: u32,
    pub name: String,
    pub callsign: String,
    pub server: String,
    pub pilot_rating: i32,
    pub military_rating: i32,
    pub latitude: f32,
    pub longitude: f32,
    pub altitude: i32,
    pub groundspeed: i32,
    pub transponder: String,
    pub heading: u16,
    pub qnh_mb: i32,
    pub flight_plan: Option<DatafeedPilotFlightPlan>,
    pub logon_time: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DatafeedController {
    pub cid: u32,
    pub name: String,
    pub callsign: String,
    pub frequency: String,
    pub facility: i32,
    pub rating: i32,
    pub server: String,
    pub visual_range: u32,
    pub text_atis: Option<Vec<String>>,
    pub last_updated: DateTime<Utc>,
    pub logon_time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DatafeedServer {
    pub ident: String,
    pub hostname_or_ip: String,
    pub location: String,
    pub name: String,
    pub clients_connection_allowed: i32,
    pub client_connections_allowed: bool,
    pub is_sweatbox: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DatafeedFacility {
    pub id: i32,
    pub short: String,
    pub long: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DatafeedPilotRating {
    pub id: i32,
    pub short_name: String,
    pub long_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DatafeedPrefile {
    pub cid: u32,
    pub name: String,
    pub callsign: String,
    pub flight_plan: DatafeedPilotFlightPlan,
    pub last_updated: DateTime<Utc>,
}

pub type DatafeedAtis = DatafeedController;
pub type DatafeedRating = DatafeedFacility;
pub type DatafeedMilitaryRating = DatafeedPilotRating;
