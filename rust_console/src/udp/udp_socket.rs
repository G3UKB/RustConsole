use std::net::UdpSocket;
use std::time::Duration;
use get_if_addrs;
use socket2;

pub fn udp_socket_ann() {
    println!("UDP Socket");
}

pub fn udp_open_bc_socket() -> UdpSocket{
    let sock = UdpSocket::bind(get_ip() + ":" + "10000").expect("couldn't bind to address");
    sock.set_broadcast(true).expect("set_broadcast call failed");
    sock.set_read_timeout(Some(Duration::from_millis(10))).expect("set_read_timeout call failed");
    return sock;
}

pub fn udp_revert_socket(sock: UdpSocket) {
    sock.set_broadcast(false).expect("set_broadcast call failed");
    sock.set_read_timeout(Some(Duration::from_millis(10))).expect("set_read_timeout call failed");
    // Set buffer sizes?
    let sock2 = socket2::Socket::from (sock);
    sock2.set_recv_buffer_size(192000);
    sock2.set_send_buffer_size(192000);
}

pub fn get_ip() -> String{
    let iface = get_if_addrs::get_if_addrs().unwrap();
    return iface[0].ip().to_string();
}