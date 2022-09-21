/*
encoder.rs

Module - encoder
Module encoder manages encoding the protocol frame

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

use crate::common::common_defs;
use crate::protocol;

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
pub fn encode(  i_seq: &mut protocol::seq_out::SeqData, 
                i_cc: &mut protocol::cc_out::CCDataMutex, 
                udp_frame: &mut [u8; common_defs::FRAME_SZ], 
                prot_frame: &mut [u8; common_defs::PROT_SZ*2]) {

    // Encode header
    prot_frame[0] = 0xef;
    prot_frame[1] = 0xfe;
    prot_frame[2] = common_defs::DATA_PKT;
    prot_frame[3] = common_defs::EP2;

    // Encode sequence number
    let next_seq = i_seq.next_ep2_seq();

    // Encode command and control bytes
    let next_cc = i_cc.cc_out_next_seq();
    
}

