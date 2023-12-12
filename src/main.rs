#![warn(clippy::pedantic)]
use actix_cors::Cors;
use actix_files as fs;
use actix_service::Service;
use actix_web::http::header::{HeaderValue, CONTENT_ENCODING, CONTENT_TYPE};
use actix_web::{get, App, HttpResponse, HttpServer, Result};
use std::env;

// Get a list of all games currently available
#[get("/")]
async fn list_games() -> Result<HttpResponse> {
    let mut dirs = Vec::new();

    for entry in std::fs::read_dir("./games")? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            dirs.push(entry.file_name().into_string().unwrap());
        }
    }
    Ok(HttpResponse::Ok().json(dirs))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| String::from("8080"))
        .parse()
        .unwrap_or(8080);

    HttpServer::new(|| {
        let cors = Cors::default().allow_any_method().allow_any_origin();

        App::new()
            .wrap(cors)
            .wrap_fn(|req, srv| {
                let req_path = req.path().to_string();
                let fut = srv.call(req);

                async move {
                    let mut res = fut.await?;

                    if req_path.contains(".wasm") {
                        res.headers_mut()
                            .insert(CONTENT_TYPE, HeaderValue::from_static("application/wasm"));
                    }

                    if req_path.contains(".gz") {
                        res.headers_mut()
                            .insert(CONTENT_ENCODING, HeaderValue::from_static("gzip"));
                    }

                    Ok(res)
                }
            })
            .service(list_games)
            .service(
                fs::Files::new("/", "./games")
                    .redirect_to_slash_directory()
                    .index_file("index.html"),
            )
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
