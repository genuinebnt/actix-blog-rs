use std::net::TcpListener;

use actix_blog::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Cannot start listener");

    run(listener)?.await?;

    Ok(())
}
