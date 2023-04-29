use rand::{distributions::Alphanumeric, Rng};
use actix_multipart::form::MultipartForm;
use actix_web::{error, web, HttpResponse, Responder, Error, HttpRequest};

use crate::{FileData, URLJson};

pub async fn save_file(req: HttpRequest, MultipartForm(form): MultipartForm<FileData>) -> Result<impl Responder, Error> {
    if !check_token(req) {
        return Err(error::ErrorForbidden("no"))
    }

    let name = form.file.file_name.unwrap();
    let path = format!("tmp/{}", name);

    if form.file.size >= 50 * 1024 * 1024 {
        return Err(error::ErrorPayloadTooLarge(format!("File size >= 50mb")))
    }

    form.file.file.persist(path).unwrap();

    Ok(HttpResponse::Ok().body(format!("http://shockpast.ru/f/{}", name)))
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
    let mut statement = connection.prepare("INSERT INTO shortened_links (full, short) VALUES (?, ?)").unwrap();

    statement.bind((1, data.url.as_str())).unwrap();
    statement.bind((2, short.as_str())).unwrap();
    statement.next().unwrap();

    Ok(HttpResponse::Ok().body(format!("http://shockpast.ru/s/{}", short)))
}

pub async fn lookup_url(path: web::Path<String>) -> Result<impl Responder, Error> {
    let connection = sqlite::open("actix.db").unwrap();
    let mut statement = connection.prepare("SELECT full FROM shortened_links WHERE short = ?").unwrap();

    statement.bind((1, path.as_str())).unwrap();
    statement.next().unwrap();

    let url = statement.read::<String, _>("full").unwrap();

    if url.is_empty() {
        return Err(error::ErrorForbidden("no"))
    }

    Ok(web::Redirect::to(url))
}

fn check_token(req: HttpRequest) -> bool {
    return req.headers().get("x-token").unwrap().to_str().unwrap() == "WmHtoQ%]6#)G&WFh:N*FZ@uZ";
}