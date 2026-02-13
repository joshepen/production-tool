mod db;
mod restapi;
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

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(restapi::configure)
    })
    .bind(("127.0.0.1", 8001))?
    .run()
    .await?;

    return Ok(());
}
