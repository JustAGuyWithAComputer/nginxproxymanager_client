use serde::Serialize;


#[derive(Serialize)]
pub struct CredentialInformation {
    pub identity: String,
    pub secret: String
}


impl CredentialInformation {
    pub fn new(email: String, password: String) -> Self {
        CredentialInformation {
            identity: email,
            secret: password
        }
    }
}


pub struct ConnectionInformation {
    pub url: String,
    pub credentials: CredentialInformation
}


impl ConnectionInformation {
    fn trim_url(url: String) -> String {
        return url.strip_suffix("/").unwrap_or(&url).to_string();
    }


    pub fn new(url: String, email: String, password: String) -> Self {
        ConnectionInformation { 
            url: ConnectionInformation::trim_url(url),
            credentials: CredentialInformation::new(email, password)
        }
    }


    pub fn from_env() -> Self {
        use std::env;
        use dotenvy::dotenv;

        dotenv().ok();

        let url = env::var("NPM_CLIENT_URL").unwrap_or("".to_string());
        let email = env::var("NPM_CLIENT_EMAIL").unwrap_or("".to_string());
        let password = env::var("NPM_CLIENT_PASSWORD").unwrap_or("".to_string());

        ConnectionInformation::new(
            ConnectionInformation::trim_url(url),
            email,
            password
        )
    }
}


