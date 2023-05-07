use std::env;

use rand::{distributions::Alphanumeric, Rng};
use actix_multipart::form::{MultipartForm, tempfile::TempFile};
use actix_web::{error, web, HttpResponse, Responder, Error, HttpRequest};
use serde::Deserialize;

use crate::util;

#[derive(Debug, MultipartForm)]
pub struct FileData {
    #[multipart(rename = "file")]
    file: TempFile
}

#[derive(Deserialize)]
pub struct URLJson {
    url: String
}

pub async fn save_file(req: HttpRequest, MultipartForm(form): MultipartForm<FileData>) -> Result<impl Responder, Error> {
    if !check_token(req) {
        return Err(error::ErrorForbidden("no"))
    }

    let name = util::secure_filename(form.file.file_name.unwrap());
    let path = format!("tmp/{}", name);

    if form.file.size >= 100 * 1024 * 1024 {
        return Err(error::ErrorPayloadTooLarge(format!("File size >= 100mb")))
    }

    form.file.file.persist(path).unwrap();
    util::clean_exif(format!("tmp/{}", name).as_str());

    Ok(HttpResponse::Ok().body(format!("http://{}/f/{}", env::var("AX_DOMAIN_NAME").to_owned().unwrap(), name)))
}

pub async fn shorten_url(req: HttpRequest, data: web::Form<URLJson>) -> Result<impl Responder, Error> {
    if !check_token(req) {
        return Err(error::ErrorForbidden("no"))
    }

    let short: String = rand::thread_rng()
                            .sample_iter(&Alphanumeric)
                            .take(6)
                            .map(char::from)
                            .collect();

    let connection = sqlite::open("actix.db").unwrap();
    let mut statement = connection.prepare("INSERT INTO shortened_links (full, short, clicks) VALUES (?, ?, 0)").unwrap();

    statement.bind((1, data.url.as_str())).unwrap();
    statement.bind((2, short.as_str())).unwrap();
    statement.next().unwrap();

    Ok(HttpResponse::Ok().body(format!("http://{}/s/{}", env::var("AX_DOMAIN_NAME").to_owned().unwrap(), short)))
}

pub async fn lookup_url(path: web::Path<String>) -> Result<impl Responder, Error> {
    let connection = sqlite::open("actix.db").unwrap();

    let mut statement = connection.prepare("SELECT full FROM shortened_links WHERE short = ?").unwrap();
    let mut clicks_statement = connection.prepare("UPDATE shortened_links SET clicks = clicks + 1 WHERE short = ?").unwrap();

    statement.bind((1, path.as_str())).unwrap();
    statement.next().unwrap();

    let url = match statement.read::<String, _>("full") {
        Ok(data) => data,
        Err(_e) => return Err(error::ErrorNotFound("no"))
    };

    clicks_statement.bind((1, path.as_str())).unwrap();
    clicks_statement.next().unwrap();

    Ok(web::Redirect::to(url))
}

fn check_token(req: HttpRequest) -> bool {
    return req.headers().get("x-token").unwrap().to_str().unwrap() == env::var_os("AX_SECRET_TOKEN").unwrap().to_str().unwrap();
}