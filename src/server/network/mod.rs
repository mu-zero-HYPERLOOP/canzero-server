use std::sync::{atomic::AtomicU32, Arc};

use tokio::sync::RwLock;

use self::node::NetworkNode;

pub mod node;


pub struct Network {
    nodes : Arc<RwLock<Vec<(u32,Arc<NetworkNode>)>>>,
    id_acc : AtomicU32,
}

impl Network {
    pub fn new() -> Self {
        Self {
            nodes : Arc::new(RwLock::new(vec![])),
            id_acc : AtomicU32::new(0),
        }
    }

    pub async fn start(&self, node : NetworkNode) {
        let nodes = self.nodes.clone();
        let node_id = self.id_acc.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let node = Arc::new(node);
        nodes.write().await.push((node_id, node.clone()));
        tokio::spawn(async move {
            loop {
                let Some(frame) = node.recv().await else {
                    break;
                };

                for (id, node) in nodes.read().await.iter() {
                    if *id != node_id { // ignore loop back!
                        if let Err(err) = node.send(&frame).await {
                            eprintln!("{err:?}");
                        };
                    }
                }
            }
            // remove node
            let mut nodes_lock = nodes.write().await;
            let Some(node_pos) = nodes_lock.iter().position(|(id,_)| *id == node_id) else {
                return;
            };
            nodes_lock.remove(node_pos);
            match node.as_ref() {
                NetworkNode::SocketCanNode(_) => println!("\u{1b}[31mShutdown socketcan connection\u{1b}[0m"),
                NetworkNode::TcpCanNode(tcp) => println!("\u{1b}[31mShutdown {}\u{1b}[0m", tcp.addr().await),
            };
        });
    }

}

