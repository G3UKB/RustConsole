use std::net::UdpSocket;
use get_if_addrs;

pub fn udp_socket_ann() {
    println!("UDP Socket");
}

pub fn udp_open_bc_socket() {
    let sock = UdpSocket::bind(get_ip() + ":" + "10000");
}

pub fn get_ip() -> String{
    let iface = get_if_addrs::get_if_addrs().unwrap();
    return iface[0].ip().to_string();
}