mod db;
mod restapi;
use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let pool = sqlx::mysql::MySqlPoolOptions::new()
        .connect(
            env::var("DATABASE_URL")
                .expect("DATABASE_URL environment variable must be set")
                .as_str(),
        )
        .await?;

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8001".to_string())
        .parse()
        .expect("PORT must be a valid number");

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(restapi::configure)
            .wrap(cors)
    })
    .bind((host.as_str(), port))?
    .run()
    .await?;

    return Ok(());
}
