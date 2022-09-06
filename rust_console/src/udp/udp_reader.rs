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
    loop {
        //println!("Number {} from the reader thread!", i);
        thread::sleep(Duration::from_millis(100));
        // Check for termination code
        let r = receiver.try_recv();
        let res = match r {
            Ok(file) => break,
            Err(error) => continue,
        };

        // Perform read data
    }
}