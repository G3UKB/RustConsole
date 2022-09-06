use std::thread;
use std::time::Duration;
use crossbeam_channel::unbounded;

pub fn udp_reader_ann() {
    println!("UDP Reader");
}

pub fn reader_start(receiver : crossbeam_channel::Receiver<i32>) {
    thread::spawn(move || {
        reader_run(receiver);
    });
}

pub fn reader_run(receiver : crossbeam_channel::Receiver<i32>) {
    for i in 1..5 {
        println!("Number {} from the reader thread!", i);
        thread::sleep(Duration::from_millis(1000));
       //let r = .try_recv();
       //let res = match r {
        //Ok(file) => break,
        //Err(error) => continue,
    }
}