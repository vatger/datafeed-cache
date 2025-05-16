use crate::vatsim::DatafeedSharedState;
use geo::{LineString, Polygon};

const GERMANY_POLY: &[(f64, f64)] = &[
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

pub(crate) type ApiStateData = actix_web::web::Data<ApiState>;

pub(crate) struct ApiState {
    pub(crate) shared_state: DatafeedSharedState,
    pub(crate) ger_poly: Polygon,
}

impl ApiState {
    pub(crate) fn new(shared_state: DatafeedSharedState) -> Self {
        Self {
            shared_state,
            ger_poly: Polygon::new(LineString::from(GERMANY_POLY.to_vec()), vec![]),
        }
    }
}
