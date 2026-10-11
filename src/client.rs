use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};

use crate::objects::{
    information::ConnectionInformation, responses::{
        AccessList, TokenResponse
    },
};


pub struct NginxProxyManagerClient {
    pub header_info: HeaderMap,
    pub connection_info: ConnectionInformation,

    pub token: String,
    pub bearer: String,
}


impl NginxProxyManagerClient {

    pub fn from(url: String, email: String, password: String) -> Self {
        NginxProxyManagerClient { 
            header_info: HeaderMap::new(),
            connection_info: ConnectionInformation::from(url, email, password),

            token: String::new(),
            bearer: String::new(),
        }
    }


    pub fn from_env() -> Self {
        NginxProxyManagerClient {
            header_info: HeaderMap::new(),
            connection_info: ConnectionInformation::from_env(),

            token: String::new(),
            bearer: String::new()
        }
    }


    fn update_token_and_header(&mut self, new_token: String) {
        self.bearer = "Bearer ".to_owned() + &new_token;
        self.token = new_token;

        if self.header_info.contains_key(AUTHORIZATION) {
            let value = self.header_info.get_mut(AUTHORIZATION).unwrap();
            *value = HeaderValue::from_str(&self.bearer).unwrap();
        } else {
            self.header_info.insert(AUTHORIZATION, HeaderValue::from_str(&self.bearer).unwrap());
        }
    }


    pub async fn login(&mut self) -> bool {
        let client = reqwest::Client::new();

        let response: TokenResponse = client
            .post(self.connection_info.url.to_string() + "/api/tokens")
            .json(&self.connection_info.credentials)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        let success = response.error.is_none();

        if success {
            self.update_token_and_header(
                response.token
            );
        } else {
            println!(
                "Failed to connect login!\n[{:#?}] {:#?}", 
                response.error.clone().unwrap().code, 
                response.error.unwrap().message
            )
        }

        return success
    }


    pub async fn refresh_token(&mut self) -> bool {
        let client = reqwest::Client::new();

        let response: TokenResponse = client
            .get(self.connection_info.url.to_string() + "/api/tokens")
            .headers(self.header_info.clone())
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        let success = response.error.is_none();

        if success {
            self.update_token_and_header(
                response.token
            );
        } else {
            println!(
                "Failed to connect login!\n[{:#?}] {:#?}", 
                response.error.clone().unwrap().code, 
                response.error.unwrap().message
            )
        }

        return success
    }


    pub async fn get_access_list(&mut self, name: String) -> Option<AccessList> {
        let client = reqwest::Client::new();
        let all_access_lists: Vec<AccessList> = client
            .get(reqwest::Url::parse_with_params(
                &(self.connection_info.url.to_string() + "/api/nginx/access-lists"),
                &[("expand", "owner,items,clients")]
            ).unwrap())
            .headers(self.header_info.clone())
            .send()
            .await.unwrap()
            .json()
            .await
            .unwrap();
            //.unwrap();

        for access_list in all_access_lists {
            if access_list.name.to_lowercase() == name.to_lowercase() {
                return Some(access_list)
            }
        }

        return None;
    }


    pub async fn update_access_list(&self, mut access_list: AccessList) -> AccessList {
        let client = reqwest::Client::new();

        let id = access_list.id.unwrap();
        access_list.id = None;

        let response: AccessList = client
            .put(self.connection_info.url.to_string() + "/api/nginx/access-lists/" + &id.to_string())
            .headers(self.header_info.clone())
            .json(&access_list)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        return response;
    }
}