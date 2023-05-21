#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use tera::Tera;

use self::models::{NewPostHandler, Post};
use self::schema::posts::dsl::*;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use diesel::r2d2::Pool;
use diesel::r2d2::{self, ConnectionManager};

#[get("/prueba_tera")]
async fn tera_test(template_manager: web::Data<tera::Tera>) -> impl Responder {
    let ctx = tera::Context::new();
    //ctx.insert("name", "Tera");

    return HttpResponse::Ok()
        .content_type("text/html")
        .body(template_manager.render("index.html", &ctx).unwrap());
}

#[get("/")]
async fn index(pool: web::Data<DbPool>, template_manager: web::Data<tera::Tera>) -> impl Responder {
    let mut conn = pool.get().expect("Problemas al traer la BD");

    match web::block(move || posts.load::<Post>(&mut conn)).await {
        Ok(data) => {
            let mut context = tera::Context::new();
            context.insert("posts", &data.unwrap());
            let rendered = template_manager
                .render("index.html", &context)
                .unwrap_or_else(|err| format!("Error al renderizar: {}", err));
            HttpResponse::Ok().content_type("text/html").body(rendered)
        }
        Err(err) => HttpResponse::Ok().body(format!("Error al recibir la data: {}", err)),
    }
}

#[get("/blog/{blog_slug}")]
async fn get_post(
    pool: web::Data<DbPool>,
    template_manager: web::Data<tera::Tera>,
    blog_slug: web::Path<String>,
) -> impl Responder {
    let mut conn = pool.get().expect("Problemas al traer la BD");
    let url_slug = blog_slug.into_inner();

    match web::block(move || posts.filter(slug.eq(url_slug)).load::<Post>(&mut conn)).await {
        Ok(data) => {
            let mut context = tera::Context::new();
            let data = data.unwrap();

            if data.len() == 0 {
                return HttpResponse::NotFound().body("No encontrado");
            }

            let data = &data[0];
            context.insert("post", data);

            let rendered = template_manager
                .render("posts.html", &context)
                .unwrap_or_else(|err| format!("Error al renderizar: {}", err));
            HttpResponse::Ok().content_type("text/html").body(rendered)
        }
        Err(err) => HttpResponse::Ok().body(format!("Error al recibir la data: {}", err)),
    }
}

#[post("/new_post")]
async fn new_post(pool: web::Data<DbPool>, item: web::Json<NewPostHandler>) -> impl Responder {
    let conn = pool.get().expect("Problemas al traer la BD");

    println!("{:?}", item);

    match web::block(move || Post::create_post(conn, &item)).await {
        Ok(data) => {
            //println!("{:?}", data);
            HttpResponse::Ok().body(format!("{:?}", data))
        }
        Err(err) => HttpResponse::Ok().body(format!("Error al recibir la data: {}", err)),
    }
}

// #[get("/{name}")]
// async fn hello_world(name: web::Path<String>) -> impl Responder {
//     format!("Hello {}!", &name)
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found");

    let connection = ConnectionManager::<PgConnection>::new(db_url);

    let pool = Pool::builder()
        .build(connection)
        .expect("No se pudo construir la Pool");

    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        App::new()
            .service(index)
            .service(new_post)
            .service(tera_test)
            .service(get_post)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(tera))
    })
    .bind(("127.0.0.1", 8090))
    .unwrap()
    .run()
    .await
}
