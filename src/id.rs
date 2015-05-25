use rand::{self, Rng};
use std::fmt::{Debug, Formatter, Error};
use std::slice::bytes;
use std::ops::BitXor;

pub const NODE_ID_BYTES: usize = 20;
pub const NODE_ID_BITS: usize = NODE_ID_BYTES * 8;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct NodeId([u8; NODE_ID_BYTES]);

impl NodeId {
    // generates a random NodeId
    pub fn new() -> NodeId {
        let mut rng = match rand::OsRng::new() {
            Ok(rng) => rng,
            Err(e) => panic!("Error getting random number generator: {}", e),
        };

        let mut a = [0; NODE_ID_BYTES];
        rng.fill_bytes(&mut a);
        NodeId(a)
    }

    pub fn from_bytes<'a>(bytes: &'a [u8]) -> NodeId {
        let mut a = [0; NODE_ID_BYTES];
        bytes::copy_memory(bytes, &mut a);
        NodeId(a)
    }

    // i should be zero referenced.
    //  - 0 is most significant bit
    //  - (NODE_ID_BITS - 1) is least significant bit
    pub fn get_bit(&self, i: usize) -> Option<bool> {
        if i >= NODE_ID_BITS {
            None
        } else {
            let byte = self.0[i / 8];
            let shift: u8 = 1 << (7 - (i % 8));
            Some( (byte & shift) != 0 )
        }
    }

    fn num_prefix_zeroes(&self) -> usize {
        let mut x = 0;
        for i in 0..NODE_ID_BITS {
            if self.get_bit(i) == 0 {
                x += 1;
            }
        }
        x
    }
}

// I would like to convert output to integer, but there's a problem:
// even the default hash (SHA-1) is 20 bytes, so u64 can't store the
// result. Would have to use BigUint, it seems. So this returns
// a NodeId, for now. Will count number of initial zeroes or something
// in this result to figure out the number.
impl BitXor for NodeId {
    type Output = NodeId;

    fn bitxor(self, _rhs: NodeId) -> NodeId {
        let mut n = NodeId::new();
        for i in 0..NODE_ID_BYTES {
            n.0[i] = self.0[i] ^ _rhs.0[i];
        }
        n
    }
}


impl Debug for NodeId {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for x in self.0.iter() {
            try!(write!(f, "{0:02x}", x));
        }
        Ok(())
    }
}

// TODO: I think I should remove this.
struct NodeIdBits<'a> {
    id: &'a NodeId,
    bit: usize,
}

impl<'a> NodeIdBits<'a> {
    fn new(id: &'a NodeId) -> NodeIdBits<'a> {
        NodeIdBits {
            id: id,
            bit: 1 << (NODE_ID_BITS - 1)
        }
    }
}

impl<'a> Iterator for NodeIdBits<'a> {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.id.get_bit(self.bit);
        self.bit += 1;
        curr
    }
}


