use crate::api::state::ApiStateData;
use crate::api::types::{
    DatafeedGeneralResponse, DatafeedGerListResponse, DatafeedListResponse, DatafeedResponse,
};
use crate::vatsim::types::{
    DatafeedAtis, DatafeedController, DatafeedMilitaryRating, DatafeedPilot, DatafeedPilotRating,
    DatafeedServer,
};
use actix_web::HttpResponse;
use chrono::Utc;
use geo::{Contains, Coord};
use serde::Serialize;

#[actix_web::get("")]
async fn get_datafeed(data: ApiStateData) -> HttpResponse {
    let read_lock = data.shared_state.read().await;
    let status = &*read_lock;

    HttpResponse::Ok().json(DatafeedResponse {
        data: &status.data,
        failed: status.failed,
    })
}

#[actix_web::get("/general")]
async fn get_general_datafeed(data: ApiStateData) -> HttpResponse {
    let read_lock = data.shared_state.read().await;
    let status = &*read_lock;

    let datafeed = status.data.as_ref();

    HttpResponse::Ok().json(DatafeedGeneralResponse {
        data: datafeed.map_or(None, |df| Some(&df.general)),
        controller_length: datafeed.map_or(0, |df| df.controllers.len()),
        pilots_length: datafeed.map_or(0, |df| df.pilots.len()),
        atis_length: datafeed.map_or(0, |df| df.atis.len()),
        failed: status.failed,
    })
}

#[actix_web::get("/controllers")]
async fn get_controllers_datafeed(data: ApiStateData) -> HttpResponse {
    let read_lock = data.shared_state.read().await;
    let status = &*read_lock;

    let controllers: &[DatafeedController] = status
        .data
        .as_ref()
        .map_or(&[], |df| df.controllers.as_slice());

    HttpResponse::Ok().json(DatafeedListResponse {
        data: controllers,
        length: controllers.len(),
        failed: status.failed,
    })
}

#[actix_web::get("/pilots")]
async fn get_pilots_datafeed(data: ApiStateData) -> HttpResponse {
    let read_lock = data.shared_state.read().await;
    let status = &*read_lock;

    let pilots: &[DatafeedPilot] = status.data.as_ref().map_or(&[], |df| df.pilots.as_slice());

    HttpResponse::Ok().json(DatafeedListResponse {
        data: pilots,
        length: pilots.len(),
        failed: status.failed,
    })
}

#[actix_web::get("/atis")]
async fn get_atis_datafeed(data: ApiStateData) -> HttpResponse {
    let read_lock = data.shared_state.read().await;
    let status = &*read_lock;

    let atis_slice: &[DatafeedAtis] = status.data.as_ref().map_or(&[], |df| df.atis.as_slice());

    HttpResponse::Ok().json(DatafeedListResponse {
        data: atis_slice,
        length: atis_slice.len(),
        failed: status.failed,
    })
}

#[actix_web::get("/server")]
async fn get_servers_datafeed(data: ApiStateData) -> HttpResponse {
    let read_lock = data.shared_state.read().await;
    let status = &*read_lock;

    let servers: &[DatafeedServer] = status.data.as_ref().map_or(&[], |df| df.servers.as_slice());

    HttpResponse::Ok().json(DatafeedListResponse {
        data: servers,
        length: servers.len(),
        failed: status.failed,
    })
}

#[actix_web::get("/pilot_ratings")]
async fn get_pilot_ratings_datafeed(data: ApiStateData) -> HttpResponse {
    let read_lock = data.shared_state.read().await;
    let status = &*read_lock;

    let pilot_ratings: &[DatafeedPilotRating] = status
        .data
        .as_ref()
        .map_or(&[], |df| df.pilot_ratings.as_slice());

    HttpResponse::Ok().json(DatafeedListResponse {
        data: pilot_ratings,
        length: pilot_ratings.len(),
        failed: status.failed,
    })
}

#[actix_web::get("/military_ratings")]
async fn get_mil_pilot_ratings_datafeed(data: ApiStateData) -> HttpResponse {
    let read_lock = data.shared_state.read().await;
    let status = &*read_lock;

    let military_ratings: &[DatafeedMilitaryRating] = status
        .data
        .as_ref()
        .map_or(&[], |df| df.military_ratings.as_slice());

    HttpResponse::Ok().json(DatafeedListResponse {
        data: military_ratings,
        length: military_ratings.len(),
        failed: status.failed,
    })
}

///
/// Below contains the handlers for the endpoints specific to VATSIM-Germany.
///
/// /vatsim/ger/[...]
///

#[actix_web::get("/controllers/ger")]
async fn get_ger_controllers_datafeed(data: ApiStateData) -> HttpResponse {
    let read_lock = data.shared_state.read().await;
    let status = &*read_lock;

    let controllers = status.data.as_ref().map_or(Vec::new(), |df| {
        df.controllers
            .iter()
            .filter(|controller| {
                (controller.callsign.starts_with("ED") || controller.callsign.starts_with("ET"))
                    && controller.frequency != "199.998"
            })
            .cloned()
            .collect()
    });

    HttpResponse::Ok().json(DatafeedGerListResponse {
        data: &controllers,
        length: controllers.len(),
        failed: status.failed,
    })
}

#[actix_web::get("/pilots/ger")]
async fn get_ger_pilots_datafeed(data: ApiStateData) -> HttpResponse {
    let read_lock = data.shared_state.read().await;
    let status = &*read_lock;

    let pilots = status.data.as_ref().map_or(Vec::new(), |df| {
        df.pilots
            .iter()
            .filter(|pilot| {
                let coord: Coord<f64> =
                    Coord::from((pilot.latitude.into(), pilot.longitude.into()));
                data.ger_poly.contains(&coord)
            })
            .cloned()
            .collect()
    });

    HttpResponse::Ok().json(DatafeedGerListResponse {
        data: &pilots,
        length: pilots.len(),
        failed: status.failed,
    })
}

#[actix_web::get("/pilots/ger")]
async fn get_ger_atis_datafeed(data: ApiStateData) -> HttpResponse {
    let read_lock = data.shared_state.read().await;
    let status = &*read_lock;

    let atis = status.data.as_ref().map_or(Vec::new(), |df| {
        df.atis
            .iter()
            .filter(|atis| {
                (atis.callsign.starts_with("ED") || atis.callsign.starts_with("ET"))
                    && atis.frequency != "199.998"
            })
            .cloned()
            .collect()
    });

    HttpResponse::Ok().json(DatafeedGerListResponse {
        data: &atis,
        length: atis.len(),
        failed: status.failed,
    })
}

///
/// Health-Check endpoint
///
/// /health-check
///

#[actix_web::get("/health-check")]
async fn get_health_check() -> HttpResponse {
    #[derive(Serialize)]
    struct Response {
        build_hash: String,
        timestamp: i64,
    }

    HttpResponse::Ok().json(Response {
        build_hash: std::env::var("COMMIT_SHA").unwrap_or("-------".into()),
        timestamp: Utc::now().timestamp(),
    })
}
