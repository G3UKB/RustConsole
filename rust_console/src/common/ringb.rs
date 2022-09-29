// Testing out ring buffers

use std::collections::VecDeque;
use std::thread;
use std::time::Duration;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;

fn main() {
    let mut deque: VecDeque<u8> = VecDeque::with_capacity(10000);
    let sh_deque = Arc::new(Mutex::new(deque));
    

    let arc1 = sh_deque.clone();
    let sender_join_handle = thread::spawn(  move || {
        rb_sender(& arc1);
    });

    thread::sleep(Duration::from_millis(1000));

    let arc2 = sh_deque.clone();
    let receiver_join_handle = thread::spawn(  move  || {
        rb_reader(& arc2);
    });

    thread::sleep(Duration::from_millis(1000));
    sender_join_handle.join();
    receiver_join_handle.join();
}

fn rb_sender(o: &Arc<Mutex<VecDeque<u8>>>) {
    let q = &mut *o.lock().unwrap();

    println!("Capacity sender {}", q.capacity());
    println!("Length sender {}", q.len());
    println!("Is empty sender {}", q.is_empty());

    q.push_back(1);
    q.push_back(2);
    q.push_back(3);
}

fn rb_reader(o: &Arc<Mutex<VecDeque<u8>>>) {
    let q = &mut *o.lock().unwrap();

    println!("Capacity reader {}", q.capacity());
    println!("Length reader {}", q.len());
    println!("Is empty reader {}", q.is_empty());

    println!("Data {:?},{:?},{:?}", q.pop_front(), q.pop_front(), q.pop_front());
}


