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

use crate::common;

const MAX_MSG:  usize = 63;
static mut DATA_OUT: [u8; MAX_MSG] = [0; MAX_MSG];
static mut DATA_IN: [MaybeUninit<u8>; MAX_MSG] = unsafe { MaybeUninit::uninit().assume_init() };
static mut ADDR: option::Option<socket2::SockAddr> = None;

pub fn hw_control_start(receiver : crossbeam_channel::Receiver<common::HWMsg>, p_sock : Arc<socket2::Socket>) {
    thread::spawn(  move || {
        hw_control_run(receiver, &p_sock);
    });
}

pub fn hw_control_run(receiver : crossbeam_channel::Receiver<common::HWMsg>, p_sock : &socket2::Socket) {
    println!("Hardware Control running");
    loop {
        thread::sleep(Duration::from_millis(100));
        // Handle messages
        let r = receiver.try_recv();
        match r {
            Ok(file) => {
                match file {
                    common::HWMsg::Terminate => break,
                    common::HWMsg::DiscoverHw => do_discover(p_sock),
                    common::HWMsg::StartHw =>  do_start(p_sock, false),
                    common::HWMsg::StopHw=>  do_stop(p_sock),
                };
            },
            Err(_error) => continue,
        };
    }
    println!("Hardware Control  exiting");
    thread::sleep(Duration::from_millis(1000));
}

fn do_discover(p_sock : &socket2::Socket) {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(255,255,255,255)), 1024);
    let sock2_addr = socket2::SockAddr::from (addr);

    unsafe {
        DATA_OUT[0] = 0xEF;
        DATA_OUT[1] = 0xFE;
        DATA_OUT[2] = 0x02;
        let r1 = p_sock.send_to(&DATA_OUT, &sock2_addr);
        match r1 {
            Ok(res) => println!("Sent discover sz:{}", res),
            Err(error) => println!("Write error! {}", error),  
        };
        
        let resp = read_response(p_sock, "Discover");
        match resp {
            None => println!("read_response failed"),
            Some(addr) => {
                println!("Addr: {:#?}", addr);
                ADDR =  Some(addr);
            },
        }
    }
}

fn do_start(p_sock : &socket2::Socket, wbs : bool) {
    
    unsafe {
        DATA_OUT[0] = 0xEF;
        DATA_OUT[1] = 0xFE;
        DATA_OUT[2] = 0x04;
        if wbs{
            DATA_OUT[3] = 0x03;
            DATA_OUT[4] = 0x01;
        }
        match &ADDR {
            None => println!("Can't start hardware as the socket address has not been obtained. Run Discover()"),
            Some(addr) => {
                let r = p_sock.send_to(&DATA_OUT, &addr);
                match r {
                    Ok(res) => println!("Sent hardware start sz:{}", res),
                    Err(error) => println!("Start hardware error! {}", error),  
                };
            }
        }
    }
}

fn do_stop(p_sock : &socket2::Socket) {
    
    unsafe {
        DATA_OUT[0] = 0xEF;
        DATA_OUT[1] = 0xFE;
        DATA_OUT[2] = 0x04;
        match &ADDR {
            None => println!("Can't stop hardware as the socket address has not been obtained. Run Discover()"),
            Some(addr) => {
                let r = p_sock.send_to(&DATA_OUT, &addr);
                match r {
                    Ok(res) => println!("Sent hardware stop sz:{}", res),
                    Err(error) => println!("Stop hardware error! {}", error),  
                };
            }
        }
    }
}

fn read_response(p_sock : &socket2::Socket, ann : &str) -> option::Option<socket2::SockAddr>{

    let  mut result : option::Option<socket2::SockAddr> = None;
    unsafe {
        let mut count = 10;
        while count > 0 {
            let r = p_sock.recv_from(&mut DATA_IN);
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