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
use std::mem::MaybeUninit;
use std::sync::Arc;
use std::option;

use socket2;

use crate::common::messages;

const MAX_MSG:  usize = 63;

pub struct HWData {
    p_sock: Arc<socket2::Socket>,
    addr: option::Option<socket2::SockAddr>,
    data_out: [u8; MAX_MSG],
    data_in: [MaybeUninit<u8>; MAX_MSG],
}

impl HWData {
	// Create a new instance and initialise the default data
	pub fn new(p_sock : Arc<socket2::Socket>) -> HWData {
		HWData {
            p_sock: p_sock,
            addr: None,
			data_out: [0; MAX_MSG],
            data_in: unsafe { MaybeUninit::uninit().assume_init()},
		}
	}
/* 
    pub fn hw_control_run(&mut self, receiver : crossbeam_channel::Receiver<messages::HWMsg>, p_sock : &socket2::Socket) {
        loop {
            thread::sleep(Duration::from_millis(100));
            // Handle messages
            let r = receiver.try_recv();
            match r {
                Ok(file) => {
                    match file {
                        messages::HWMsg::Terminate => break,
                        messages::HWMsg::DiscoverHw => self.do_discover(p_sock),
                        messages::HWMsg::StartHw =>  self.do_start(p_sock, false),
                        messages::HWMsg::StopHw=>  self.do_stop(p_sock),
                    };
                },
                Err(_error) => continue,
            };
        }
    }
*/
    pub fn do_discover(&mut self) {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(255,255,255,255)), 1024);
        let sock2_addr = socket2::SockAddr::from (addr);
    
        unsafe {
            self.data_out[0] = 0xEF;
            self.data_out[1] = 0xFE;
            self.data_out[2] = 0x02;
            let r1 = self.p_sock.send_to(&self.data_out, &sock2_addr);
            match r1 {
                Ok(res) => println!("Sent discover sz:{}", res),
                Err(error) => println!("Write error! {}", error),  
            };
            
            let resp = self.read_response("Discover");
            match resp {
                None => println!("read_response failed"),
                Some(addr) => {
                    println!("Addr: {:#?}", addr);
                    self.addr =  Some(addr);
                },
            }
        }
    }
    
    pub fn do_start(&mut self, wbs : bool) {
        
        unsafe {
            self.data_out[0] = 0xEF;
            self.data_out[1] = 0xFE;
            self.data_out[2] = 0x04;
            if wbs{
                self.data_out[3] = 0x03;
                self.data_out[4] = 0x01;
            }
            match &self.addr {
                None => println!("Can't start hardware as the socket address has not been obtained. Run Discover()"),
                Some(addr) => {
                    let r = self.p_sock.send_to(&self.data_out, &addr);
                    match r {
                        Ok(res) => println!("Sent hardware start sz:{}", res),
                        Err(error) => println!("Start hardware error! {}", error),  
                    };
                }
            }
        }
    }
    
    pub fn do_stop(&mut self) {
        
        unsafe {
            self.data_out[0] = 0xEF;
            self.data_out[1] = 0xFE;
            self.data_out[2] = 0x04;
            match &self.addr {
                None => println!("Can't stop hardware as the socket address has not been obtained. Run Discover()"),
                Some(addr) => {
                    let r = self.p_sock.send_to(&self.data_out, &addr);
                    match r {
                        Ok(res) => println!("Sent hardware stop sz:{}", res),
                        Err(error) => println!("Stop hardware error! {}", error),  
                    };
                }
            }
        }
    }
    
    fn read_response(&mut self, ann : &str) -> option::Option<socket2::SockAddr>{
    
        let  mut result : option::Option<socket2::SockAddr> = None;
        unsafe {
            let mut count = 10;
            while count > 0 {
                let r = self.p_sock.recv_from(&mut self.data_in);
                match r {
                    Ok(res) => {
                        if res.0 > 0 {
                            println!("{} response sz:{}", ann, res.0);
                        result = Some(res.1);
                            break;       
                        } else {
                            count = count-1;
                            if count <= 0 {
                                println!("Timeout: Failed to read after 10 attempts!");
                                break;
                            }
                            continue;
                        };
                    },
                    Err(error) => {
                        count = count-1;
                        if count <= 0 {
                            println!("Error: Failed to read after 10 attempts! {}", error);
                            break;
                        }
                        continue;  
                    }
                };
                   
            };
        };
        return result;
    }
}

/*
//=============================================================================================
// Entry point to start thread
pub fn hw_control_start(receiver : crossbeam_channel::Receiver<messages::HWMsg>, p_sock : Arc<socket2::Socket>) {
    thread::spawn(  move || {
        hw_control_init(receiver, &p_sock);
    });
}

// Create and initialise the manager instance.
// Thread exits when loop in hw_control_run() exits.
pub fn hw_control_init(receiver : crossbeam_channel::Receiver<messages::HWMsg>, p_sock : &socket2::Socket) {
    println!("Hardware Control running");
    let mut hw_control = HWData::new();
    hw_control.hw_control_run(receiver, p_sock);
    println!("Hardware Control  exiting");
    thread::sleep(Duration::from_millis(1000));
}
*/
