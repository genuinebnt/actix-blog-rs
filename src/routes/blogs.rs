use actix_web::{get, post, web, HttpRequest, HttpResponse};

use crate::state::Blog;

#[get("/blogs")]
pub async fn get_blogs() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[post("/blogs")]
pub async fn create_blog(_req: HttpRequest, form: web::Form<Blog>) -> HttpResponse {
    println!("{}:{}:{}", form.0.author, form.0.title, form.0.content);

    HttpResponse::Ok().finish()
}
