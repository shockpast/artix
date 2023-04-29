mod sharex;

use dotenv::dotenv;
use actix_files as fs;
use actix_multipart::form::{MultipartForm, tempfile::{TempFile, TempFileConfig}};
use actix_web::{web, App, HttpServer, http::KeepAlive};
use serde::Deserialize;

#[derive(Debug, MultipartForm)]
pub struct FileData {
    #[multipart(rename = "file")]
    file: TempFile
}

#[derive(Deserialize)]
pub struct URLJson {
    url: String
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // there might be some better way to do that kind of job
    let connection = sqlite::open("actix.db").unwrap();
    connection.execute("
        CREATE TABLE IF NOT EXISTS shortened_links (full TEXT, short TEXT)
    ").unwrap();

    dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .app_data(TempFileConfig::default().directory("tmp"))
            .service(web::resource("/upload").route(web::post().to(sharex::save_file)))
            .service(web::resource("/shorten").route(web::post().to(sharex::shorten_url)))
            .service(web::resource("/s/{id}").route(web::get().to(sharex::lookup_url)))
            .service(fs::Files::new("/f", "./tmp"))
    })
    .keep_alive(KeepAlive::Os)
    .bind(("0.0.0.0", 3000))?
    .workers(4)
    .run()
    .await
}