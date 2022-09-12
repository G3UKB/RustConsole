/*
mod.rs

Module - udp
Module udp manages udp_socket, udp_reader, udp_writer modules

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


pub mod udp_socket;
pub mod udp_reader;
pub mod udp_writer;
pub mod hw_control;

use crate::common;
use crossbeam_channel::unbounded;

pub struct UDPdata{
    pub r_sender : crossbeam_channel::Sender<common::ReaderMsg>,
    pub r_receiver : crossbeam_channel::Receiver<common::ReaderMsg>,
    pub w_sender : crossbeam_channel::Sender<common::WriterMsg>,
    pub w_receiver : crossbeam_channel::Receiver<common::WriterMsg>,
    pub hw_sender : crossbeam_channel::Sender<common::HWMsg>,
    pub hw_receiver : crossbeam_channel::Receiver<common::HWMsg>,
}

impl UDPdata {
    pub fn new() -> UDPdata {
        let (r_s, r_r) = unbounded();
        let (w_s, w_r) = unbounded();
        let (hw_s, hw_r) = unbounded();
        UDPdata {  
            r_sender : r_s,
            r_receiver : r_r,
            w_sender : w_s,
            w_receiver : w_r,
            hw_sender : hw_s,
            hw_receiver : hw_r,
        }
    }

    pub fn udp_init(&mut self) {
        println!("Initialising UDP threads");
        let mut i_socket = udp_socket::Sockdata::new();
        //i_socket.udp_revert_socket();
        let p_sock = i_socket.udp_sock_ref();

        let arc = p_sock.clone();
        udp_reader::reader_start(self.r_receiver.clone(), arc);

        let arc1 = p_sock.clone();
        udp_writer::writer_start(self.w_receiver.clone(), arc1);

        let arc2 = p_sock.clone();
        hw_control::hw_control_start(self.hw_receiver.clone(), arc2);

        // Test
        self.hw_sender.send(common::HWMsg::DiscoverHw).unwrap();
        self.hw_sender.send(common::HWMsg::StartHw).unwrap();
        self.hw_sender.send(common::HWMsg::StopHw).unwrap();
    }

    pub fn udp_close(&mut self) {
        self.r_sender.send(common::ReaderMsg::Terminate).unwrap();
        self.w_sender.send(common::WriterMsg::Terminate).unwrap();
        self.hw_sender.send(common::HWMsg::Terminate).unwrap();
    }
}
