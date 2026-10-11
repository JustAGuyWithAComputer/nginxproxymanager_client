use std::error::Error;

use reqwest::{
    RequestBuilder, Response, header::{
        AUTHORIZATION, HeaderMap, HeaderValue
    }
};

use crate::objects::{
    information::ConnectionInformation, responses::{
        AccessList, TokenErrorMessage, TokenResponse
    },
};


pub struct NginxProxyManagerClient {
    pub token: String,

    pub header_info: HeaderMap,
    pub connection_info: ConnectionInformation,
}


impl NginxProxyManagerClient {

    pub fn from(url: String, email: String, password: String) -> Self {
        NginxProxyManagerClient { 
            token: String::new(),

            header_info: HeaderMap::new(),
            connection_info: ConnectionInformation::from(url, email, password),
        }
    }


    pub fn from_env() -> Self {
        NginxProxyManagerClient {
            token: String::new(),

            header_info: HeaderMap::new(),
            connection_info: ConnectionInformation::from_env(),
        }
    }


    fn update_token_and_header(&mut self, new_token: String) -> Result<bool, Box<dyn Error>> {
        self.token = new_token.clone();

        self.header_info.insert(
            AUTHORIZATION, 
            HeaderValue::from_str(&format!("Bearer {new_token}"))?
        );

        Ok(true)
    }


    pub async fn login(&mut self) -> Result<bool, Box<dyn Error>> {
        let url: &String = &self.connection_info.url;

        let request = reqwest::Client::new()
            .post(format!("{url}/api/tokens"))
            .json(&self.connection_info.credentials);


        let response: Response = request.send().await?;
        let response: TokenResponse = response.json().await?;

        let success: bool = response.error.is_none();

        if success {
            let _ = self.update_token_and_header(response.token.unwrap());
        } else {
            let error: TokenErrorMessage = response.error.unwrap();

            let code: i64 = error.code;
            let message: String = error.message;

            return Err(format!("Failed to login! [{code}] {message}").into())
        }

        return Ok(success)
    }


    pub async fn refresh_token(&mut self) -> Result<bool, Box<dyn Error>> {
        let url: &String = &self.connection_info.url;

        let request: RequestBuilder = reqwest::Client::new()
            .get(format!("{url}/api/tokens"))
            .headers(self.header_info.clone());

        let response: Response = request.send().await?;
        let response: TokenResponse = response.json().await?;

        let success: bool = response.error.is_none();

        if success {
            self.update_token_and_header(response.token.unwrap())?;
        } else {
            let error: TokenErrorMessage = response.error.unwrap();

            let code: i64 = error.code;
            let message: String = error.message;

            return Err(format!("Failed to refresh token! [{code}] {message}").into())
        }

        Ok(success)
    }


    pub async fn get_access_list(&mut self, name: String) -> Result<Option<AccessList>, Box<dyn Error>> {
        let url: &String = &self.connection_info.url;
        
        let request: RequestBuilder = reqwest::Client::new()
            .get(format!("{url}/api/nginx/access-lists?expand=clients"))
            .headers(self.header_info.clone());

        let response: Response = request.send().await?;
        let response: Vec<AccessList> = response.json().await?;

        for access_list in response {
            if access_list.name == name {
                return Ok(Some(access_list))
            }
        }

        Ok(None)
    }


    pub async fn update_access_list(&self, mut access_list: AccessList) -> Result<AccessList, Box<dyn Error>> {
        let url: &String = &self.connection_info.url;

        // NginxProxyManager doesn't like the access_list having an id when PUTing.
        let id: i64 = access_list.id.unwrap(); access_list.id = None;
        
        let request: RequestBuilder = reqwest::Client::new()
            .put(format!("{url}/api/nginx/access-lists/{id}"))
            .headers(self.header_info.clone())
            .json(&access_list);

        let response: Response = request.send().await?;
        let response: AccessList = response.json().await?;

        Ok(response)
    }
}