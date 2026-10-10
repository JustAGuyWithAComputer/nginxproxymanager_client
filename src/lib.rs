pub struct ConnectionInformation {
    pub url: String,
    pub email: String,
    pub password: String,
    pub access_list: String
}


impl ConnectionInformation {
    pub fn new(url: String, email: String, password: String, access_list: String) -> Self {
        ConnectionInformation { 
            url: url, 
            email: email, 
            password: password, 
            access_list: access_list 
        }
    }

    pub fn from_env() -> Self {
        use std::env;
        use dotenvy::dotenv;

        dotenv().ok();

        let url = env::var("NPM_CLIENT_URL").unwrap_or("".to_string());
        let email = env::var("NPM_CLIENT_EMAIL").unwrap_or("".to_string());
        let password = env::var("NPM_CLIENT_PASSWORD").unwrap_or("".to_string());
        let access_list = env::var("NPM_CLIENT_ACCESS_LIST").unwrap_or("".to_string());

        ConnectionInformation::new(
            url,
            email,
            password,
            access_list
        )
    }
}