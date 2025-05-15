

use actix_files::Files;
use actix_web::{App, HttpServer, web};
mod database;
mod models;
mod routers;
mod services;
use database::db::Database;
use routers::routes::*;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::db_init().await.unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(Files::new("/md_pictures/", "./src/assets/md_pictures/"))
            .configure(index_routes)
            .configure(login_routes)
            .configure(regiser_routes)
            .configure(post)
            .configure(blog_search)
            .configure(blog_tag_all)
            .configure(blog_tag)
            .configure(blog_arch)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await;

    server
}
