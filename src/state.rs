#[derive(Default, Debug)]
pub struct AppState {
    pub data: Mutex<Vec<Blog>>,
}
