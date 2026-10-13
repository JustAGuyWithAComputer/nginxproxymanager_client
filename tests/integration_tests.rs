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
    
    if client.client_exists_in_access_list(
        access_list_name.clone(), "127.0.0.1".to_string()
    ).await? {
        client.remove_client_from_access_list(
            access_list_name.clone(), "127.0.0.1".to_string()
        ).await?
    }

    client.add_client_to_access_list(
        access_list_name.clone(), "127.0.0.1".to_string(), "allow".to_string()
    ).await?;

    assert_eq!(
        client.client_exists_in_access_list(access_list_name.clone(), "127.0.0.1".to_string()).await?, 
        true, 
        "failed to update access list!"
    );

    client.remove_client_from_access_list(
        access_list_name.clone(), "127.0.0.1".to_string()
    ).await?;

    assert_eq!(
        client.client_exists_in_access_list(access_list_name.clone(), "127.0.0.1".to_string()).await?, 
        false, 
        "failed to update access list!"
    );
    
    Ok(())
}

