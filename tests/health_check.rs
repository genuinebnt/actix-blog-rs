use chrono::Utc;
use std::net::TcpListener;

use reqwest;

fn spawn_app() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Cannot start listener");
    let port = listener.local_addr().unwrap().port();
    let server = actix_blog::startup::run(listener).expect("Cannot start server");

    let _ = actix_rt::spawn(server);

    port
}

#[actix_web::test]
async fn returns_200_on_success_health_endpoint() {
    let port = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("http://127.0.0.1:{}/health_check", port))
        .header("Content-Type", "application/x-www-form_urlencoded")
        .send()
        .await
        .expect("Failed to get a response");

    assert_eq!(200, response.status().as_u16());
}

#[actix_web::test]
async fn create_blog_returns_200_for_valid_form_data() {
    let port = spawn_app();

    let client = reqwest::Client::new();

    let params = [
        ("author", "genuine"),
        ("title", "Test blog"),
        ("content", "test body"),
    ];

    let response = client
        .post(&format!("http://127.0.0.1:{}/blogs", port))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await
        .unwrap();

    assert_eq!(200, response.status().as_u16());
}

#[actix_web::test]
async fn return_all_blogs() {
    let port = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("http://127.0.0.1:{}/blogs", port))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .await
        .expect("Failed to get response");

    assert_eq!(response.status().as_u16(), 200);
}
