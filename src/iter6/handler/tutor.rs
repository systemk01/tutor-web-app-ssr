use actix_files as fs;
use actix_web::{App, Error, HttpResponse, HttpServer, Result, error, web};
use awc::Client;
use serde::{Deserialize, Serialize};
use tera::Tera;
use crate::iter6::model::Tutor;


pub async fn handle_get_tutors(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let client = Client::new();
    let response = client
        .get("http://localhost:3000/tutors/")
        .send()
        .await
        .unwrap()
        .body()
        .await
        .unwrap();

    let str_list = std::str::from_utf8(response.as_ref()).unwrap();
    println!("{:?}", &str_list);
    let tutor_list: Vec<Tutor> = serde_json::from_str(str_list).unwrap();
    let mut ctx = tera::Context::new();
    ctx.insert("tutors", &tutor_list);
    let rendered_html = tmpl
        .render("tutorlist.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered_html))
}