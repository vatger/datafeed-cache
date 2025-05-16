use crate::api::state::ApiState;
use crate::vatsim::DatafeedSharedState;
use actix_web::middleware::TrailingSlash::Trim;
use actix_web::{App, HttpServer, middleware, web};

mod handlers;
mod state;

pub(crate) async fn init_api(shared_datafeed: DatafeedSharedState) -> std::io::Result<()> {
    let api_state = web::Data::new(ApiState::new(shared_datafeed));

    HttpServer::new(move || {
        App::new()
            .app_data(api_state.clone())
            .wrap(middleware::NormalizePath::new(Trim))
            .wrap(middleware::Logger::new(
                "%a \"%r\" %s \"%{User-Agent}i\" %Ts",
            ))
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
            .service(handlers::get_health_check)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
