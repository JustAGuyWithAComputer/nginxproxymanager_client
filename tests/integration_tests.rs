use std::error::Error;

use nginxproxymanager_client::{
    client::NginxProxyManagerClient, objects::{
        information::ConnectionInformation, 
        responses::{
            AccessList, AccessListClient
        }
    }, 
};


#[test]
fn connection_information_valid() {
    let info: ConnectionInformation = ConnectionInformation::from_env();

    assert_ne!(info.url, "", "NPM_CLIENT_URL is missing from .env!");
    assert_ne!(info.credentials.identity, "", "NPM_CLIENT_EMAIL is missing from .env!");
    assert_ne!(info.credentials.secret, "", "NPM_CLIENT_PASSWORD is missing from .env!");

    assert_ne!(info.url, "https://domain.tld", "NPM_CLIENT_URL is using a placement value in .env!");
    assert_ne!(info.credentials.identity, "username@email.com", "NPM_CLIENT_EMAIL is using a placement value in .env!");
    assert_ne!(info.credentials.secret, "abc123", "NPM_CLIENT_PASSWORD is using a placement value in .env!");
}


#[tokio::test]
async fn login_credentials_valid() -> Result<(), Box<dyn Error>> {
    let mut client: NginxProxyManagerClient = NginxProxyManagerClient::from_env();

    assert_eq!(client.login().await?, true, "failed to login!");
    assert_eq!(client.refresh_token().await?, true, "failed to refresh token!");

    Ok(())
}


#[tokio::test]
async fn automatic_login_valid() -> Result<(), Box<dyn Error>> {
    let mut client = NginxProxyManagerClient::from_env();

    assert_eq!(client.refresh_token().await?, true, "failed to login automatically!");

    Ok(())
}


#[tokio::test]
async fn can_modify_access_list() -> Result<(), Box<dyn Error>> {
    let access_list_name: String = dotenvy::var("NPM_CLIENT_ACCESS_LIST")?;
    let mut client: NginxProxyManagerClient = NginxProxyManagerClient::from_env();
    
    let mut access_list: AccessList = client.get_access_list(access_list_name).await?;

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

    let original: String = serde_json::to_string(&access_list.clients)?;
    let result: String = serde_json::to_string(
        &client.update_access_list(access_list).await?.clients
    )?;

    assert_eq!(original, result, "failed to update access list!");
    
    Ok(())
}

