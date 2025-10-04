// use futures::StreamExt;
use futures::StreamExt;
use libp2p::{
    core::upgrade,
    identity,
    noise,
    tcp,
    yamux,
    ping,
    swarm::{Swarm, SwarmEvent, Config},
    Multiaddr, PeerId, Transport,
};
// mod utils;   
// use utils::chat::*;
// use utils::chat::chat;
// use utils::chat::chat_dialer;



use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async_main())
}

// fn send_message(swarm: &mut Swarm<Behaviour, transport, local_peer_id>, message: &str) -> Result<(), Box<dyn std::error::Error>> {
//     let message = swarm.behaviour_mut().send_message(message);
//     Ok(())
// }

// fn receive_message(swarm: &mut Swarm<Behaviour, transport, local_peer_id>) -> Result<String, Box<dyn std::error::Error>> {
//     let message = swarm.behaviour_mut().receive_message().unwrap();
//     Ok(message)
// }

async fn async_main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // 🔑 Générer une clé et PeerId
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Mon PeerId: {local_peer_id}");

    // 🔑 Configurer Nois    
    let id_keys = identity::Keypair::generate_ed25519();
    let noise = noise::Config::new(&id_keys).unwrap();
    let _builder = tcp::tokio::Transport::default()
        .upgrade(upgrade::Version::V1)
        .authenticate(noise.clone());

    // 🔑 Construire transport TCP + Noise + Yamux
    let transport = tcp::tokio::Transport::default()
        .upgrade(upgrade::Version::V1)
        .authenticate(noise.clone())
        .multiplex(yamux::Config::default())
        .boxed();

    // 🔑 Ajouter un Behaviour (ici Ping)
    let behaviour = ping::Behaviour::default(); 

    // let chat = chat::Behaviour::default();


    let mut swarm = Swarm::new(transport, behaviour, local_peer_id, Config::with_tokio_executor());

    if args.len() == 1 {
        // Mode listener
        let addr: Multiaddr = "/ip4/127.0.0.1/tcp/0".parse()?;
        swarm.listen_on(addr)?;
        println!("Mode: listener");
    } else {
        // Mode dialer
        let target: Multiaddr = args[1].parse()?;
        swarm.dial(target.clone())?;
        println!("Mode: dialer → {target}");
    }

    // 🔑 Boucle principale
    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Écoute sur {address}");
            }
            SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                println!("Connecté à {peer_id}");
            }
            SwarmEvent::Behaviour(ping::Event { peer, result, connection }) => {
                println!("Ping de {peer:?}: {result:?}");
                let _connection = connection.clone();
            }
            // SwarmEvent::Behaviour(chat::Event { peer, message }) => {
            //     println!("Message de {peer:?}: {message}");
            // }
            _ => {}
        }
    }
}


