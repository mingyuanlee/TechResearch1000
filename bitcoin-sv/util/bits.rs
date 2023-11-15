use std::cmp::min;

const LSHIFT_MASK: [u8; 8] = [0xff, 0x7f, 0x3f, 0x1f, 0x0f, 0x07, 0x03, 0x01];
const RSHIFT_MASK: [u8; 8] = [0xff, 0xfE, 0xfc, 0xf8, 0xf0, 0xe0, 0xc0, 0x80];

#[derive(Debug, Default, Clone)]
pub struct Bits {
  pub data: Vec<u8>,
  pub len: usize,
}

impl Bits {
  pub fn new() -> Bits {
    Bits {
      data: vec![],
      len: 0,
    }
  }

  pub fn with_capacity(capacity: usize) -> Bits {
    Bits {
      data: Vec::with_capacity(capacity / 8),
      len: 0,
    }
  }

  // Creates the bits from a slice
  pub fn from_slice(data: &[u8], len: usize) -> Bits {
    let mut vec = data.to_vec();
    let len = min(data.len() * 8, len);
    if len > vec.len() * 8 {
      vec.truncate((len + 7) / 8);
    }
    let rem = (len % 8) as u8;
    if rem != 0 {
      let last = vec.len() - 1;
      vec[last] &= (!((1 << (8_u8 - rem)) - 1)) as u8;
    }
    Bits { data: vec, len }
  }
}