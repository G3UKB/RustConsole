/*
decoder.rs

Module - decoder
Module decoder manages decoding the protocol frame

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
use std::mem::MaybeUninit;

pub fn frame_decode(n_smpls: u32, n_rx: u32, rate: u32, in_sz: usize, udp_frame: [u8; common_defs::PROT_SZ * 2]) {

	/* Decode the incoming data packet
	*
	* Arguments:
	*  n_smpls			--	number of I/Q samples per frame per receiver
	*  n_rx				--	number of receivers
	*  rate				-- 	48000/96000/192000/384000
	* in_sz				--	size of input data buffer
	*  ptr_in_bytes   	--  ptr to the input data
	*/

}