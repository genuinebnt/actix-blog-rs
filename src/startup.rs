use std::net::TcpListener;

use actix_web::{dev::Server, App, HttpServer};

use crate::{
    routes::{
        blogs::{create_blog, get_blogs},
        health_check::health_check,
    },
    state::AppState,
};

pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| {
        App::new()
            .app_data(AppState::default())
            .service(health_check)
            .service(create_blog)
            .service(get_blogs)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
