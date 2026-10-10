use dotenvy::dotenv;
use std::env;


#[test]
pub fn env_properly_configured() {
    dotenv().ok();

    let url = env::var("KEY").unwrap_or("".to_string());
    let email = env::var("EMAIL").unwrap_or("".to_string());
    let password = env::var("PASSWORD").unwrap_or("".to_string());
    let access_list = env::var("ACCESS_LIST").unwrap_or("".to_string());

    assert_ne!(url, "", "URL is missing from .env!");
    assert_ne!(email, "", "EMAIL is missing from .env!");
    assert_ne!(password, "", "PASSWORD is missing from .env!");
    assert_ne!(access_list, "", "ACCESS_LIST is missing from .env!");

    assert_ne!(url, "https://domain.tld/", "URL is using a placement value in .env!");
    assert_ne!(email, "username@email.com", "EMAIL is using a placement value in .env!");
    assert_ne!(password, "abc123", "PASSWORD is using a placement value in .env!");
    assert_ne!(access_list, "Access List Name", "ACCESS_LIST is using a placement value in .env!");
}