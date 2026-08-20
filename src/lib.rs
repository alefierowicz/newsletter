use actix_web::{App, HttpResponse, HttpServer, Responder, dev::Server, web};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let port = &listener.local_addr().unwrap().port();
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .listen(listener)?
        .run();
    println!("listening on port {}!", port);
    Ok(server)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
