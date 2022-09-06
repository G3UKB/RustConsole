
use std::thread;
use std::time::Duration;

use crate::udp::udp_reader;
use crate::udp::udp_writer;


pub mod udp;

fn main() {
    println!("Console main!");

    udp::UDPdata::udp_ann();
    udp_reader::udp_reader_ann();
    let mut i_udp = udp::UDPdata::new();
    udp::UDPdata::udp_init(&mut i_udp);
    udp_writer::udp_writer_ann();
    thread::sleep(Duration::from_millis(5000));
    udp::UDPdata::udp_close(&mut i_udp);
}
