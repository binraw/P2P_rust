use libp2p::swarm::NetworkBehaviour;

use libp2p::{
    kad::{
        Behaviour as Kademlia,           // Le type Kademlia est nommé Behaviour
        Event as KademliaEvent,          // Le type KademliaEvent est nommé Event
    },
    request_response::{
        Behaviour as RequestResponse,    // Le type RequestResponse est nommé Behaviour
        Event as RequestResponseEvent,   // Le type RequestResponseEvent est nommé Event
    },
};
use crate::utils::chat::{ChatCodec, ChatRequest, ChatResponse};

use libp2p::{
    mdns::Behaviour as Mdns,
    gossipsub::{Behaviour as Gossipsub, Topic},
    identify::Behaviour as Identify,

};
// use libp2p::request_response;
use libp2p::kad::store::MemoryStore;
use libp2p::gossipsub::TopicHash;
 use crate::utils::chat::Message;



#[derive(NetworkBehaviour)]
struct MyBehaviour {
    mdns: Mdns,
    gossipsub: Gossipsub,
    identify: Identify,
    
    // NOUVEAU : Kademlia pour la découverte globale
    #[behaviour(event_process)]
    kademlia: Kademlia<MemoryStore>,
    
    // NOUVEAU : Request/Response pour les messages point-à-point (P2P)
    #[behaviour(event_process)]
    chat_rr: RequestResponse<ChatCodec>,
    
    #[behaviour(ignore)]
    rooms: Vec<TopicHash>,
}

// ...
// N'oubliez pas l'implémentation NetworkBehaviourEventProcess pour les nouveaux événements :

impl NetworkBehaviourEventProcess<KademliaEvent> for MyBehaviour {
    fn inject_event(&mut self, event: KademliaEvent) {
        // Logique de gestion de Kademlia (ex: Pair trouvé)
        // Vous traiterez ici les résultats des recherches ou du bootstrapping
    }
}

impl NetworkBehaviourEventProcess<RequestResponseEvent<ChatRequest, ChatResponse>> for MyBehaviour {
    fn inject_event(&mut self, event: RequestResponseEvent<ChatRequest, ChatResponse>) {
        match event {
            RequestResponseEvent::Message { peer, message: request_response::Message::Request { request_id, request, channel } } => {
                log::info!("RR - Requête reçue de {} : {:?}", peer, request.0);
                
                // Ici, vous désérialisez 'request.0' en votre enum Message
                // Puis vous construisez et envoyez une réponse via 'self.chat_rr.send_response(channel, response)'
                
                // Exemple d'envoi de réponse simple (Pong)
                let response_data = "PONG".as_bytes().to_vec();
                if let Err(e) = self.chat_rr.send_response(channel, ChatResponse(response_data)) {
                    log::error!("Erreur lors de l'envoi de la réponse RR: {}", e);
                }
            },
            RequestResponseEvent::Message { peer, message: request_response::Message::Response { request_id, response } } => {
                log::info!("RR - Réponse reçue de {} pour la requête {:?} : {:?}", peer, request_id, response.0);
                // Traiter la réponse (ex: vérifier le Pong)
            },
            _ => {}
        }
    }
}