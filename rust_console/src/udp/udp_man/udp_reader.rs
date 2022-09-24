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
use std::mem::MaybeUninit;
use std::sync::Arc;
use crate::protocol;

use crate::common::common_defs;
use crate::common::messages;

pub fn reader_start(receiver : crossbeam_channel::Receiver<messages::ReaderMsg>, p_sock : Arc<socket2::Socket>, p_addr : Arc<socket2::SockAddr>) {
    thread::spawn(  move || {
        reader_run(receiver, &p_sock, &p_addr);
    });
}

pub fn reader_run(receiver : crossbeam_channel::Receiver<messages::ReaderMsg>, p_sock : &socket2::Socket, p_addr : &socket2::SockAddr) {
    println!("UDP Reader running");

    let mut listen: bool = false;
    let mut udp_frame: [MaybeUninit<u8>; common_defs::FRAME_SZ] = unsafe {MaybeUninit::uninit().assume_init()};
    let prot_frame : [u8; common_defs::PROT_SZ*2];

    loop {
        thread::sleep(Duration::from_millis(100));
        // Check for messages
        let r = receiver.try_recv();
        match r {
            Ok(msg) => {
                match msg {
                    messages::ReaderMsg::Terminate => break,
                    messages::ReaderMsg::StartListening => {
                        listen = true;
                        println!("Listening for data...");
                    }
                };
            },
            // Do nothing if there are no message matches
            _ => (),
        };
        // Perform read data?
        if listen {
            let r = p_sock.recv_from(&mut udp_frame);
            match r {
                Ok((sz,_addr)) => {
                    println!("Received {:?} data bytes", sz);
                    split_frame(udp_frame);
                }
                Err(e) => (), //println!("Error or timeout on receive data [{}]", e),
            } 
        }
    }
    println!("UDP Reader exiting");
    thread::sleep(Duration::from_millis(1000));
}

fn split_frame(udp_frame: [MaybeUninit<u8>; common_defs::FRAME_SZ]) {
    protocol::decoder::frame_decode(126, 1, 48000, common_defs::FRAME_SZ*2, udp_frame); 
}