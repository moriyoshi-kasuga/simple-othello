#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoginRequest {
    pub username: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoginResponse {
    pub token: String,
}
