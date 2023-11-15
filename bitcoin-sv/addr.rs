use std::fmt;
use std::io;
use std::io::{Read, Write};

const MAX_ADDR_COUNT: u64 = 1000;

// Default: generate default values
// PartialEq: implement == and !=
// Eq: don't understand
// Hash: has a way to produce a hash value
// Clone: can produce a duplicate of their value
#[derive(Default, PartialEq, Eq, Hash, Clone)]
pub struct Addr {
  // List of addresses of known nodes
  pub addrs: Vec<NodeAddrEx>,
}

impl Serializable<Addr> for Addr {
  fn read(reader: &mut dyn)
}