use nginxproxymanager_client::{
    client::NginxProxyManagerClient, objects::{
        information::ConnectionInformation, responses::{AccessList, AccessListClient}
    }
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
async fn login_credentials_valid() {
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

    let access_list_result = client.get_access_list(target_access_list).await;

    assert_eq!(access_list_result.is_some(), true, "failed to find Access List!");

    if access_list_result.is_some() {
        let mut access_list = access_list_result.unwrap();

        let mut test_entry_index: usize = 0;
        let mut test_entry_exists = false;

        for (index, entry) in access_list.clients.iter().enumerate() {
            if entry.address == "127.0.0.1" {
                test_entry_index = index;
                test_entry_exists = true;
            }
        }

        if test_entry_exists {
            access_list.clients.remove(test_entry_index);
        } else {
            access_list.clients.push(
                AccessListClient::from(
                "127.0.0.1".to_string(),
                "allow".to_string()
                )
            );
        }

        let original = serde_json::to_string(&access_list.clients).unwrap();
        let result = serde_json::to_string(&client.update_access_list(access_list).await.clients).unwrap();

        assert_eq!(original, result, "failed to update access list!");
    }
    
}

