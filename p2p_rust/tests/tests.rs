use p2p_rust::utils::chat::*;
use futures::io::Cursor;
use p2p_rust::utils::ping::*;
use libp2p::Multiaddr;
use libp2p::swarm::{SwarmEvent};
use futures::StreamExt;
use libp2p::ping;
use tokio::time::Duration;


#[tokio::test]
async fn test_chat_codec_roundtrip() {
    // Crée un codec
    let mut codec = ChatCodec;

    // Message à envoyer
    let request = ChatRequest(b"Hello world".to_vec());

    // Buffer pour écrire les données
    let mut write_buffer = Vec::new();

    // ----- Écriture -----
    {
        let mut cursor = Cursor::new(&mut write_buffer);
        codec
            .write_request(&ChatProtocol, &mut cursor, request.clone())
            .unwrap();
    }

    // ----- Lecture -----
    {
        let mut read_cursor = Cursor::new(&write_buffer);
        let received = codec
            .read_request(&ChatProtocol, &mut read_cursor)
            .unwrap();

        // Vérifie que le message reçu == celui envoyé
        assert_eq!(received.0, request.0);
    }
}




#[tokio::test]
async fn test_ping_between_two_nodes() {
    // 🧱 Construire deux swarms
    let mut node1 = build_ping_swarm().await;
    let mut node2 = build_ping_swarm().await;

    // Node1 écoute sur une adresse locale
    let addr: Multiaddr = "/ip4/127.0.0.1/tcp/0".parse().unwrap();
    node1.listen_on(addr).unwrap();

    // Récupérer l'adresse effective (port attribué)
    let listener_addr = loop {
        if let Some(SwarmEvent::NewListenAddr { address, .. }) = node1.next().await {
            break address;
        }
    };

    // Node2 se connecte à Node1
    node2.dial(listener_addr.clone()).unwrap();

    // Attendre quelques événements Ping
    let mut success = false;
    let start = std::time::Instant::now();
    while start.elapsed() < Duration::from_secs(5) {
        tokio::select! {
            Some(event) = node1.next() => {
                if let SwarmEvent::Behaviour(ping::Event { result, .. }) = event {
                    if result.is_ok() {
                        success = true;
                        break;
                    }
                }
            }
            Some(event) = node2.next() => {
                if let SwarmEvent::Behaviour(ping::Event { result, .. }) = event {
                    if result.is_ok() {
                        success = true;
                        break;
                    }
                }
            }
        }
    }

    assert!(success, "aucun ping réussi entre les deux nœuds");
}