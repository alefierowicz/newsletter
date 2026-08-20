use actix_web::{App, HttpResponse, HttpServer, Responder, dev::Server, web};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let port = &listener.local_addr().unwrap().port();

    let server = HttpServer::new(|| {
        App::new().route("/health_check", web::get().to(health_check))
        // .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    println!("listening on port {}!", port);
    Ok(server)
}

// #[derive(serde::Deserialize)]
// struct FormData {
//     name: String,
//     email: String,
// }

// async fn subscribe(_form: web::Form<FormData>) -> impl Responder {
//     HttpResponse::Ok().finish()
// }

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
