use std::thread;
use std::time::Duration;
use std::sync::mpsc::Receiver;

pub fn udp_reader_ann() {
    println!("UDP Reader");
}

pub fn reader_start(udp_rx : Receiver<i32>) {
    thread::spawn(move || {
        reader_run(udp_rx);
    });
}

pub fn reader_run(udp_rx : Receiver<i32>) {
    println!("UDP Reader thread");
}