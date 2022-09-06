
use std::thread;
use std::time::Duration;

pub mod udp;

fn main() {
    println!("Console main!");

    udp::UDPdata::udp_ann();
    let mut i_udp = udp::UDPdata::new();
    udp::UDPdata::udp_init(&mut i_udp);
    thread::sleep(Duration::from_millis(5000));
    udp::UDPdata::udp_close(&mut i_udp);
}
