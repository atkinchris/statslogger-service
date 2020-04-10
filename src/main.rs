use actix_web::{middleware, post, web, App, HttpResponse, HttpServer, Responder};
use std::env;

#[post("/")]
async fn handler(body: web::Json<serde_json::Value>) -> impl Responder {
    println!("{:#?}", body.into_inner());
    HttpResponse::Ok()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT")
        .unwrap_or("4000".to_string())
        .parse()
        .expect("PORT must be a number");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .service(handler)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
