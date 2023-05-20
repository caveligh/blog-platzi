#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

use dotenvy::dotenv;
use std::env;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use self::models::Post;
use self::schema::posts::dsl::*;
use actix_web::web::Data;
use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use diesel::r2d2::Pool;
use diesel::r2d2::{self, ConnectionManager};

#[get("/")]
async fn index(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("Problemas al traer la BD");
    
    match web::block(move || {posts.load::<Post>(&mut conn)}).await {
        Ok(data) => {
            //println!("{:?}", data);
            HttpResponse::Ok().body(format!("{:?}", data))
        },
        Err(err) => {
            HttpResponse::Ok().body(format!("Error al recibir la data: {}", err))
        }
    }
}

#[get("/{name}")]
async fn hello_world(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found");

    let connection = ConnectionManager::<PgConnection>::new(db_url);

    let pool = Pool::builder()
        .build(connection)
        .expect("No se pudo construir la Pool");

    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(hello_world)
            .app_data(Data::new(pool.clone()))
    })
    .bind(("127.0.0.1", 8090))
    .unwrap()
    .run()
    .await
}
