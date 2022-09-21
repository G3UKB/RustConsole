/*
udp_writer.rs

Module - udp_writer
Manages write data over UDP to the SDR hardware

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

use socket2;
use std::sync::Arc;

use crate::common::common_defs;
use crate::protocol;

pub struct UDPWData{
	sock2 : Arc<socket2::Socket>,
    udp_frame : [u8; common_defs::FRAME_SZ],
    prot_frame : [u8; common_defs::PROT_SZ*2],
    //pub i_cc: Arc<protocol::cc_out::CCDataMutex>,
    //pub i_seq: Arc<protocol::seq_man::SeqData>,
    pub i_cc: protocol::cc_out::CCDataMutex,
    pub i_seq: protocol::seq_out::SeqData,
}

// Implementation methods on CCData
impl UDPWData {
	// Create a new instance and initialise the default arrays
	pub fn new(p_sock : Arc<socket2::Socket>) -> UDPWData {
        // Create an instance of the cc_out type
        let i_cc = protocol::cc_out::CCDataMutex::new();
        // Create an instance of the sequence type
        let i_seq = protocol::seq_out::SeqData::new();

		UDPWData {
			sock2: p_sock,
            udp_frame: [0; common_defs::FRAME_SZ],
            prot_frame: [0; common_defs::PROT_SZ*2],
            i_cc: i_cc,
            i_seq: i_seq,
		}
	}

    pub fn prime(&mut self) {
        protocol::encoder::encode(&mut self.i_seq, &mut self.i_cc, &mut self.udp_frame, &mut self.prot_frame);
    }
}
