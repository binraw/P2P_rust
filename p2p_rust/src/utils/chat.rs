use gossipsub::GossipsubEvent;
use tet_libp2p_core::{identity::Keypair,transport::{Transport, MemoryTransport}, Multiaddr};
use gossipsub::MessageAuthenticity;

// Utilise la memoire pour le transport donc a changer pour le tcp

pub fn chat() -> Result<(), Box<dyn std::error::Error>> {
let local_key = Keypair::generate_ed25519();
let local_peer_id = tet_libp2p_core::PeerId::from(local_key.public());

// Set up an encrypted TCP Transport over the Mplex
// This is test transport (memory).
let noise_keys = tet_libp2p_noise::Keypair::<tet_libp2p_noise::X25519Spec>::new().into_authentic(&local_key).unwrap();
let transport = MemoryTransport::default()
           .upgrade(tet_libp2p_core::upgrade::Version::V1)
           .authenticate(tet_libp2p_noise::NoiseConfig::xx(noise_keys).into_authenticated())
           .multiplex(tet_libp2p_mplex::MplexConfig::new())
           .boxed();

// Create a Gossipsub topic
let topic = gossipsub::IdentTopic::new("example");

// Set the message authenticity - How we expect to publish messages
// Here we expect the publisher to sign the message with their key.
let message_authenticity = MessageAuthenticity::Signed(local_key);

// Create a Swarm to manage peers and events
let mut swarm = {
    // set default parameters for gossipsub
    let gossipsub_config = gossipsub::GossipsubConfig::default();
    // build a gossipsub network behaviour
    let mut gossipsub: gossipsub::Gossipsub =
        gossipsub::Gossipsub::new(message_authenticity, gossipsub_config).unwrap();
    // subscribe to the topic
    gossipsub.subscribe(&topic);
    // create the swarm
    tet_libp2p_swarm::Swarm::new(
        transport,
        gossipsub,
        local_peer_id,
    )
};

// Listen on a memory transport.
let memory: Multiaddr = tet_libp2p_core::multiaddr::Protocol::Memory(10).into();
let addr = tet_libp2p_swarm::Swarm::listen_on(&mut swarm, memory).unwrap();
println!("Listening on {:?}", addr);

Ok(())
}

// voir car normalement je dois tester en mode memoire mais la le dialer est en tcp donc a changer

pub fn chat_dialer() -> Result<(), Box<dyn std::error::Error>> {
    let local_key = Keypair::generate_ed25519();
    let local_peer_id = tet_libp2p_core::PeerId::from(local_key.public());

    let noise_keys = tet_libp2p_noise::Keypair::<tet_libp2p_noise::X25519Spec>::new().into_authentic(&local_key).unwrap();
    let transport = MemoryTransport::default()
           .upgrade(tet_libp2p_core::upgrade::Version::V1)
           .authenticate(tet_libp2p_noise::NoiseConfig::xx(noise_keys).into_authenticated())
           .multiplex(tet_libp2p_mplex::MplexConfig::new())
           .boxed();

    let mut swarm = {
        let gossipsub_config = gossipsub::GossipsubConfig::default();
        let mut gossipsub: gossipsub::Gossipsub =
            gossipsub::Gossipsub::new(message_authenticity, gossipsub_config).unwrap();
        gossipsub.subscribe(&topic);
        tet_libp2p_swarm::Swarm::new(transport, gossipsub, local_peer_id)
    };

    let target: Multiaddr = "/ip4/127.0.0.1/tcp/10000".parse()?;
    swarm.dial(target.clone())?;
    println!("Mode: dialer → {target}");

    Ok(())
}

