use futures::StreamExt;
use libp2p::{mdns, noise, swarm::SwarmEvent, tcp, yamux};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    let mut swarm_obj = libp2p::SwarmBuilder::with_new_identity()
                            .with_tokio()
                            .with_tcp(
                                tcp::Config::default(), 
                                noise::Config::new, 
                                yamux::Config::default
                            )?
                            .with_quic()
                            .with_behaviour(|key| {
                                let mdns =
                                mdns::tokio::Behaviour::new(mdns::Config::default(), key.public().to_peer_id())?;
                                Ok(mdns)
                            })?
                            .build();
    
    swarm_obj.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;
    swarm_obj.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

 
    loop {
        // tokio::select! {
            match swarm_obj.select_next_some().await {
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Listening on local address {:?}", address);
                },
                SwarmEvent::Behaviour(mdns::Event::Discovered(peers)) => {
                    for (peer, addr) in peers {
                        println!("discovered {} {}", peer, addr);
                    }
                },
                SwarmEvent::Behaviour(mdns::Event::Expired(expired)) => {
                    for (peer, addr) in expired {
                        println!("expired {} {}", peer, addr);
                    }
                }
                _ => {}
            }
        // }
    }
    
}