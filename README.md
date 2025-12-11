
### Ce projet implémente un nœud libp2p qui :
génère un identifiant unique (PeerId)
écoute sur un port TCP libre
sécurise les connexions avec Noise (authentification + chiffrement)
supporte le multiplexage via Yamux
utilise le protocole Ping pour tester la connectivité entre pairs
gère tous les événements via un Swarm (SwarmEvent)




# infos

dans un systeme decentraliser avec du p2p il va avoir des noeuds d'amorcage ecrit en dur
pour permettre la recherche d'autres noeuds qui sont ajoute a la table de routage apres la premiere connexion
aux noeuds d'amorcage.