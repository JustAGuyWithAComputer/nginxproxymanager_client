use nginxproxymanager_client::{
    client::NginxProxyManagerClient,
    objects::information::ConnectionInformation
};


#[test]
fn connection_information_valid() {
    let info = ConnectionInformation::from_env();

    assert_ne!(info.url, "", "NPM_CLIENT_URL is missing from .env!");
    assert_ne!(info.credentials.identity, "", "NPM_CLIENT_EMAIL is missing from .env!");
    assert_ne!(info.credentials.secret, "", "NPM_CLIENT_PASSWORD is missing from .env!");

    assert_ne!(info.url, "https://domain.tld", "NPM_CLIENT_URL is using a placement value in .env!");
    assert_ne!(info.credentials.identity, "username@email.com", "NPM_CLIENT_EMAIL is using a placement value in .env!");
    assert_ne!(info.credentials.secret, "abc123", "NPM_CLIENT_PASSWORD is using a placement value in .env!");
}


#[tokio::test]
async fn login_information_valid() {
    let mut client = NginxProxyManagerClient::from_env();

    assert_eq!(client.login().await, true, "failed to login!");

    assert_eq!(client.refresh_token().await, true, "failed to refresh token!");
    assert_eq!(client.refresh_token().await, true, "failed to refresh token!");
}


#[tokio::test]
async fn can_modify_access_list() {
    let mut client = NginxProxyManagerClient::from_env();
    let target_access_list = dotenvy::var("NPM_CLIENT_ACCESS_LIST").unwrap();

    assert_eq!(client.login().await, true, "failed to login!");

    /*let client = NginxProxyManagerClient::new(
        ConnectionInformation::from_env()
    );*/
    
}

