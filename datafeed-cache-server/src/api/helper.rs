use geo::{Contains, Coord, LineString, Polygon};
use once_cell::sync::Lazy;
use datafeed_cache_shared::datafeed::{Datafeed, DatafeedController, DatafeedPilot};

const GERMANY_POLY_DEF: &[(f64, f64)] = &[
    (47.610078, 7.476857),
    (48.97135, 8.189489),
    (49.466566, 6.380295),
    (51.028373, 5.932113),
    (54.84348, 7.145719),
    (54.642252, 14.133224),
    (50.843505, 14.750028),
    (50.249433, 12.140394),
    (48.675772, 13.854829),
    (47.475928, 12.926649),
];

pub(crate) static GERMANY_POLY: Lazy<Polygon> = Lazy::new(|| {
    Polygon::new(LineString::from(GERMANY_POLY_DEF.to_vec()), vec![])
});

pub(crate) fn get_ger_pilots(df: &Option<Datafeed>) -> Vec<DatafeedPilot> {
    match df {
        Some(df) => {
            df.pilots.iter()
                .filter(|pilot| {
                    let coord: Coord<f64> =
                        Coord::from((pilot.latitude.into(), pilot.longitude.into()));
                    GERMANY_POLY.contains(&coord)
                })
                .cloned()
                .collect()
        }
        None => {vec![]}
    }
}

pub(crate) fn get_ger_controllers(df: &Option<Datafeed>) -> Vec<DatafeedController> {
    match df {
        Some(df) => {
            df.controllers.iter()
                .filter(|controller| {
                    (controller.callsign.starts_with("ED") || controller.callsign.starts_with("ET"))
                    && controller.frequency != "199.998"
                })
                .cloned()
                .collect()
        }
        None => {vec![]}
    }
}