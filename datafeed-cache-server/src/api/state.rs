use crate::vatsim::DatafeedSharedState;

pub(crate) type ApiStateData = actix_web::web::Data<ApiState>;

pub(crate) struct ApiState {
    pub(crate) shared_state: DatafeedSharedState,
}

impl ApiState {
    pub(crate) fn new(shared_state: DatafeedSharedState) -> Self {
        Self {
            shared_state,
        }
    }
}
