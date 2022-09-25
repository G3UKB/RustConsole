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

//==================================================================================
// Runtime object for thread
pub struct UDPRData<'a>{
    receiver : crossbeam_channel::Receiver<messages::ReaderMsg>,
	p_sock :  &'a socket2::Socket,
    p_addr :  &'a socket2::SockAddr,
    udp_frame : [MaybeUninit<u8>; common_defs::FRAME_SZ],
    prot_frame : [u8; common_defs::PROT_SZ*2],
    //pub i_cc: protocol::cc_in::CCDataMutex,
    pub i_seq: protocol::seq_in::SeqData,
    listen: bool,
}

// Implementation methods on UDPRData
impl UDPRData<'_> {
	// Create a new instance and initialise the default arrays
	pub fn new<'a>(receiver : crossbeam_channel::Receiver<messages::ReaderMsg>, p_sock : &'a socket2::Socket, p_addr : &'a socket2::SockAddr) -> UDPRData<'a> {
        // Create an instance of the cc_in type
        //let i_cc = protocol::cc_in::CCDataMutex::new();
        // Create an instance of the sequence type
        let i_seq = protocol::seq_in::SeqData::new();

		UDPRData {
            receiver: receiver,
			p_sock: p_sock,
            p_addr: p_addr,
            udp_frame: unsafe {MaybeUninit::uninit().assume_init()},
            prot_frame: [0; common_defs::PROT_SZ*2],
            //i_cc: i_cc,
            i_seq: i_seq,
            listen: false,
		}
	}

    // Run loop for reader
    pub fn reader_run(&mut self) {
        loop {
            thread::sleep(Duration::from_millis(100));
            // Check for messages
            let r = self.receiver.try_recv();
            match r {
                Ok(msg) => {
                    match msg {
                        messages::ReaderMsg::Terminate => break,
                        messages::ReaderMsg::StartListening => {
                            self.listen = true;
                            println!("Listening for data...");
                        }
                    };
                },
                // Do nothing if there are no message matches
                _ => (),
            };
            // Check for read data
            if self.listen {
                let r = self.p_sock.recv_from(&mut self.udp_frame);
                match r {
                    Ok((_sz,_addr)) => {
                        //println!("Received {:?} data bytes", _sz);
                        self.split_frame();
                    }
                    Err(_e) => (), //println!("Error or timeout on receive data [{}]", e),
                } 
            }
        }
    }

    // Split frame into protocol fields and data content and decode
    fn split_frame(&mut self) {
        // TBD
        protocol::decoder::frame_decode(126, 1, 48000, common_defs::FRAME_SZ*2, self.udp_frame);
    }

}

//==================================================================================
// Thread startup

pub fn reader_start(receiver : crossbeam_channel::Receiver<messages::ReaderMsg>, p_sock : Arc<socket2::Socket>, p_addr : Arc<socket2::SockAddr>) {
    thread::spawn(  move || {
        reader_run(receiver, &p_sock, &p_addr);
    });
}

fn reader_run(receiver : crossbeam_channel::Receiver<messages::ReaderMsg>, p_sock : &socket2::Socket, p_addr : &socket2::SockAddr) {
    println!("UDP Reader running");

    // Instantiate the runtime object
    let mut i_reader = UDPRData::new(receiver,  p_sock,  p_addr);

    // Exits when the reader loop exits
    i_reader.reader_run();

    println!("UDP Reader exiting");
    thread::sleep(Duration::from_millis(1000));
}
