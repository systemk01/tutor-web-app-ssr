use actix_web::{web, error, App, Error, HttpResponse, HttpServer, Result};
use serde::{Deserialize, Serialize};
use std::env;
use tera::Tera;


//store tera template in application state
async fn index(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let s = tmpl
        .render("form.html", &tera::Context::new())
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[derive(Serialize, Deserialize)]
pub struct Tutor {
    name: String,
}

async fn handle_post_tutor(
    tmpl: web::Data<tera::Tera>,
    params: web::Form<Tutor>,
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("name", &params.name);
    ctx.insert("text", "Welcome!");
    let s = tmpl
        .render("user.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on: 127.0.0.1:8080");
    HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/iter2/**/*")).unwrap();

        App::new()
            .app_data(web::Data::new(tera))
            .configure(app_config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/tutors").route(web::post().to(handle_post_tutor))),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::{header::CONTENT_TYPE, header::HeaderValue, StatusCode};
    use actix_web::web::Form;
    use actix_web::test::{self};
    use actix_web::dev::Service;

#[actix_rt::test]
async fn handle_post_tutor_unit_test() {
    let params = Form(Tutor{
        name: "Tooby".to_string(),
    });
    let tera = Tera::new(concat!(
        env!("CARGO_MANIFEST_DIR"), 
        "/static/iter2/**/*")).unwrap();
    let webdata_tera = web::Data::new(tera);
    let resp = handle_post_tutor(webdata_tera, params).await.unwrap();
    assert_eq!(resp.status(),StatusCode::OK);
    assert_eq!(
        resp.headers().get(CONTENT_TYPE).unwrap(),
        HeaderValue::from_static("text/html")
    );
}

//Integration test case
#[actix_rt::test]
async fn handle_post_tutor_integration_test() {
    let tera = Tera::new(concat!(
        env!("CARGO_MANIFEST_DIR"), 
        "/static/iter2/**/*")).unwrap();
        let app = test::init_service(App::new().app_data(web::Data::new(tera)).configure(app_config)).await;
        let req = test::TestRequest::post()
        .uri("/tutors")
        .set_form(&Tutor {
            name: "Terry".to_string(),
        })
        .to_request();
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(
        resp.headers().get(CONTENT_TYPE).unwrap(),
        HeaderValue::from_static("text/html")
    );
}

}