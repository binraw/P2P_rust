use libp2p::{
    ping,
    swarm::{Swarm, SwarmEvent, Config},
    tcp,
    yamux,
    noise,
    identity,
    Multiaddr, PeerId, Transport,
};
use futures::StreamExt;
// use std::env;
use libp2p::core::upgrade;

// pub async fn async_ping() -> Result<(), Box<dyn std::error::Error>> {
//     let args: Vec<String> = env::args().collect();

//     // 🔑 Générer une clé et PeerId
//     let local_key = identity::Keypair::generate_ed25519();
//     let local_peer_id = PeerId::from(local_key.public());
//     println!("Mon PeerId: {local_peer_id}");

//     // 🔑 Configurer Nois    
//     let id_keys = identity::Keypair::generate_ed25519();
//     let noise = noise::Config::new(&id_keys).unwrap();
//     let _builder = tcp::tokio::Transport::default()
//         .upgrade(upgrade::Version::V1)
//         .authenticate(noise.clone());

//     // 🔑 Construire transport TCP + Noise + Yamux
//     let transport = tcp::tokio::Transport::default()
//         .upgrade(upgrade::Version::V1)
//         .authenticate(noise.clone())
//         .multiplex(yamux::Config::default())
//         .boxed();

//     // 🔑 Ajouter un Behaviour (ici Ping)
//     let behaviour = ping::Behaviour::default(); 

//     let mut swarm = Swarm::new(transport, behaviour, local_peer_id, Config::with_tokio_executor());

//     if args.len() == 1 {
//         // Mode listener
//         let addr: Multiaddr = "/ip4/127.0.0.1/tcp/0".parse()?;
//         swarm.listen_on(addr)?;
//         println!("Mode: listener");
//     } else {
//         // Mode dialer
//         let target: Multiaddr = args[1].parse()?;
//         swarm.dial(target.clone())?;
//         println!("Mode: dialer → {target}");
//     }

//     // 🔑 Boucle principale
//     loop {
//         match swarm.select_next_some().await {
//             SwarmEvent::NewListenAddr { address, .. } => {
//                 println!("Écoute sur {address}");
//             }
//             SwarmEvent::ConnectionEstablished { peer_id, .. } => {
//                 println!("Connecté à {peer_id}");
//             }
//             SwarmEvent::Behaviour(ping::Event { peer, result, connection }) => {
//                 println!("Ping de {peer:?}: {result:?}");
//                 let _connection = connection.clone();
//             }
//             _ => {}
//         }
//     }
// }

pub async fn build_ping_swarm() -> Swarm<ping::Behaviour> {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    let noise = noise::Config::new(&local_key).unwrap();
    let transport = tcp::tokio::Transport::default()
        .upgrade(upgrade::Version::V1)
        .authenticate(noise)
        .multiplex(yamux::Config::default())
        .boxed();

    let behaviour = ping::Behaviour::default();
    Swarm::new(transport, behaviour, local_peer_id, Config::with_tokio_executor())
}

pub async fn run_ping_once(mut swarm: Swarm<ping::Behaviour>) -> Option<ping::Event> {
    // Attendre un seul événement Ping
    while let Some(event) = swarm.next().await {
        if let SwarmEvent::Behaviour(ping_event) = event {
            return Some(ping_event);
        }
    }
    None
}