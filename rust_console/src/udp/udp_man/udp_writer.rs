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

use std::thread;
use std::time::Duration;
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
    pub i_seq: protocol::seq_man::SeqData,
}

// Implementation methods on CCData
impl UDPWData {
	// Create a new instance and initialise the default arrays
	//pub fn new(p_sock : Arc<socket2::Socket>, i_seq: Arc<protocol::seq_man::SeqData>, i_cc: Arc<protocol::cc_out::CCDataMutex>) -> UDPWData {
    pub fn new(p_sock : Arc<socket2::Socket>, i_seq: protocol::seq_man::SeqData, i_cc: protocol::cc_out::CCDataMutex) -> UDPWData {
    //pub fn new(p_sock : Arc<socket2::Socket>) -> UDPWData {
		UDPWData {
			sock2: p_sock,
            udp_frame: [0; common_defs::FRAME_SZ],
            prot_frame: [0; common_defs::PROT_SZ*2],
            i_cc: i_cc,
            i_seq: i_seq,
		}
	}

    /*
	*	<0xEFFE><0x01><End Point><Sequence Number>< 2 x HPSDR frames>
	*	Where:
	*		End point = 1 byte[0x02 – representing USB EP2]
	*		Sequence Number = 4 bytes[32 bit unsigned]
	*		HPSDR data = 1024 bytes[2 x 512 byte USB format frames]
	*
	*	The following fields are merged :
	*		metis_header
	*		out_seq		-- next output sequence number to use
	*		cc_out 		-- round robin control bytes
	*		usb_header +
	*		proc_data	-- 2 frames worth of USB format frames
	*
	*	Data is encoded into the packet buffer
	*/
    fn encode(&mut self) {

        // Encode header
        self.prot_frame[0] = 0xef;
        self.prot_frame[1] = 0xfe;
        self.prot_frame[2] = common_defs::DATA_PKT;
        self.prot_frame[3] = common_defs::EP2;

        // Encode sequence number
        let next_cc = self.i_cc.cc_out_next_seq();
        let next_seq = self.i_seq.next_ep2_seq();
    }
}
