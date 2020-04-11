#[macro_use]
extern crate diesel;

use actix_web::{middleware, post, web, App, Error, HttpResponse, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenv::dotenv;
use schema::stats;
use std::env;

mod schema;

type DbPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Insertable)]
#[table_name = "stats"]
pub struct Stats {
    pub data: serde_json::Value,
}

#[post("/")]
async fn handler(
    pool: web::Data<DbPool>,
    body: web::Json<serde_json::Value>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Couldn't get db connection from pool");

    web::block::<_, _, diesel::result::Error>(move || {
        diesel::insert_into(schema::stats::dsl::stats)
            .values(Stats {
                data: body.into_inner(),
            })
            .execute(&conn)?;
        Ok(())
    })
    .await?;

    Ok(HttpResponse::Created().finish())
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool: DbPool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let port = env::var("PORT")
        .unwrap_or("4000".to_string())
        .parse()
        .expect("PORT must be a number");

    let bind = ("0.0.0.0", port);
    println!("Starting server at: {}:{}", bind.0, bind.1);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .service(handler)
    })
    .bind(bind)?
    .run()
    .await
}
