pub mod client;
pub mod config;
pub mod routes;
pub mod state;
pub mod view_model;
pub mod views;

use actix_web::{App, HttpServer, web};
use client::BackendClient;
use config::AppConfig;
use state::AppState;

pub async fn run() -> std::io::Result<()> {
    if let Err(err) = dfps_configuration::load_env("app.web.frontend") {
        eprintln!("warning: frontend env file not loaded: {err}");
    }
    let config = AppConfig::from_env().map_err(|err| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("frontend config error: {err}"),
        )
    })?;
    let client = BackendClient::from_config(&config).map_err(|err| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("failed to create backend client: {err}"),
        )
    })?;
    let state = AppState::new(config.clone(), client);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .configure(routes::configure)
    })
    .bind(&config.listen_addr)?
    .run()
    .await
}
