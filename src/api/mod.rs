use crate::api::state::ApiState;
use crate::datafeed::DatafeedSharedState;
use actix_web::{App, HttpServer, web};

mod handlers;
mod state;
pub(crate) mod types;

pub(crate) async fn init_api(shared_datafeed: DatafeedSharedState) -> std::io::Result<()> {
    HttpServer::new(move || {
        let shared_datafeed = shared_datafeed.clone();

        App::new()
            .app_data(web::Data::new(ApiState::new(shared_datafeed)))
            .service(
                web::scope("/datafeed")
                    .service(handlers::get_datafeed)
                    .service(handlers::get_general_datafeed)
                    .service(handlers::get_controllers_datafeed)
                    .service(handlers::get_pilots_datafeed)
                    .service(handlers::get_atis_datafeed)
                    .service(handlers::get_servers_datafeed)
                    .service(handlers::get_pilot_ratings_datafeed)
                    .service(handlers::get_mil_pilot_ratings_datafeed)
                    .service(handlers::get_ger_controllers_datafeed)
                    .service(handlers::get_ger_pilots_datafeed)
                    .service(handlers::get_ger_atis_datafeed),
            )
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
