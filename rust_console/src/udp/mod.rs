use crossbeam_channel::unbounded;
use std::net::UdpSocket;

pub mod udp_socket;
pub mod udp_reader;
pub mod udp_writer;

pub struct UDPdata{
    pub sender : crossbeam_channel::Sender<i32>,
    pub receiver : crossbeam_channel::Receiver<i32>,
}

impl UDPdata {
    pub fn new() -> UDPdata {
        let (s, r) = unbounded();
        UDPdata {  
            sender : s,
            receiver : r,
        }
    }

    pub fn udp_ann() {
        println!("UDP Module");
        udp_socket::udp_socket_ann();
        udp_reader::udp_reader_ann();
        udp_writer::udp_writer_ann();
    }

    pub fn udp_init(&mut self) {
        let sock = udp_socket::udp_open_bc_socket();
        udp_socket::udp_revert_socket(sock);
        udp_reader::reader_start(self.receiver.clone());
    }

    pub fn udp_close(&mut self) {
        self.sender.send(0).unwrap();
    }
}
