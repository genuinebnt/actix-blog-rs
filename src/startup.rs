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
