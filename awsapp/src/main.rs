mod apps;
mod aws_app_context;
use crate::aws_app_context::AwsAppContext;
use actix_session::Session;
use actix_web::http::StatusCode;
use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Result};


#[get("/apps")]
async fn get_apps(_: Session, _: HttpRequest) -> Result<HttpResponse> {
    let context = AwsAppContext::get();
    let x = &context;
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("application/json; charset=utf-8")
        .json(x.get_apps()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    AwsAppContext::init();
    HttpServer::new(|| App::new().service(get_apps))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
