pub struct ConnectionInformation {
    pub url: String,
    pub email: String,
    pub password: String,
}


impl ConnectionInformation {
    fn trim_url(url: String) -> String {
        return url.strip_suffix("/").unwrap_or(&url).to_string();
    }


    pub fn new(url: String, email: String, password: String) -> Self {
        ConnectionInformation { 
            url: ConnectionInformation::trim_url(url), 
            email: email, 
            password: password, 
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