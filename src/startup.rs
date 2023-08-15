use actix_web::{dev::Server, web, App, HttpServer};
use std::net::TcpListener;

use crate::routes;

pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(routes::health_check))
            .route("subscriptions", web::post().to(routes::subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
