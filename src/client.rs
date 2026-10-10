use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};

use crate::objects::{
    responses::{
        TokenResponse,
        TokenFailResponse
    },

    information::ConnectionInformation,
};


pub struct NginxProxyManagerClient {
    pub header_info: HeaderMap,
    pub connection_info: ConnectionInformation,

    pub token: String,
    pub bearer: String,
}


impl NginxProxyManagerClient {

    pub fn new(connection_info: ConnectionInformation) -> Self {
        NginxProxyManagerClient { 
            header_info: HeaderMap::new(),
            connection_info: connection_info,

            token: String::new(),
            bearer: String::new(),

        }
    }


    fn update_token_and_header(&mut self, new_token: String) {
        self.bearer = String::from("Bearer ") + &new_token.to_string();
        self.token = new_token;

        if self.header_info.contains_key(AUTHORIZATION) {
            if let Some(value) = self.header_info.get_mut(AUTHORIZATION) {
                *value = HeaderValue::from_str(&self.bearer).unwrap();
            }
        } else {
            self.header_info.insert(AUTHORIZATION, HeaderValue::from_str(&self.bearer).unwrap());
        }
    }


    pub async fn login(&mut self) -> bool {
        let client = reqwest::Client::new();

        let response = client.post(
            self.connection_info.url.to_string() + "/api/tokens"
        ).json(&self.connection_info.credentials).send().await.unwrap();

        let success = response.status().is_success();

        if success {
            self.update_token_and_header(
                response.json::<TokenResponse>().await.unwrap().token
            );
        } else {
            let token_response = response.json::<TokenFailResponse>().await.unwrap();

            println!(
                "Failed to connect login!\n[{:#?}] {:#?}", 
                token_response.error.code, 
                token_response.error.message
            )
        }

        return success
    }


    pub async fn refresh_token(&mut self) -> bool {
        let client = reqwest::Client::new();
        
        let response = client.get(
            self.connection_info.url.to_string() + "/api/tokens"
        ).headers(self.header_info.clone()).send().await.unwrap();

        let success = response.status().is_success();

        if success {
            self.update_token_and_header(
                response.json::<TokenResponse>().await.unwrap().token
            );
        } else {
            let token_response = response.json::<TokenFailResponse>().await.unwrap();

            println!(
                "Failed to refresh token!\n[{:#?}] {:#?}", 
                token_response.error.code, 
                token_response.error.message
            )
        }

        return success
    }

}