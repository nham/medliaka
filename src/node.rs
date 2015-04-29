use std::net;
use std::ops::BitXor;
use std::collections::LinkedList;
use rand::{self, Rng};

pub const NODE_ID_BYTES: usize = 20;
pub const NODE_ID_BITS: u32 = (NODE_ID_BYTES as u32) * 8;

pub const BUCKET_SIZE: u32 = 20;

#[derive(Clone, PartialEq, Eq)]
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

//  The routing tree for node with id NodeInfoStore::id
pub struct NodeInfoStore {
    // ID of the node whose store this is.
    id: NodeId,

    buckets: BucketTree,
}

impl NodeInfoStore {
    pub fn new(id: NodeId, bucket_size: u32) -> NodeInfoStore {
        NodeInfoStore {
            id: id,
            buckets: BucketTree::new(bucket_size),
        }
    }

    pub fn new_default(id: NodeId) -> NodeInfoStore {
        NodeInfoStore::new(id, BUCKET_SIZE)
    }

}


struct BucketTree {
    size: u32,
    root: BucketTreeNode,
}

enum BucketTreeNode {
    Bucket(Bucket),
    Compound(Box<BucketTreeNode>, Box<BucketTreeNode>),
}

impl BucketTree {
    // Creates a BucketTree with a single (empty) bucket.
    pub fn new(size: u32) -> BucketTree {
        BucketTree {
            size: size,
            root: BucketTreeNode::Bucket(Bucket::new()),
        }
    }

}


// Stores NodeInfo entries for some range of addresses
struct Bucket {
    // Linked list because each bucket is sorted by how fresh
    // the NodeInfo is, and stale information gets evicted when necessary
    // (to make room for fresh info)
    info: LinkedList<NodeInfo>,
    //last_changed: time::Tm,
}

/*

Here is the logic for updating a node's routing tree when that node sees a any message (request or reply) from another node.


def see(info: ContactInfo) {
    look up the appropriate bucket for the sender's node ID in the routing tree

    // In what follows, less-recently-updated nodes are placed towards the
    // head of the list, and more-recently-updated ones are placed towards
    // the tail

    If the node ID exists in the bucket {
        move that entry to the tail of the list.
        and possibly update the IP address / UDP port // TODO: clarify
    } else {
        if the k-bucket has free space {
            it is inserted at the tail of the list
        } else {
            the node at the head of the list is contacted.

            if it fails to respond {
                the correspond entry is removed from the list
                the new contact is added at the tail.
            } else {
                if the k-bucket can be divided {
                    the bucket is split
                    the new contact is added to the tail of the appropriate bucket.
                    possibly the contacted node's entry is moved to the tail
                } else {
                    // does this need to be done? a response will
                    // result in see() being called again, which will
                    // presumably do this very thing.
                    the contacted node's entry is moved to the tail

                    discard new node (do nothing)
                }
            }
        }

    }

}

*/
impl Bucket {
    pub fn new() -> Bucket {
        Bucket {
            info: LinkedList::new(),
        }
    }

    pub fn contains_id<'a>(&self, id: &'a NodeId) -> bool {
        for contact in self.info.iter() {
            if contact.id == *id {
                return true;
            }
        }
        return false;
    }

    // TODO: should this method call evict()?
    pub fn insert(&mut self, info: NodeInfo) -> Result<(), &'static str> {
        Err("unimplemented")
    }
}
