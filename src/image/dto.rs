use serde::Deserialize;

#[derive(Deserialize)]
pub struct CloudinaryResponse {
    pub secure_url: String,
    pub public_id: String,
}

#[derive(Deserialize)]
pub struct Base64Upload {
    pub image: String,
}
