//use std::sync::mpsc;

use crossbeam_channel::unbounded;

pub mod udp_reader;
pub mod udp_writer;

pub struct UDPdata{
    //pub udp_tx : mpsc::Sender<i32>, 
    //pub udp_rx : mpsc::Receiver<i32>,
    pub sender : crossbeam_channel::Sender<i32>,
    pub receiver : crossbeam_channel::Receiver<i32>,
}

impl UDPdata {
    pub fn new() -> UDPdata {
        //let (l_udp_tx, l_udp_rx) = mpsc::channel();
        let (s, r) = unbounded();
        UDPdata {  
            sender : s,
            receiver : r,
        }
    }

    pub fn udp_ann() {
        println!("UDP Module");
    }

    pub fn udp_init(&mut self) {
        //udp_reader::reader_start(self.udp_rx);
        udp_reader::reader_start(self.receiver.clone());
    }

    pub fn udp_close(&mut self) {
        self.sender.send(0).unwrap();
    }
}
