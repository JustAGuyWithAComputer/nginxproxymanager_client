use nginxproxymanager_client::ConnectionInformation;


#[test]
fn env_is_valid() {
    let info = ConnectionInformation::from_env();

    assert_ne!(info.url, "", "NPM_CLIENT_URL is missing from .env!");
    assert_ne!(info.email, "", "NPM_CLIENT_EMAIL is missing from .env!");
    assert_ne!(info.password, "", "NPM_CLIENT_PASSWORD is missing from .env!");
    assert_ne!(info.access_list, "", "NPM_CLIENT_ACCESS_LIST is missing from .env!");

    assert_ne!(info.url, "https://domain.tld/", "NPM_CLIENT_URL is using a placement value in .env!");
    assert_ne!(info.email, "username@email.com", "NPM_CLIENT_EMAIL is using a placement value in .env!");
    assert_ne!(info.password, "abc123", "NPM_CLIENT_PASSWORD is using a placement value in .env!");
    assert_ne!(info.access_list, "Access List Name", "NPM_CLIENT_ACCESS_LIST is using a placement value in .env!");
}