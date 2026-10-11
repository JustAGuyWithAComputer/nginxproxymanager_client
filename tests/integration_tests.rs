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
fn connection_information_is_not_missing_or_placement() -> Result<(), Box<dyn Error>> {
    let info: ConnectionInformation = ConnectionInformation::from_env();
    let mut error: String = String::from("Invalid or missing environment variables! ");

    match info.url.as_str() {
        "https://domain.tld" => error += "NPM_CLIENT_URL[placement], ",
        "" => error += "NPM_CLIENT_URL[missing], ",
        _ => {}
    }

    match info.credentials.identity.as_str() {
        "username@domain.tld" => error += "NPM_CLIENT_EMAIL[placement], ",
        "" => error += "NPM_CLIENT_EMAIL[missing], ",
        _ => {}
    }

    match info.credentials.secret.as_str() {
        "abc123" => error += "NPM_CLIENT_PASSWORD[placement], ",
        "" => error += "NPM_CLIENT_PASSWORD[missing], ",
        _ => {}
    }

    if error != "Invalid or missing environment variables! " {
        Err(error.strip_suffix(", ").unwrap_or(&error).into())
    } else {
        Ok(())
    }
}


#[tokio::test]
async fn login_credentials_valid() -> Result<(), Box<dyn Error>> {
    let mut client: NginxProxyManagerClient = NginxProxyManagerClient::from_env();

    client.login().await?;

    Ok(())
}


#[tokio::test]
async fn automatic_login_and_refresh_token_valid() -> Result<(), Box<dyn Error>> {
    let mut client: NginxProxyManagerClient = NginxProxyManagerClient::from_env();

    client.refresh_token().await?;

    Ok(())
}


#[tokio::test]
async fn can_modify_access_list() -> Result<(), Box<dyn Error>> {
    let access_list_name: String = dotenvy::var("NPM_CLIENT_ACCESS_LIST")?;
    let mut client: NginxProxyManagerClient = NginxProxyManagerClient::from_env();
    
    let mut access_list: AccessList = client.get_access_list(access_list_name).await?;

    let mut test_entry_index: usize = 0;
    let mut test_entry_exists: bool = false;

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

