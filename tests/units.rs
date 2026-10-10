use nginxproxymanager_client::object::ConnectionInformation;


#[test]
fn env_connection_information_valid() {
    let info = ConnectionInformation::from_env();

    assert_ne!(info.url, "", "NPM_CLIENT_URL is missing from .env!");
    assert_ne!(info.email, "", "NPM_CLIENT_EMAIL is missing from .env!");
    assert_ne!(info.password, "", "NPM_CLIENT_PASSWORD is missing from .env!");

    assert_ne!(info.url, "https://domain.tld", "NPM_CLIENT_URL is using a placement value in .env!");
    assert_ne!(info.email, "username@email.com", "NPM_CLIENT_EMAIL is using a placement value in .env!");
    assert_ne!(info.password, "abc123", "NPM_CLIENT_PASSWORD is using a placement value in .env!");
}