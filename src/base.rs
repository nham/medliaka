use std::net;
use rand::{self, Rng};

pub const KBUCKET_SIZE: u32 = 20;

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
    // generates a random NodeId
    pub fn new() -> NodeId {
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

    pub fn to_array(&self) -> [u8; NODE_ID_BYTES] {
        self.id
    }
}

pub struct NodeInfoStore {
    // ID of the node whose state this is.
    id: NodeId,

    // `num_hash_bits` is length of `buckets`
    num_hash_bits: u32,
    buckets: Vec<KBucket>,
}

impl NodeInfoStore {
    // create a NodeInfoStore out of a NodeId, using default values for
    // bucket size and number of bits for hashes
    pub fn new(id: NodeId) -> NodeInfoStore {
        NodeInfoStore::with(id, KBUCKET_SIZE, NODE_ID_BITS)
    }

    pub fn with(id: NodeId, bucket_size: u32, hash_size: u32) -> NodeInfoStore{
        let mut v = Vec::with_capacity(hash_size as usize);
        for _ in 0..hash_size {
            v.push(KBucket::new(bucket_size));
        }

        NodeInfoStore {
            id: id,
            num_hash_bits: hash_size,
            buckets: v,
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
// Each bucket can store up to KBUCKET_SIZE entries. 
//
// Has a least-recently seen eviction policy, except live nodes are
// never evicted.
//
// The current size of the KBucket is `size`
struct KBucket {
    info: Vec<NodeInfo>,
    size: u32,
}

impl KBucket {
    fn new(size: u32) -> KBucket {
        KBucket {
            info: Vec::with_capacity(size as usize),
            size: size,
        }
    }
}
