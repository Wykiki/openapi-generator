use actix_cors::Cors;
use actix_service::Service;
use actix_web::{get, http::header, http::HeaderValue, web, App, HttpServer, Responder};

mod app;
mod model;
mod route;

use app::config;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap_fn(|req, srv| {
                let fut = srv.call(req);
                async {
                    let mut res = fut.await?;
                    res.headers_mut().insert(
                        header::ACCESS_CONTROL_ALLOW_ORIGIN,
                        HeaderValue::from_static("*"),
                    );
                    Ok(res)
                }
            })
            .wrap(
                Cors::new()
                    .allowed_origin("All")
                    .send_wildcard()
                    // .allowed_methods(vec!["GET", "POST"])
                    // .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    // .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish(),
            )
            .configure(config)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
