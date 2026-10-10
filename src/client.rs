use std::collections::HashMap;

use crate::object::{ConnectionInformation, CredentialInformation};


pub struct NginxProxyManagerClient {
    pub connection_info: ConnectionInformation,
    pub credential_info: CredentialInformation,
    
    pub token: String
}


impl NginxProxyManagerClient {

    pub fn new(connection_info: ConnectionInformation) -> Self {
        NginxProxyManagerClient { 
            credential_info: CredentialInformation::from(&connection_info),
            connection_info: connection_info,

            token: String::new()
        }
    }


    pub async fn login(&mut self) -> bool {
        let client = reqwest::Client::new();
        let response = client.post(
            self.connection_info.url.to_string() + "/api/tokens"
        ).json(&self.credential_info).send().await.unwrap();

        let success = response.status().is_success();
        
        if success {
            let raw = response.text().await.unwrap();
            
            let data: HashMap<String, String> = serde_json::from_str(&raw).unwrap();
            let token: String = data.get("token").unwrap().to_string();

            self.token = token;
        }

        return success
    }
}