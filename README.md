
### Ce projet implémente un nœud libp2p qui :
génère un identifiant unique (PeerId)
écoute sur un port TCP libre
sécurise les connexions avec Noise (authentification + chiffrement)
supporte le multiplexage via Yamux
utilise le protocole Ping pour tester la connectivité entre pairs
gère tous les événements via un Swarm (SwarmEvent)