mod errors;
mod handler;
mod model;
mod routes;
mod state;
mod store;

use state::AppState;
use tera::Tera;

use std::env;

use sqlx::postgres::PgPool;

use actix_web::{web, App, HttpServer};

use routes::{app_config, course_config};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let pg_pool = PgPool::connect(&database_url).await.unwrap();
    let shared_data = web::Data::new(AppState { pg_pool });

    let host_port = env::var("HOST_PORT").expect("HOST:PORT address is not set in .env file");
    println!("Serving on: {}", host_port);

    HttpServer::new(move || {
        let tera = Tera::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/static/templates/**/*"
        ))
        .unwrap();
        App::new()
            .app_data(web::Data::new(tera))
            .app_data(shared_data.clone())
            .configure(course_config)
            .configure(app_config)
    })
    .bind(&host_port)?
    .run()
    .await?;

    Ok(())
}
