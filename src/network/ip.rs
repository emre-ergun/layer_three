use std::net::Ipv4Addr;

pub fn same_subnet(src: Ipv4Addr, dst: Ipv4Addr, subnet: String) -> bool {
    let subnet_parsed: Ipv4Addr = subnet.parse().unwrap();

    let src_network = u32::from(src) & u32::from(subnet_parsed);
    let dst_network = u32::from(dst) & u32::from(subnet_parsed);

    src_network == dst_network
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subnet_routing() {
        let src = Ipv4Addr::new(192, 168, 0, 1);
        let dest = Ipv4Addr::new(192, 168, 0, 2);
        let subnet = "255.255.0.0".to_owned();

        let result = same_subnet(src, dest, subnet);
        assert!(result);
    }
}
