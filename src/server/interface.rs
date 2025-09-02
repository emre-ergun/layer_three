use crate::mac::{self, MacAddress};

#[derive(Clone)]
pub struct Interface {
    pub mac: MacAddress,
    pub ip: String,
    pub subnet_mask: String,
    pub gateway: Option<String>,
}

impl Interface {
    pub fn new(mac: &str, ip: String, subnet_mask: String, gateway: Option<String>) -> Interface {
        Interface {
            mac: mac::parse_mac_address(mac),
            ip,
            subnet_mask,
            gateway,
        }
    }
}
