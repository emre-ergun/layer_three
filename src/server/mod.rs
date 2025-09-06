pub mod arp_cache;
pub mod hosts;
pub mod interface;
use core::fmt;

use interface::Interface;

#[allow(dead_code)]
struct Route {
    pub desttination: String,
    pub gateway: String,
    pub interface_name: String,
}

#[allow(dead_code)]
pub struct Server {
    pub hostname: String,
    pub interface: Interface,
    routes: Vec<Route>,
    pub arp_table: arp_cache::ArpCache,
}

impl Server {
    pub fn new(hostname: String, interface: Interface) -> Server {
        Server {
            hostname,
            interface,
            routes: vec![],
            arp_table: arp_cache::ArpCache::new(),
        }
    }
}

impl fmt::Debug for Server {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // chaining debug statements in rust is messy and tricky
        // if we just make a string here, it is easier
        let mac_str = self
            .interface
            .mac
            .iter()
            .map(|byte| format!("{:02X}", byte))
            .collect::<Vec<String>>()
            .join(":");

        f.debug_struct("Server")
            .field("hostname", &self.hostname)
            .field("interface.mac", &mac_str)
            .finish()
    }
}
