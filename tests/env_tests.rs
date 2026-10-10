use nginxproxymanager_client::objects::information::ConnectionInformation;


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