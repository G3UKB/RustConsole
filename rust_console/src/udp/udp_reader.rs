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
    for i in 1..5 {
        println!("Number {} from the reader thread!", i);
        thread::sleep(Duration::from_millis(1000));
       let r = udp_rx.try_recv();
       let res = match r {
        Ok(file) => break,
        Err(error) => continue,
       };
    }
}