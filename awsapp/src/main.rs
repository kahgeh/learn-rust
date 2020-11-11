mod apps;
mod aws_app_context;
use actix_web::web::scope;
use actix_web::{App, HttpResponse, HttpServer, Responder, web::{Json, Path}, get,put};
use crate::apps::Application;
use crate::aws_app_context::APPCONTEXT;

#[get("/apps")]
async fn get_apps() -> impl Responder {
    let context = APPCONTEXT.read().unwrap();
    HttpResponse::Ok().json(context.get_apps())
}

#[put("/apps/{app_id}")]
async fn save_app(body:Json<Application>, _:Path<String>) -> impl Responder {
    let context = APPCONTEXT.write().unwrap();
    let app = body.into_inner();
    context.save_app(&app);
    
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let context=APPCONTEXT.read().unwrap();

    let init_result = context.init().await;
    match init_result {
        Ok(())=> println!("initialised"),
        Err(e)=> println!("error initialising {:?}", e)
    }

    HttpServer::new(|| 
            App::new()
                .service(scope("/v1")
                        .service(get_apps)
                        .service(save_app)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
