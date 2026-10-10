use nginxproxymanager_client::{
    client::NginxProxyManagerClient,
    objects::information::ConnectionInformation
};



#[tokio::test]
async fn login_information_valid() {
    let mut client = NginxProxyManagerClient::new(
        ConnectionInformation::from_env()
    );

    assert_eq!(client.login().await, true, "failed to login!");

    assert_eq!(client.refresh_token().await, true, "failed to refresh token!");
    assert_eq!(client.refresh_token().await, true, "failed to refresh token!");
}


#[tokio::test]
async fn can_modify_access_list() {
    /*let client = NginxProxyManagerClient::new(
        ConnectionInformation::from_env()
    );*/
    
}

