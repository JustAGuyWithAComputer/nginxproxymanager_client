use serde::Deserialize;


#[derive(Deserialize)]
pub struct TokenResponse {
    pub token: String,
    pub expires: String
}


#[derive(Deserialize)]
pub struct TokenFailErrorMessage {
    pub code: i64,
    pub message: String
}


#[derive(Deserialize)]
pub struct TokenFailResponse {
    pub error: TokenFailErrorMessage
}