use std::{
    error::Error, time::Duration
};

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
    pub timeout: Duration,
    pub logged_in: bool,

    pub header_info: HeaderMap,
    pub connection_info: ConnectionInformation,
}


impl NginxProxyManagerClient {

    pub fn from(url: String, email: String, password: String, timeout: Option<u64>) -> Self {
        NginxProxyManagerClient { 
            token: String::new(),
            timeout: Duration::from_secs(timeout.unwrap_or(5)),
            logged_in: false,

            header_info: HeaderMap::new(),
            connection_info: ConnectionInformation::from(url, email, password),
        }
    }


    pub fn from_env() -> Self {
        NginxProxyManagerClient {
            token: String::new(),
            timeout: Duration::from_secs(5),
            logged_in: false,

            header_info: HeaderMap::new(),
            connection_info: ConnectionInformation::from_env(),
        }
    }


    fn update_token_and_header(&mut self, token: String) -> Result<(), Box<dyn Error>> {
        self.token = token.clone();

        self.header_info.insert(
            AUTHORIZATION, 
            HeaderValue::from_str(&format!("Bearer {token}"))?
        );

        Ok(())
    }


    pub async fn login(&mut self) -> Result<bool, Box<dyn Error>> {
        let url: &String = &self.connection_info.url;

        let request = reqwest::Client::new()
            .post(format!("{url}/api/tokens"))
            .json(&self.connection_info.credentials)
            .timeout(self.timeout);


        let response: Response = request.send().await?;
        let response: TokenResponse = response.json().await?;

        if response.token.is_some() {
            self.logged_in = true;
            self.update_token_and_header(response.token.unwrap())?;

            Ok(true)
        } else {
            let error: TokenErrorMessage = response.error.unwrap();

            let code: i64 = error.code;
            let message: String = error.message;

            self.logged_in = false;

            Err(format!("Failed to login! [{code}] {message}").into())
        }
    }


    pub async fn refresh_token(&mut self) -> Result<bool, Box<dyn Error>> {
        if !self.logged_in {
            self.login().await?;
        }

        let url: &String = &self.connection_info.url;

        let request: RequestBuilder = reqwest::Client::new()
            .get(format!("{url}/api/tokens"))
            .headers(self.header_info.clone())
            .timeout(self.timeout);

        let response: Response = request.send().await?;
        let response: TokenResponse = response.json().await?;

        if response.token.is_some() {
            self.logged_in = true;
            self.update_token_and_header(response.token.unwrap())?;

            Ok(true)
        } else {
            let error: TokenErrorMessage = response.error.unwrap();

            let code: i64 = error.code;
            let message: String = error.message;

            self.logged_in = false;

            Err(format!("Failed to refresh token! [{code}] {message}").into())
        }
    }


    pub async fn get_access_list(&mut self, name: String) -> Result<Option<AccessList>, Box<dyn Error>> {
        if !self.logged_in {
            self.login().await?;
        }
        
        let url: &String = &self.connection_info.url;
        
        let request: RequestBuilder = reqwest::Client::new()
            .get(format!("{url}/api/nginx/access-lists?expand=clients"))
            .headers(self.header_info.clone())
            .timeout(self.timeout);

        let response: Response = request.send().await?;
        let response: Vec<AccessList> = response.json().await?;

        let mut matching_access_list: Option<AccessList> = None;

        for access_list in response {
            if access_list.name == name {
                matching_access_list = Some(access_list);
            }
        }

        if matching_access_list.is_some() {
            Ok(matching_access_list)
        } else {
            Err(format!("Failed to find Access List: '{name}'").into())
        }
    }


    pub async fn update_access_list(&mut self, access_list: AccessList) -> Result<AccessList, Box<dyn Error>> {
        if !self.logged_in {
            self.login().await?;
        }
        
        let id: i64 = access_list.id;
        let url: &String = &self.connection_info.url;
        
        let request: RequestBuilder = reqwest::Client::new()
            .put(format!("{url}/api/nginx/access-lists/{id}"))
            .headers(self.header_info.clone())
            .json(&access_list)
            .timeout(self.timeout);

        let response: Response = request.send().await?;
        let response: AccessList = response.json().await?;
        
        Ok(response)
    }
}