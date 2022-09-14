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

use crate::common::cc_out_defs;

//========================================================================
// Constants
// Round robin sequence for sending CC bytes
// Note 0-6 for CCOBufferIdx 
const RR_CC:  u32 = 6;

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
static mut CC_IDX: u32 = 0;
// Default MOX state
static mut CC_MOX_STATE: bool = false;

// Default array contains the C0 values that define how C1-C4 are defined
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

//========================================================================
// For each field in C2 - C4 we define the bits to set for the number of values for that setting.
// See the enum definitions in common/cc_out_defs for the index list of each field.
// Example: speed has 4 possible values so there are 4 byte values for the indexes 0-3. These are
// relative to the actual field starting bit and not bit 0. The second value is the mask that
// defines where those bits are placed in the byte. It does not define which byte C2 - C4 as that
// is defined by the calling function that sets the bits.

// Speed
static mut CCO_SPEED_B: [u8; 4] = [ 0x00, 0x01, 0x10, 0x11 ];
static mut CCO_SPEED_M: u8 = 0xfc;
// 10MHz ref
static mut CCO_10MHZ_REF_B: [u8; 3] = [ 0x00,0x04,0x08 ];
static mut CCO_10MHZ_REF_M: u8 = 0x3;
// 122MHs ref
static mut CCO_122MHZ_REF_B: [u8; 2] = [ 0x00,0x10 ];
static mut CCO_122MHZ_REF_M: u8 = 0xef;
// Board config
static mut CCO_BOARD_CONFIG_B: [u8; 4] = [ 0x00,0x20,0x40,0x60 ];
static mut CCO_BOARD_CONFIG_M: u8 = 0x9f;
// Mic src
static mut CCO_MIC_SRC_B: [u8; 2] = [ 0x00,0x80 ];
static mut CCO_MIC_SRC_M: u8 = 0x7f;
// Alex attenuator
static mut CCO_ALEX_ATTN_B: [u8; 4] = [ 0x00,0x01,0x10,0x11 ];
static mut CCO_ALEX_ATTN_M: u8 = 0xfc;
// Preamp
static mut CCO_PREAMP_b: [u8; 2] = [ 0x00,0x04 ];
static mut CCO_preamp_m: u8 = 0xfb;
// Alex RX ant
unsigned char cco_rx_ant_b[] = { 0x00,0x20,0x40,0x60 };
unsigned char cco_rx_ant_m = 0x9f;
// Alex RX out
unsigned char cco_alex_rx_out_b[] = { 0x00,0x80 };
unsigned char cco_alex_rx_out_m = 0x7f;
// Alex TX relay
unsigned char cco_alex_tx_rly_b[] = { 0x00,0x01,0x10 };
unsigned char cco_alex_tx_rly_m = 0xfc;
// Duplex
unsigned char cco_duplex_b[] = { 0x00,0x04 };
unsigned char cco_duplex_m = 0xfb;
// No.RX
unsigned char cco_num_rx_b[] = { 0x00,0x08,0x10 };
unsigned char cco_num_rx_m = 0xc7;
// Alex auto
unsigned char cco_alex_auto_b[] = { 0x00,0x40 };
unsigned char cco_alex_auto_m = 0xbf;
// Alex HPF bypass
unsigned char cco_hpf_bypass_b[] = { 0x00,0x20 };
unsigned char cco_hpf_bypass_m = 0xdf;
// LPF Filter selects
unsigned char cco_alex_lpf_30_20_b[] = { 0x00,0x01 };
unsigned char cco_alex_lpf_30_20_m = 0xfe;
unsigned char cco_alex_lpf_60_40_b[] = { 0x00,0x02 };
unsigned char cco_alex_lpf_60_40_m = 0xfd;
unsigned char cco_alex_lpf_80_b[] = { 0x00,0x04 };
unsigned char cco_alex_lpf_80_m = 0xfb;
unsigned char cco_alex_lpf_160_b[] = { 0x00,0x08 };
unsigned char cco_alex_lpf_160_m = 0xf7;
unsigned char cco_alex_lpf_6_b[] = { 0x00,0x10 };
unsigned char cco_alex_lpf_6_m = 0xef;
unsigned char cco_alex_lpf_12_10_b[] = { 0x00,0x20 };
unsigned char cco_alex_lpf_12_10_m = 0xdf;
unsigned char cco_alex_lpf_17_15_b[] = { 0x00,0x40 };
unsigned char cco_alex_lpf_17_15_m = 0xbf;
// HPF Filter selects
unsigned char cco_alex_hpf_13_b[] = { 0x00,0x01 };
unsigned char cco_alex_hpf_13_m = 0xfe;
unsigned char cco_alex_hpf_20_b[] = { 0x00,0x02 };
unsigned char cco_alex_hpf_20_m = 0xfd;
unsigned char cco_alex_hpf_9_5_b[] = { 0x00,0x04 };
unsigned char cco_alex_hpf_9_5_m = 0xfb;
unsigned char cco_alex_hpf_6_5_b[] = { 0x00,0x08 };
unsigned char cco_alex_hpf_6_5_m = 0xf7;
unsigned char cco_alex_hpf_1_5_b[] = { 0x00,0x10 };
unsigned char cco_alex_hpf_1_5_m = 0xef;
