#[path = "../iter6/mod.rs"]
mod iter6;
use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use iter6::{dbaccess, errors, handler, model, routes::{self, tutor_config}, state::AppState};
use routes::{app_config, course_config};
use sqlx::postgres::PgPool;
use std::env;
use tera::Tera;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    //Start HTTP Server
    let host_port = env::var("HOST_PORT").expect("HOST:PORT adress is not set in .env file!");
    println!("listening on {}", &host_port);
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file!");
    let db_pool = PgPool::connect(&database_url).await.unwrap();
    //Construct AppState
    let shared_data = web::Data::new(AppState { db: db_pool });

    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/iter6/**/*")).unwrap();

        App::new()
            .app_data(web::Data::new(tera))
            .app_data(shared_data.clone())
            .configure(course_config)
            .configure(app_config)
            .configure(tutor_config)
    })
    .bind(&host_port)?
    .run()
    .await
}
