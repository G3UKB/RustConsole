use std::sync::mpsc;

pub mod udp_reader;
pub mod udp_writer;

pub struct UDPdata{
    udp_tx : mpsc::Sender<i32>, 
    udp_rx : mpsc::Receiver<i32>, 
}

impl UDPdata {
    pub fn udp_ann() {
        println!("UDP Module");
    }

    pub fn udp_init() {

    }

    pub fn udp_close() {

    }
}