use std::sync::mpsc;

pub mod udp_reader;
pub mod udp_writer;

pub struct UDPdata{
    pub udp_tx : mpsc::Sender<i32>, 
    pub udp_rx : mpsc::Receiver<i32>, 
}

impl UDPdata {
    pub fn new() -> UDPdata {
        let (l_udp_tx, l_udp_rx) = mpsc::channel();
        UDPdata {
            udp_tx: l_udp_tx,
            udp_rx: l_udp_rx,
        }
    }

    pub fn udp_ann() {
        println!("UDP Module");
    }

    pub fn udp_init(&mut self) {
        udp_reader::reader_start(self.udp_rx);
    }

    pub fn udp_close(&mut self) {
        self.udp_tx.send(0).unwrap();
    }
}
