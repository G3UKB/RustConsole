/*
udp_reader.rs

Module - udp_reader
Manages read data over UDP from the SDR hardware

Copyright (C) 2022 by G3UKB Bob Cowdery

This program is free software; you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation; either version 2 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program; if not, write to the Free Software
Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA

The authors can be reached by email at:

bob@bobcowdery.plus.com
*/

use std::thread;
use std::time::Duration;
use socket2;
use std::sync::Arc;

pub fn reader_start(receiver : crossbeam_channel::Receiver<i32>, p_sock : Arc<socket2::Socket>) {
    thread::spawn(  move || {
        reader_run(receiver, &p_sock);
    });
}

pub fn reader_run(receiver : crossbeam_channel::Receiver<i32>, p_sock : &socket2::Socket) {
    println!("UDP Reader running");
    loop {
        thread::sleep(Duration::from_millis(100));
        // Check for termination code
        let r = receiver.try_recv();
        let _res = match r {
            Ok(_file) => break,
            Err(_error) => continue,
        };

        // Perform read data
    }
    println!("UDP Reader exiting");
    thread::sleep(Duration::from_millis(1000));
}