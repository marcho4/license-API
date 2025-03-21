mod api;
mod db;
mod models;

use actix_web::{web, App, HttpServer, middleware};
use crate::api::license_api::*;
use crate::db::db::DbRepo;
use tokio::sync::Mutex;
use env_logger::Env;
use actix_cors::Cors;



pub struct AppState {
    pub db: Mutex<DbRepo>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Init DB repo
    let db = DbRepo::new().await;

    // Creating State with DB repo
    let state = web::Data::new(AppState{ db: Mutex::new(db) }) ;

    // Initializing Logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Start API
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::default()
                .allow_any_method()
                .allow_any_header()
                .allowed_origin("http://localhost:3000")
                .supports_credentials()
            )
            .service(

                web::scope("/license")
                    .service(get_license)
                    .service(add_license)
                    .service(renew_license)
                    .service(delete_license)
                    .service(activate)
                    .service(get_all_licenses)
                    .service(get_all_comm_licenses)
            )
    })
        .bind(("0.0.0.0", 8001))?
        .run()
        .await
}