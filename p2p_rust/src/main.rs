
pub mod utils;

// use utils::ping::*;





// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // let rt = tokio::runtime::Runtime::new()?;
//     // rt.block_on(async_ping())
//     Ok(())
// }



use libp2p::{
    gossipsub::{Gossipsub, MessageAuthenticity, GossipsubConfigBuilder, Topic},
    identity,
    futures::StreamExt,
    mdns::{Mdns, MdnsEvent},
    swarm::{SwarmEvent, NetworkBehaviourEventProcess, NetworkBehaviour, Swarm},
    PeerId, Multiaddr, Transport, tcp::TokioTcpConfig,
};

// 1. Définir le "Behavior" personnalisé pour votre nœud
#[derive(NetworkBehaviour)]
#[behaviour(event_process)]
struct MyBehaviour {
    // Permet la découverte locale des pairs sur le même réseau
    mdns: Mdns,
    // Le protocole de messagerie pour les rooms
    gossipsub: Gossipsub,
    // [Optionnel] Permet aux pairs de s'identifier
    identify: libp2p::identify::Identify,
    
    // Vous pouvez ajouter ici la logique spécifique pour les événements
    #[behaviour(ignore)]
    rooms: Vec<Topic>, // Stockage des topics/rooms auxquels le pair est abonné
}

// 2. Implémenter le traitement des événements
impl NetworkBehaviourEventProcess<MdnsEvent> for MyBehaviour {
    // Quand on trouve ou perd un pair via mDNS
    fn inject_event(&mut self, event: MdnsEvent) {
        if let MdnsEvent::Discovered(list) = event {
            for (peer_id, multiaddr) in list {
                log::info!("mDNS a découvert un pair: {} à {}", peer_id, multiaddr);
                // On peut ajouter le pair à la liste des adresses connues (Kademlia ou autre)
                // Pour l'instant, on se contente de l'afficher.
            }
        }
    }
}

// (Implémenter NetworkBehaviourEventProcess pour GossipsubEvent, etc. pour une gestion complète)
// ...

// Suite de main.rs

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Créer une paire de clés unique pour ce nœud
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    log::info!("ID du pair local: {}", local_peer_id);

    // 1. Créer le Transport (comment les données circulent)
    let transport = TokioTcpConfig::new()
        .nodelay(true)
        .upgrade(libp2p::core::upgrade::Version::V1)
        .authenticate(
            libp2p::noise::NoiseAuthenticated::new(&local_key) // Chiffrement des communications
        )
        .multiplex(libp2p::yamux::YamuxConfig::default()) // Multiplexage sur une seule connexion TCP
        .boxed();

    // 2. Configurer Gossipsub
    let gossipsub_config = GossipsubConfigBuilder::default()
        .build()?;
    let gossipsub = Gossipsub::new(
        MessageAuthenticity::Signed(local_key), // S'assurer que les messages sont signés
        gossipsub_config,
    )?;

    // 3. Configurer mDNS et Identify
    let mdns = Mdns::new(Default::default()).await?;
    let identify = libp2p::identify::Identify::new(
        libp2p::identify::IdentifyConfig::new("/my-p2p-chat/1.0.0".to_string(), local_key.public())
    );

    // 4. Configurer Kademlia (Kademlia doit être initialisé avant le Swarm)
    let mut kademlia = Kademlia::new(
        local_peer_id,
        KademliaStore::new(local_peer_id.clone()),
        KademliaConfig::default(),
    )?;

    // 5. Configurer Request/Response (RR)
    let rr_config = RequestResponseConfig::default();
    let rr_protocols = iter::once((
        ChatProtocol,
        ProtocolSupport::Full,
    ));

    let chat_rr = RequestResponse::new(
        ChatCodec, 
        rr_protocols, 
        rr_config
    );

    // 6. Créer le Swarm (Doit maintenant inclure Kademlia et chat_rr)
    let mut swarm = Swarm::new(
        transport,
        MyBehaviour { 
            mdns, 
            gossipsub, 
            identify, 
            kademlia, // <-- NOUVEAU
            chat_rr,  // <-- NOUVEAU
            rooms: vec![] 
        },
        local_peer_id,
    );

    // 7. Écouter sur une adresse et Bootstrapping
    let addr: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse()?;
    swarm.listen_on(addr)?;
    swarm.behaviour_mut().kademlia.bootstrap().ok(); // Démarre la recherche de pairs

    log::info!("Écoute en cours sur {:?}", swarm.listeners());

    // 8. Boucle de gestion des événements (Cœur ASYNCHRONE)
    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                log::info!("Adresse d'écoute locale: {}", address);
            },
            SwarmEvent::Behaviour(event) => {
                // CETTE LIGNE EST MAINTENANT PLUS RICHE
                // Elle gère mDNS, Gossipsub, Identify, Kademlia, et Request/Response !
                // La logique est dans les implémentations NetworkBehaviourEventProcess.
                log::trace!("Événement du Behavior: {:?}", event);
            },
            _ => {} 
        }
    }
}


let room_name = "general_chat";
let topic = Topic::new(room_name);

if let Err(e) = swarm.behaviour_mut().gossipsub.subscribe(&topic) {
    log::error!("Erreur lors de l'abonnement au topic {}: {}", room_name, e);
} else {
    log::info!("Abonné à la room: {}", room_name);
    swarm.behaviour_mut().rooms.push(topic);
}

let message = format!("{}: Hello everyone!", local_peer_id); // Inclure l'ID de l'expéditeur
let topic = Topic::new("general_chat"); // Le même topic que celui de la room

if let Err(e) = swarm.behaviour_mut().gossipsub.publish(topic, message.as_bytes()) {
    log::error!("Erreur lors de la publication du message: {}", e);
} else {
    log::info!("Message publié dans la room!");
}


// Ajouter ceci à votre fichier, en plus de l'implémentation pour MdnsEvent
impl NetworkBehaviourEventProcess<libp2p::gossipsub::GossipsubEvent> for MyBehaviour {
    fn inject_event(&mut self, event: libp2p::gossipsub::GossipsubEvent) {
        if let libp2p::gossipsub::GossipsubEvent::Message { propagation_source: _, message_id: _, message } = event {
            // Un message est arrivé !
            let topic = message.topic.as_str();
            if let Ok(msg_content) = String::from_utf8(message.data) {
                log::info!("--- MESSAGE REÇU dans la room '{}' ---", topic);
                log::info!("{}", msg_content);
                log::info!("--------------------------------------");
            }
        }
    }
}
// N'oubliez pas l'implémentation pour libp2p::identify::IdentifyEvent
impl NetworkBehaviourEventProcess<libp2p::identify::IdentifyEvent> for MyBehaviour {
    fn inject_event(&mut self, _event: libp2p::identify::IdentifyEvent) {}
}