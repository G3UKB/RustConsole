use crossbeam_channel::unbounded;
use socket2;

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

    pub fn udp_ann(&mut self) {
        println!("UDP Module");
        
        udp_reader::udp_reader_ann();
        udp_writer::udp_writer_ann();
    }

    pub fn udp_init(&mut self) {
        let mut i_socket = udp_socket::Sockdata::new();
        i_socket.udp_socket_ann();
        i_socket.udp_revert_socket();
        let p_sock = i_socket.udp_sock_ref();

        udp_reader::reader_start(self.receiver.clone(), p_sock);
    }

    pub fn udp_close(&mut self) {
        self.sender.send(0).unwrap();
    }
}
