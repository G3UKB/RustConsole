/*
cc_out.rs

Module - cc_out
Module cc_out manages encoding the protocol command and control bytes to the hardware

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

use std::sync::Arc;

use crate::common::cc_out_defs;

//========================================================================
// Constants
// Round robin sequence for sending CC bytes
// Note 0-6 for CCOBufferIdx 
const RR_CC:  usize = 6;

//========================================================================
// Enumerations for bit fields in the CC structure
// CC buffer index
enum CCOBufferIdx {
	BGen,
	BRx1TxF,
	BRx1F,
	BRx2F,
	BRx3F,
	BMisc1,
	BMisc2
}

// CC byte index
enum CCOByteIdx {
	CC0,
	CC1,
	CC2,
	CC3,
	CC4
}

//========================================================================
// State variables

// Current indux into array
static mut CC_IDX: usize = 0;
// Default MOX state
static mut CC_MOX_STATE: bool = false;

/* 
static mut CC_ARRAY: [[u8; 5];7] = 
[
	[ 0x00, 0x00, 0x00, 0x00, 0x00 ],
	[ 0x02, 0x00, 0x00, 0x00, 0x00 ],
	[ 0x04, 0x00, 0x00, 0x00, 0x00 ],
	[ 0x06, 0x00, 0x00, 0x00, 0x00 ],
	[ 0x08, 0x00, 0x00, 0x00, 0x00 ],
	[ 0x12, 0x00, 0x00, 0x00, 0x00 ],
	[ 0x14, 0x00, 0x00, 0x00, 0x00 ],
];

// Single row of the array is returned as next in sequence
static mut CC_EL: [u8; 5] = [ 0x00, 0x00, 0x00, 0x00, 0x00 ];
*/

//========================================================================
// For each field in C2 - C4 we define the bits to set for the number of values for that setting.
// See the enum definitions in common/cc_out_defs for the index list of each field.
// Example: speed has 4 possible values so there are 4 byte values for the indexes 0-3. These are
// relative to the actual field starting bit and not bit 0. The second value is the mask that
// defines where those bits are placed in the byte. It does not define which byte C2 - C4 as that
// is defined by the calling function that sets the bits.
// These are read only.

// Speed
static CCO_SPEED_B: [u8; 4] = [ 0x00, 0x01, 0x10, 0x11 ];
static CCO_SPEED_M: u8 = 0xfc;
// 10MHz ref
static CCO_10MHZ_REF_B: [u8; 3] = [ 0x00,0x04,0x08 ];
static CCO_10MHZ_REF_M: u8 = 0x3;
// 122MHs ref
static CCO_122MHZ_REF_B: [u8; 2] = [ 0x00,0x10 ];
static CCO_122MHZ_REF_M: u8 = 0xef;
// Board config
static CCO_BOARD_CONFIG_B: [u8; 4] = [ 0x00,0x20,0x40,0x60 ];
static CCO_BOARD_CONFIG_M: u8 = 0x9f;
// Mic src
static CCO_MIC_SRC_B: [u8; 2] = [ 0x00,0x80 ];
static CCO_MIC_SRC_M: u8 = 0x7f;
// Alex attenuator
static CCO_ALEX_ATTN_B: [u8; 4] = [ 0x00,0x01,0x10,0x11 ];
static CCO_ALEX_ATTN_M: u8 = 0xfc;
// Preamp
static CCO_PREAMP_B: [u8; 2] = [ 0x00,0x04 ];
static CCO_preamp_M: u8 = 0xfb;
// Alex RX ant
static CCO_RX_ANT_B: [u8; 4] = [ 0x00,0x20,0x40,0x60 ];
static CC0_RX_ANT_M: u8 = 0x9f;
// Alex RX out
static CCO_ALEX_RX_OUT_B: [u8; 2] = [ 0x00,0x80 ];
static CCO_ALEX_RX_OUT_M: u8 = 0x7f;
// Alex TX relay
static CCO_ALEX_TX_RLY_B: [u8; 3] = [ 0x00,0x01,0x10 ];
static CCO_ALEX_TX_RLY_M: u8 = 0xfc;
// Duplex
static CCO_DUPLEX_B: [u8; 2] = [ 0x00,0x04 ];
static CCO_DUPLEX_M: u8 = 0xfb;
// No.RX
static CCO_NUM_RX_B: [u8; 3] = [ 0x00,0x08,0x10 ];
static CCO_NUM_RX_M: u8 = 0xc7;
// Alex auto
static CCO_ALEX_AUTO_B: [u8; 2] = [ 0x00,0x40 ];
static CCO_ALEX_AUTO_M: u8 = 0xbf;
// Alex HPF bypass
static CCO_HPF_BYPASS_B: [u8; 2] = [ 0x00,0x20 ];
static CCO_HPF_BYPASS_M: u8 = 0xdf;
// LPF Filter selects
static CCO_ALEX_LPF_30_20_B: [u8; 2] = [ 0x00,0x01 ];
static CCO_ALEX_LPF_30_20_M: u8 = 0xfe;
static CCO_ALEX_LPF_60_40_B: [u8; 2] = [ 0x00,0x02 ];
static CCO_ALEX_LPF_60_40_M: u8 = 0xfd;
static CCO_ALEX_LPF_80_B: [u8; 2] = [ 0x00,0x04 ];
static CCO_ALEX_LPF_80_M: u8 = 0xfb;
static CCO_ALEX_LPF_160_: [u8; 2] = [ 0x00,0x08 ];
static CCO_ALEX_LPF_160_M: u8 = 0xf7;
static CCO_ALEX_LPF_6_B: [u8; 2] = [ 0x00,0x10 ];
static CCO_ALEX_LPF_6_M: u8 = 0xef;
static CCO_ALEX_LPF_12_10_B: [u8; 2] = [ 0x00,0x20 ];
static CCO_ALEX_LPF_12_10_M: u8 = 0xdf;
static CCO_ALEX_LPF_17_15_B: [u8; 2] = [ 0x00,0x40 ];
static CCO_ALEX_LPF_17_15_M: u8 = 0xbf;
// HPF Filter selects
static CCO_ALEX_hpf_13_B: [u8; 2] = [ 0x00,0x01 ];
static CCO_ALEX_hpf_13_M: u8 = 0xfe;
static CCO_ALEX_hpf_20_B: [u8;2] = [ 0x00,0x02 ];
static CCO_ALEX_hpf_20_M: u8 = 0xfd;
static CCO_ALEX_hpf_9_5_B: [u8; 2] = [ 0x00,0x04 ];
static CCO_ALEX_hpf_9_5_M: u8 = 0xfb;
static CCO_ALEX_hpf_6_5_B: [u8; 2] = [ 0x00,0x08 ];
static CCO_ALEX_hpf_6_5_M: u8 = 0xf7;
static CCO_ALEX_hpf_1_5_B: [u8; 2] = [ 0x00,0x10 ];
static CCO_ALEX_hpf_1_5_M: u8 = 0xef;

//========================================================================
// Implementations

// The arrays that are modified by several threads/callers are wrapped in an Arc
// allowing safe sharing.
pub struct CCData{
	// Default array contains the C0 values that define how C1-C4 are defined
	cc_array : [[u8; 5];7],
	// Single row of the array is returned as next in sequence
	cc_el : [u8; 5],
}

// Implementation methods on CCData
impl CCData {
	// Create a new instance and initialise the default arrays
	pub fn new() -> CCData {
		CCData {  
			cc_array: (
				[
					[ 0x00, 0x00, 0x00, 0x00, 0x00 ],
					[ 0x02, 0x00, 0x00, 0x00, 0x00 ],
					[ 0x04, 0x00, 0x00, 0x00, 0x00 ],
					[ 0x06, 0x00, 0x00, 0x00, 0x00 ],
					[ 0x08, 0x00, 0x00, 0x00, 0x00 ],
					[ 0x12, 0x00, 0x00, 0x00, 0x00 ],
					[ 0x14, 0x00, 0x00, 0x00, 0x00 ],
				]
			),
			cc_el: ([ 0x00, 0x00, 0x00, 0x00, 0x00 ]),
		}
	}

	// Return the next CC data in sequence
	pub fn cc_out_next_seq(&mut self) -> [u8; 5] {
		unsafe {
			self.cc_el = self.cc_array[0..4][CC_IDX];
			CC_IDX = CC_IDX + 1;
			if CC_IDX > RR_CC {
				CC_IDX = 0;
			}
		};
		return self.cc_el;
	}
}
