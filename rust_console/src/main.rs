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

/// Entry point for RustConsole SDR application
///
/// # Examples
///
fn main() {
    println!("Starting Rust Console...");

    // Create an instance of the UDPdata manager type
    let mut i_udp = udp::udp_man::UDPdata::new();

    // Initialise UDP module
    // This will run the reader and writer and subordinate modules
    i_udp.udp_init();

    // Temporary code to wait a while then close everything and exit
    thread::sleep(Duration::from_millis(10000));
    println!("Starting shutdown...");
    i_udp.udp_close();

    println!("Rust console closing...");
    thread::sleep(Duration::from_millis(1000));
}

