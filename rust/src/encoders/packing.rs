use bitvec::prelude::*;
use base91::{slice_encode, slice_decode};

use super::errors::Error;

pub fn pack_int(payload: &Vec<u16>, bits: usize) -> Result<Vec<u8>, Error> {
    assert!(bits <= 16, "bits must be <= 16");
    let mut packed_bits = bitvec![u8, Lsb0; 0; 4 + (payload.len() + 1) * bits];
    // scheme of packing
    // 0000 [0..0] .. [0..0] .. 0..0
    // ^     ^                  ^
    // |     |                  end of stream marker: 0x`bits`
    // |     `payload.len()` chunks of `bits` bits
    // first 4 bits contain `bits` value
    packed_bits[0..4].store_le::<u16>(bits as u16);

    let bin_base: u16 = 2;
    let max_num = bin_base.pow(bits as u32) - 1;

    for (i, num) in payload.iter().enumerate() {
        if *num > max_num {
            return Err(Error::BitPackOverflow(*num, bits));
        }
        packed_bits[4 + i * bits .. 4 + (i + 1) * bits].store_le::<u16>(*num);
    }
    // end-of-stream marker
    packed_bits[4 + payload.len() * bits .. 4 + (payload.len() + 1) * bits].store_le::<u16>(0);

    // eprintln!("packed bits: {:?}", &packed_bits);
    Ok(packed_bits.into_vec())
}

pub fn unpack_int(packed: Vec<u8>) -> Vec<u16> {
    let packed_bits = BitVec::<u8, Lsb0>::from_vec(packed);
    let mut payload = Vec::new();

    let bits = packed_bits[0..4].load_le::<u16>() as usize;

    for i in 0..packed_bits.len() / bits {
        let num = packed_bits[4 + i * bits .. 4 + (i + 1) * bits].load_le::<u16>();
        match num {
            0 => break,
            _ => payload.push(num),
        }
    }

    // eprintln!("unpacked bits: {:?}", &packed_bits);
    payload
}

pub fn base91_encode(payload: &Vec<u8>) -> String {
    String::from_utf8_lossy(&slice_encode(payload)).to_string()
}

pub fn base91_decode(payload: &str) -> Vec<u8> {
    slice_decode(payload.as_bytes())
}
