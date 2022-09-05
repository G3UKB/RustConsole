
use crate::udp::udp_reader;
use crate::udp::udp_writer;


pub mod udp;

fn main() {
    println!("Console main!");

    udp::UDPdata::udp_ann();
    udp_reader::udp_reader_ann();
    udp::UDPdata::udp_init();
    udp_writer::udp_writer_ann();
}
