use std::net;
use std::ops::BitXor;
use std::collections::LinkedList;
use rand::{self, Rng};

pub const BUCKET_SIZE: u32 = 20;

// NODE_ID_BYTES is used as an index and to create fixed-length arrays
// so it is most convenient for it to be usize. Even though it should
// probably be u32.
pub const NODE_ID_BYTES: usize = 20;
pub const NODE_ID_BITS: u32 = (NODE_ID_BYTES as u32) * 8;

type NodeIdArray = [u8; NODE_ID_BYTES];

#[derive(Clone)]
pub struct NodeId {
    id: NodeIdArray,
}

impl NodeId {
    pub fn new() -> NodeId {
        NodeId { id: [0; NODE_ID_BYTES] }
    }

    // generates a random NodeId
    pub fn new_random() -> NodeId {
        let mut rng = match rand::OsRng::new() {
            Ok(rng) => rng,
            e => panic!("Error getting random number generator: {}"),
        };

        let mut x: NodeIdArray = [0; NODE_ID_BYTES];
        rng.fill_bytes(&mut x);
        NodeId { id: x }
    }

    pub fn from_bytes<'a>(id: &'a [u8]) -> NodeId {
        assert_eq!(id.len(), NODE_ID_BYTES);

        // There's probably some better way to do this, but I'm blanking
        // on it right now.
        let mut x: NodeIdArray = [0; NODE_ID_BYTES];
        for i in 0..NODE_ID_BYTES {
            x[i] = id[i]
        }

        NodeId { id: x }
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
            n.id[i] = self.id[i] ^ _rhs.id[i];
        }
        n
    }
}

// is called Routing Table by some accounts/implementations
// you can also think of this as a contact list. contacts are
// organized by NodeId
pub struct NodeInfoStore {
    // ID of the node whose state this is.
    id: NodeId,

    num_buckets: u32,
    buckets: LinkedList<Bucket>,
}

impl NodeInfoStore {
    // create a NodeInfoStore out of a NodeId, using default values for
    // bucket size and number of bits for hashes
    pub fn new(id: NodeId) -> NodeInfoStore {
        NodeInfoStore::with(id, BUCKET_SIZE, NODE_ID_BITS)
    }

    pub fn with(id: NodeId, bucket_size: u32, hash_size: u32) -> NodeInfoStore{
        let mut buckets = LinkedList::new();
        buckets.push_back(Bucket::new(bucket_size)); // initial bucket

        NodeInfoStore {
            id: id,
            num_buckets: 0,
            buckets: buckets,
        }
    }
}

// Whenever a node sends a message to the current node, the current node
// will record the socket address (IP address and port)
#[derive(Clone)]
pub struct NodeInfo {
    pub address: net::SocketAddr,
    pub id: NodeId,
}



// Stores NodeInfo for some slice of the hash space, usually
// 2^i <= x < 2^{i+1} for some int i such that 0 \leq i < NODE_ID_BITS_SIZE
//
// Each bucket can store up to BUCKET_SIZE entries.
//
// Has a least-recently seen eviction policy, except live nodes are
// never evicted.
//
// The current size of the Bucket is `size`
struct Bucket {
    info: Vec<NodeInfo>,
    size: u32,
}

impl Bucket {
    fn new(size: u32) -> Bucket {
        Bucket {
            info: Vec::with_capacity(size as usize),
            size: size,
        }
    }
}
