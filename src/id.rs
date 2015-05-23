use std::slice::bytes;

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
