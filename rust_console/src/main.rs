/*
main.rs

Entry module for the RustConsole SDR application

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

pub mod udp;
pub mod common;
pub mod protocol;
use common::cc_out_defs:: {CCOSpeed};

/// Entry point for RustConsole SDR application
///
/// # Examples
///
fn main() {
    println!("Starting RustConsole...");

    // Create an instance of the UDPdata type
    let mut i_udp = udp::udp_man::UDPdata::new();
    // Announce udp module and initialise it
    i_udp.udp_init();   // This will run the reader and writer threads

    // Temporary code to wait a while then close everything and exit
    thread::sleep(Duration::from_millis(5000));
    udp::udp_man::UDPdata::udp_close(&mut i_udp);
    thread::sleep(Duration::from_millis(1000));

    // More temp code to test cc
    let mut i_cc = protocol::cc_out::CCDataMutex::new();
    i_cc.cc_speed(CCOSpeed::S96kHz);
    println!("{:#02x?}", i_cc.cc_out_next_seq());
    println!("{:#02x?}", i_cc.cc_out_next_seq());
    println!("{:#02x?}", i_cc.cc_out_next_seq());
    println!("{:#02x?}", i_cc.cc_out_next_seq());
    println!("{:#02x?}", i_cc.cc_out_next_seq());
    println!("{:#02x?}", i_cc.cc_out_next_seq());
    println!("{:#02x?}", i_cc.cc_out_next_seq());

}
