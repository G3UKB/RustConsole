use crossbeam_channel::unbounded;

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
    }

    pub fn udp_init(&mut self) {
        udp_reader::reader_start(self.receiver.clone());
    }

    pub fn udp_close(&mut self) {
        self.sender.send(0).unwrap();
    }
}
