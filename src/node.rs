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

    // can overflow somewhat easily
    pub fn num_bits(&self) -> usize {
        self.num_bytes() * 8
    }

    // i should be zero referenced.
    //  - 0 is most significant bit
    //  - (self.num_bits() - 1) is least significant bit
    pub fn get_bit(&self, i: usize) -> Option<bool> {
        if i >= self.num_bits() {
            None
        } else {
            let byte = self.id[i / 8];
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

/*

Here is the pseudocode for the logic a node uses when updating its routing tree. The node  does this whenever it sees any message (request or reply) from another node.

def see(info: NodeContactInfo) {
    let SID = sender's node ID = info.id
    look up the appropriate bucket for `SID` in the routing tree

    // In what follows, less-recently-updated nodes are placed towards the
    // head of the list, and more-recently-updated ones are placed towards
    // the tail

    if an entry for `SID` exists in the bucket {
        move that entry to the tail of the list.
        and possibly update the IP address / UDP port // TODO: clarify
    } else {
        if the bucket has free space {
            the new entry is inserted at the tail of the list
            the node at the head of the list is contacted.

            if that node fails to respond {
                the corresponding entry is removed from the list
                the new entry is added at the tail.
            } else {
                if the k-bucket can be divided {
                    the bucket is split
                    the new entry is added to the tail of the appropriate bucket.
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
    pub fn see(&mut self, info: NodeInfo) {
        let sid = &info.id;
        let mut bkt = self.buckets.find_bucket(sid);

        if let Some(pos) = bkt.find_id_pos(sid) {
            bkt.move_to_end(pos);
        }

    }

}

struct NodeIdBits<'a> {
    id: &'a NodeId,
    bit: usize,
}

impl<'a> NodeIdBits<'a> {
    fn new(id: &'a NodeId) -> NodeIdBits<'a> {
        NodeIdBits {
            id: id,
            bit: 1 << (id.num_bits() - 1)
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


enum BucketTreeNode {
    Bucket(Bucket),
    Compound(Box<BucketTreeNode>, Box<BucketTreeNode>),
}

impl BucketTreeNode {
    pub fn find_bucket<'a>(&'a mut self, id_bits: &'a NodeId) -> &'a mut Bucket {
        let mut iter = NodeIdBits::new(&id_bits);
        match self.find_bucket_recursive(&mut iter) {
            None => panic!("Got `None` from find_bucket_recursive. Help"),
            Some(b) => b
        }
    }

    fn find_bucket_recursive<'a>(&'a mut self, id_bits: &mut NodeIdBits<'a>) -> Option<&'a mut Bucket> {
        match *self {
            BucketTreeNode::Bucket(ref mut bucket) => Some(bucket),
            BucketTreeNode::Compound(ref mut node0, ref mut node1) => {
                let b = match id_bits.next() {
                    None => panic!("find_bucket_recursive: No bits left in NodeId"),
                    Some(b) => b,
                };

                if b {
                    node1.find_bucket_recursive(id_bits)
                } else {
                    node0.find_bucket_recursive(id_bits)
                }
            },
        }
    }
}


struct BucketTree {
    size: u32,
    root: BucketTreeNode,
}


impl BucketTree {
    // Creates a BucketTree with a single (empty) bucket.
    pub fn new(size: u32) -> BucketTree {
        BucketTree {
            size: size,
            root: BucketTreeNode::Bucket(Bucket::new()), }
    }

    pub fn find_bucket<'a>(&'a mut self, id_bits: &'a NodeId) -> &'a mut Bucket {
        self.root.find_bucket(id_bits)
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


impl Bucket {
    pub fn new() -> Bucket {
        Bucket {
            info: LinkedList::new(),
        }
    }

    pub fn contains_id<'a>(&self, id: &'a NodeId) -> bool {
        self.find_id_pos(id).is_some()
    }

    // Return the index (in the linked list) of the (first) entry containing
    // `id` if it exists
    pub fn find_id_pos<'a>(&self, id: &'a NodeId) -> Option<usize> {
        for (i, contact) in self.info.iter().enumerate() {
            if contact.id == *id {
                return Some(i);
            }
        }
        return None;
    }

    pub fn move_to_end(&mut self, pos: usize) -> bool {
        let mut split = self.info.split_off(pos);
        let x = match split.pop_front() {
            Some(val) => val,
            None => return false,
        };
        split.push_back(x);
        self.info.append(&mut split);
        true
    }

    pub fn len(&self) -> usize {
        self.info.len()
    }

    // TODO: should this method call evict()?
    pub fn insert(&mut self, info: NodeInfo) -> Result<(), &'static str> {
        Err("unimplemented")
    }
}
