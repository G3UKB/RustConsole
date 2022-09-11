/*
hw_control.rs

Module - hw_control
Manages starting and stopping the SDR hardware

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
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use socket2;
use std::sync::Arc;
use bytebuffer;

use crate::common;

pub fn hw_control_start(receiver : crossbeam_channel::Receiver<common::HWMsg>, p_sock : Arc<socket2::Socket>) {
    thread::spawn(  move || {
        hw_control_run(receiver, &p_sock);
    });
}

pub fn hw_control_run(receiver : crossbeam_channel::Receiver<common::HWMsg>, p_sock : &socket2::Socket) {
    println!("Hardware Control running");
    loop {
        thread::sleep(Duration::from_millis(100));
        // Check for termination code
        let r = receiver.try_recv();
        match r {
            Ok(file) => {
                match file {
                    common::HWMsg::Terminate => break,
                    common::HWMsg::Discover_HW => discover(p_sock),
                    common::HWMsg::Start_HW => println!("Start!"),
                    common::HWMsg::Stop_HW=> println!("Stop!"),
                };
            },
            Err(_error) => continue,
        };
    }
    println!("Hardware Control  exiting");
    thread::sleep(Duration::from_millis(1000));
}

fn discover(p_sock : &socket2::Socket) {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(255,255,255,255)), 1024);
    let sock2_addr = socket2::SockAddr::from (addr);
    let mut buffer = bytebuffer::ByteBuffer::new();
    let mut buf: [u8; 20] = [0; 20];
    buffer.write_bytes(&vec![0xEF, 0xFF, 0x02]);
    p_sock.send_to(buffer, &sock2_addr);
    let r = p_sock.recv_from(&buf);
    
    println!("Discover!");
}