use base::{NodeId, NodeInfo, NodeInfoStore};

use std::net;
use std::io;

// inspired by https://github.com/divius/rust-dht
pub struct KrpcService {
    id: NodeId,
    info_store: NodeInfoStore, // Use Arc<RwLock<NodeInfoStore>>?
    socket: net::UdpSocket,
}

impl KrpcService {
    /// New service with default node table.
    pub fn new(info: NodeInfo) -> io::Result<KrpcService> {
        let info_store = NodeInfoStore::new( info.id.clone() );
        let socket = try!( net::UdpSocket::bind(info.address) );

       Ok(KrpcService {
           id: info.id,
           info_store: info_store,
           socket: socket,
       })
    }
}
