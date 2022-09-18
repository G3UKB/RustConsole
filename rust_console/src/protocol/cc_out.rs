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

use std::sync::Mutex;
use std::sync::MutexGuard;
use crate::common::cc_out_defs:: {CCOSpeed};

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
// For each field in C2 - C4 we define the bits to set for the number of values for that setting.
// See the enum definitions in common/cc_out_defs for the index list of each field.
// Example: speed has 4 possible values so there are 4 byte values for the indexes 0-3. These are
// relative to the actual field starting bit and not bit 0. The second value is the mask that
// defines where those bits are placed in the byte. It does not define which byte C2 - C4 as that
// is defined by the calling function that sets the bits.
// These are read only.

// Speed
static CCO_SPEED_B: [u8; 4] = [ 0x00, 0x01, 0x02, 0x03 ];
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
static CCO_ALEX_ATTN_B: [u8; 4] = [ 0x00,0x01,0x02,0x03 ];
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
static CCO_ALEX_TX_RLY_B: [u8; 3] = [ 0x00,0x01,0x02 ];
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
	// Current index into array
	cc_idx: usize,
	// Default MOX state
	cc_mox_state: bool,
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
			cc_idx: 0,
			cc_mox_state: false,
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

}

pub struct CCDataMutex {
	ccdata_mutex: Mutex<CCData>,
} 

impl CCDataMutex {
	// Create a new instance and initialise the Mutex
	pub fn new() -> CCDataMutex {
		CCDataMutex {
			ccdata_mutex: Mutex::new(CCData::new()),
		}
	}

	// Return the next CC data in sequence
	pub fn cc_out_next_seq(&mut self) -> [u8; 5] {
		
		let mut m = self.ccdata_mutex.lock().unwrap();
		m.cc_el = m.cc_array[m.cc_idx];
		
		// Check for MOX
		if m.cc_idx == 0 {
			if m .cc_mox_state {
				// Need to set the MOX bit
				m.cc_array[0] [0]= m.cc_array[0] [0] | 0x01;
			}
			else {
				// Need to reset the MOX bit
				m.cc_array[0] [0] = m.cc_array[0] [0] & 0xfe;
			}
		}

		// Bump the index
		m.cc_idx = m.cc_idx + 1;
		if m.cc_idx > RR_CC {
			m.cc_idx = 0;
		}

		// Return a copy of the current index array
		return m.cc_el.clone();
	}

	//==============================================================
	// Functions to manipulate fields in the cc_array

	// Get the given byte at the given index in cc_array
	fn cc_get_byte(&self, m: &mut MutexGuard<CCData>, array_idx: usize, byte_idx: usize) -> u8 {
		return m.cc_array[array_idx] [byte_idx];
	}

	// Overwrite the given byte at the given index in cc_array 
	fn cc_put_byte(&self, m: &mut MutexGuard<CCData>, array_idx: usize, byte_idx: usize, b: u8) {
		m.cc_array[array_idx] [byte_idx] = b;
	}

	// Given a target bit setting and the current bit field and mask return the modified field
	fn cc_set_bits(&self, bit_setting: u8, bit_field: u8, bit_mask: u8) -> u8 {
		return (bit_field & bit_mask) | bit_setting;
	}

	// Update the given field in cc_array
	fn cc_update(&mut self, array_idx: usize, byte_idx: usize, bit_setting: u8, bit_field: u8, bit_mask: u8) {
		let mut m = self.ccdata_mutex.lock().unwrap();
		let b: u8 = self.cc_get_byte(&mut m, array_idx, byte_idx);
		let new_b: u8 = self.cc_set_bits(bit_setting, bit_field, bit_mask);
		self.cc_put_byte(&mut m, array_idx, byte_idx, new_b);
	}

	//==============================================================
	// Setting functions for every bit field in cc_array

	// Set/clear the MOX bit
	pub fn cc_mox(&mut self, mox: bool) {
		let mut m = self.ccdata_mutex.lock().unwrap();
		if mox {
			m.cc_mox_state = true;
		} else {
			m.cc_mox_state = false;
		}
	}

	//========================================
	// Configuration settings
	// Set the bandwidth
	pub fn cc_speed(&mut self, speed: CCOSpeed) {
		let setting = CCO_SPEED_B[speed as usize];
		self.cc_update(CCOBufferIdx::BGen as usize, CCOByteIdx::CC1 as usize, setting, setting, CCO_SPEED_M);
	}

	// Set the 10MHz ref source
	pub fn cc_10_ref(&mut self, reference: u8) {
		let setting = CCO_10MHZ_REF_B[reference as usize];
		self.cc_update(CCOBufferIdx::BGen as usize, CCOByteIdx::CC1 as usize, setting, setting, CCO_10MHZ_REF_M);
	}

	//========================================
	// Set sensible initialisation values
	pub fn cc_init(&mut self) {
		self.cc_mox(false);
		self.cc_speed(CCOSpeed::S48kHz);
	}

}
