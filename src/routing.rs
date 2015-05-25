use id::{NodeId, NODE_ID_BITS};

use std::collections::LinkedList;
use std::net;

// Whenever a node sends a message to the current node, the current node
// will record the socket address (IP address and port)
#[derive(Clone)]
pub struct NodeInfo {
    pub address: net::SocketAddr,
    pub id: NodeId,
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

    pub fn len(&self) -> u32 {
        self.info.len() as u32
    }

    // TODO: should this method call evict()?
    pub fn insert(&mut self, info: NodeInfo) -> Result<(), &'static str> {
        Err("unimplemented")
    }
}



pub const BUCKET_SIZE: u32 = 20;

//  The routing table for a node. Stores NodeInfo entries in buckets.
pub struct RoutingTable {
    // ID of the node whose store this is.
    id: NodeId,

    buckets: Vec<Bucket>,
}

impl RoutingTable {
    pub fn new(id: NodeId) -> RoutingTable {
        let mut buckets = vec![];
        for _ in 0..NODE_ID_BITS {
            buckets.push(vec![]);
        }

        RoutingTable {
            id: id,
            buckets: buckets,
        }
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
        } else {
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
        } else {
            if bkt.len() == self.bucket_size() {

            }

        }
    }

    fn bucket_index(&self, id: NodeId) -> usize {
        (id ^ self.id).num_prefix_zeroes()
    }

}

