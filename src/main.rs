pub mod database;
pub mod tools;
pub mod util;

use dotenv::dotenv;
use actix_files::Files;
use actix_multipart::form::{tempfile::TempFileConfig, MultipartFormConfig};
use actix_web::{web, App, HttpServer, http::KeepAlive};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    database::create();
    dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .app_data(TempFileConfig::default().directory("tmp"))
            .app_data(MultipartFormConfig::default().total_limit(100*1024*1024)) // 100mb
            .service(web::resource("/upload").route(web::post().to(tools::sharex::save_file)))
            .service(web::resource("/shorten").route(web::post().to(tools::sharex::shorten_url)))
            .service(web::resource("/s/{id}").route(web::get().to(tools::sharex::lookup_url)))
            .service(Files::new("/f", "./tmp"))
    })
    .keep_alive(KeepAlive::Os)
    .bind(("0.0.0.0", 3000))?
    .workers(4)
    .run()
    .await
}