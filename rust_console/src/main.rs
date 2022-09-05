use std::sync::mpsc;

use crate::udp::udp_reader;
use crate::udp::udp_writer;


pub mod udp;

fn main() {
    println!("Console main!");

    let (udp_tx, udp_rx) = mpsc::channel();

    udp::udp_ann();
    udp_reader::udp_reader_ann();
    udp_reader::reader_start(udp_rx);
    udp_writer::udp_writer_ann();
}
