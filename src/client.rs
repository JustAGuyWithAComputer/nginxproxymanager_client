use crate::object::ConnectionInformation;


pub struct NginxProxyManagerClient {
    pub connection_info: ConnectionInformation,
    
}


impl NginxProxyManagerClient {
    pub fn new(connection_info: ConnectionInformation) -> Self {
        NginxProxyManagerClient { 
            connection_info: connection_info
        }
    }


    
}