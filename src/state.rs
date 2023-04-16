use std::sync::Mutex;

use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
pub struct Blog {
    pub author: String,
    pub title: String,
    pub content: String,
}

#[derive(Default, Debug)]
pub struct AppState {
    pub data: Mutex<Vec<Blog>>,
}
