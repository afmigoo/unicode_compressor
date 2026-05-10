use bitvec::prelude::*;
use base91::{slice_encode, slice_decode};

fn main() {
    let bits = 3;
    let nums = [1, 1, 2, 3, 5, 7, 3, 0];
    let mut bv = bitvec![u8, Lsb0; 0; nums.len() * bits];

    for (i, num) in nums.iter().enumerate() {
        bv[i * bits .. (i + 1) * bits].store_le::<u8>(*num);
    }
    for (i, num) in nums.iter().enumerate() {
        println!("{:?}", bv[i * bits .. (i + 1) * bits].load_le::<u8>());
    }

    println!("{:?}", bv);

    let encoded = slice_encode(&bv.into_vec());
    println!("encoded: {:?}", String::from_utf8_lossy(&encoded));

    let decoded = slice_decode(&encoded);
    let nums_decoded = BitVec::<u8, Lsb0>::from_vec(decoded);
    
    for i in 0..nums_decoded.len() / bits {
        println!("{:?}", nums_decoded[i * bits .. (i + 1) * bits].load_le::<u8>());
    }
}
