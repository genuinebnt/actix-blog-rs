#[derive(Deserialize, Default, Debug)]
pub struct Blog {
    pub author: String,
    pub title: String,
    pub content: String,
    pub created_at: i64,
}

#[get("/blogs")]
pub async fn get_blogs() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[post("/blogs")]
pub async fn create_blog(
    _req: HttpRequest,
    state: web::Data<AppState>,
    form: web::Form<Blog>,
) -> HttpResponse {
    let mut data = state.data.lock().unwrap();
    data.push(form.0);
    HttpResponse::Ok().finish()
}
