use serde::{Deserialize, Serialize};


#[derive(Deserialize, Debug, Clone)]
pub struct TokenErrorMessage {
    pub code: i64,
    pub message: String
}


#[derive(Deserialize, Debug)]
pub struct TokenResponse {
    pub token: Option<String>,
    pub expires: Option<String>,

    pub error: Option<TokenErrorMessage>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct AccessListClient {
    pub address: String,
    pub directive: String,
}


impl AccessListClient {
    pub fn from(address: String, directive: String) -> Self {
        AccessListClient {
            address: address, 
            directive: directive 
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct AccessList {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub name: String,
    
    pub pass_auth: bool,
    pub satisfy_any: bool,

    pub clients: Vec<AccessListClient>
}
