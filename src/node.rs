use std::net;
use std::ops::BitXor;
use std::collections::LinkedList;
use rand::{self, Rng};

pub const NODE_ID_BYTES: usize = 20;
pub const NODE_ID_BITS: u32 = (NODE_ID_BYTES as u32) * 8;

pub const BUCKET_SIZE: u32 = 20;

#[derive(Clone)]
pub struct NodeId {
    id: Vec<u8>,
}


impl NodeId {
    pub fn new(num_bytes: usize) -> NodeId {
        let v = Vec::with_capacity(num_bytes);
        NodeId { id: v }
    }

    pub fn new_default() -> NodeId {
        NodeId::new(NODE_ID_BYTES)
    }

    // generates a random NodeId
    pub fn new_random(num_bytes: usize) -> NodeId {
        let mut rng = match rand::OsRng::new() {
            Ok(rng) => rng,
            e => panic!("Error getting random number generator: {}"),
        };

        let mut v = Vec::with_capacity(num_bytes);
        rng.fill_bytes(&mut v);
        NodeId { id: v }
    }

    // generates a random NodeId
    pub fn new_random_default() -> NodeId {
        NodeId::new_random(NODE_ID_BYTES)
    }


    pub fn from_bytes<'a>(bytes: &'a [u8]) -> NodeId {
        let mut v = Vec::with_capacity(bytes.len());
        v.push_all(bytes);
        NodeId { id: v }
    }


    pub fn num_bytes(&self) -> usize {
        self.id.len()
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
        let mut n = NodeId::new(self.num_bytes());
        for i in 0..NODE_ID_BYTES {
            n.id[i] = self.id[i] ^ _rhs.id[i];
        }
        n
    }
}


// Whenever a node sends a message to the current node, the current node
// will record the socket address (IP address and port)
#[derive(Clone)]
pub struct NodeInfo {
    pub address: net::SocketAddr,
    pub id: NodeId,
}

// is called Routing Table by some accounts/implementations
// you can also think of this as a contact list. contacts are
// organized by NodeId
pub struct NodeInfoStore {
    // ID of the node whose state this is.
    id: NodeId,

    num_buckets: u32,
    buckets: Vec<Bucket>,
}

impl NodeInfoStore {
    pub fn new(id: NodeId, bucket_size: u32) -> NodeInfoStore {
        let mut buckets = Vec::with_capacity(id.num_bytes() * 8);
        buckets.push(Bucket::new(bucket_size)); // initial bucket

        NodeInfoStore {
            id: id,
            num_buckets: 1,
            buckets: buckets,
        }
    }

    pub fn new_default(id: NodeId) -> NodeInfoStore {
        NodeInfoStore::new(id, BUCKET_SIZE)
    }

}



// Stores NodeInfo entries for some range of addresses
struct Bucket {
    // Linked list because each bucket is sorted by how fresh
    // the NodeInfo is, and stale information gets evicted when necessary
    // (to make room for fresh info)
    info: LinkedList<NodeInfo>,
    size: u32,
    //last_changed: time::Tm,
}

impl Bucket {
    fn new(size: u32) -> Bucket {
        Bucket {
            info: LinkedList::new(),
            size: size,
        }
    }
}
