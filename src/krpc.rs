use id::{NodeId};
use routing::{NodeInfo, RoutingTable};

use std::net;
use std::io;

// inspired by https://github.com/divius/rust-dht
pub struct KrpcService {
    id: NodeId,
    table: RoutingTable, // Use Arc<RwLock<NodeInfoStore>>?
    socket: net::UdpSocket,
}

impl KrpcService {
    /// New service with default node table.
    pub fn new(info: NodeInfo) -> io::Result<KrpcService> {
        let table = RoutingTable::new( info.id.clone() );
        let socket = try!( net::UdpSocket::bind(info.address) );

       Ok(KrpcService {
           id: info.id,
           table: table,
           socket: socket,
       })
    }
}
