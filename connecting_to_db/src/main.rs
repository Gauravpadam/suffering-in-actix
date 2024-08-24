use actix_web::{web, middleware::Logger, App, HttpResponse, HttpServer, Responder};
use sea_orm::DatabaseConnection;
use sea_orm::Database;

mod utils;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    dotenv::dotenv().ok();
    env_logger::init();

    let port: u16 = (*utils::constants::PORT).clone();
    let address: String = (*utils::constants::ADDRESS).clone();
    let database_url: String = (*utils::constants::DATABASE_URL).clone();

    // This one line, connects you with the db via seaORM
    let db: DatabaseConnection = Database::connect(database_url).await.unwrap();

    HttpServer::new(|| {
        App::new()
        .wrap(Logger::default())
        .configure(routes::home_routes::config)
    })
    .bind((address, port))?
    .run()
    .await
}
