use nginxproxymanager_client::{
    client::NginxProxyManagerClient,
    object::ConnectionInformation
};



#[tokio::test]
async fn login_information_valid() {
    let mut client = NginxProxyManagerClient::new(
        ConnectionInformation::from_env()
    );

    assert_eq!(client.login().await, true, "failed to login!");
}


#[tokio::test]
async fn can_modify_access_list() {
    /*let client = NginxProxyManagerClient::new(
        ConnectionInformation::from_env()
    );*/
    
}


