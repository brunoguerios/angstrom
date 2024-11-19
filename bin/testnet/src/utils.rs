use std::net::{IpAddr, Ipv4Addr};

pub fn my_ip() -> IpAddr {
    let my_local_ip = local_ip_address::local_ip().unwrap();

    println!("This is my local IP address: {:?}", my_local_ip);
    my_local_ip
}

#[test]
pub fn my_ip_t() {
    my_ip();
}
