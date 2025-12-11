use futures::{AsyncRead, AsyncWrite, AsyncReadExt, AsyncWriteExt};
use serde::{Serialize, Deserialize};

// Note: RequestResponseCodec n'est pas disponible sans la feature "request-response"
// Les fonctions chat() et chat_dialer() sont commentées car elles utilisent des dépendances manquantes
use tokio::io::{AsyncRead, AsyncWrite, AsyncReadExt, AsyncWriteExt};
use std::io::ErrorKind;
use futures::future::BoxFuture; // Pour les futures retournées par le Codec libp2p
use std::{io::ErrorKind, iter};

// --- LE TRAIT VRAI DE LIBP2P UTILISE DES FUTURES ---

// (Pour l'exemple, nous allons simuler les opérations asynchrones)

pub struct ChatCodec;

impl libp2p::request_response::RequestResponseCodec for ChatCodec {
    // ... Types Protocol, Request, Response (comme définis avant)
    type Protocol = ChatProtocol;
    type Request = ChatRequest;
    type Response = ChatResponse;
    
    // La fonction doit retourner un Future qui sera poll
    fn read_request(&mut self, protocol: &Self::Protocol, io: &mut (impl AsyncRead + Unpin + Send + '_)) -> BoxFuture<'_, std::io::Result<Self::Request>> {
        Box::pin(async move {
            // 1. Lire le préfixe de longueur (4 octets pour u32)
            let len = io.read_u32().await?;
            if len == 0 {
                return Err(std::io::Error::new(ErrorKind::InvalidData, "Empty message"));
            }
            let len = len as usize;
            
            // 2. Lire le message de la taille spécifiée
            let mut buf = vec![0u8; len];
            io.read_exact(&mut buf).await?;
            
            Ok(ChatRequest(buf))
        })
    }

    fn write_request(&mut self, protocol: &Self::Protocol, io: &mut (impl AsyncWrite + Unpin + Send + '_), ChatRequest(data): ChatRequest) -> BoxFuture<'_, std::io::Result<()>> {
        Box::pin(async move {
            let len = data.len() as u32;

            // 1. Écrire le préfixe de longueur
            io.write_u32(len).await?;

            // 2. Écrire les données
            io.write_all(&data).await?;

            // 3. Flusher pour garantir l'envoi (NE PAS fermer le flux)
            io.flush().await?; 
            
            Ok(())
        })
    }
    
    // read_response et write_response seraient implémentées de la même manière
    fn read_response(&mut self, protocol: &Self::Protocol, io: &mut (impl AsyncRead + Unpin + Send + '_)) -> BoxFuture<'_, std::io::Result<Self::Response>> {
        // Implémentation identique à read_request, mais retourne ChatResponse
        // ...
        Box::pin(async move {
             let len = io.read_u32().await?;
             let mut buf = vec![0u8; len as usize];
             io.read_exact(&mut buf).await?;
             Ok(ChatResponse(buf))
        })
    }

    fn write_response(&mut self, protocol: &Self::Protocol, io: &mut (impl AsyncWrite + Unpin + Send + '_), ChatResponse(data): ChatResponse) -> BoxFuture<'_, std::io::Result<()>> {
        // Implémentation identique à write_request, mais prend ChatResponse
        // ...
        Box::pin(async move {
             let len = data.len() as u32;
             io.write_u32(len).await?;
             io.write_all(&data).await?;
             io.flush().await?;
             Ok(())
        })
    }

    impl PortocolName for ChatProtocol {
        fn protocol_name(&self) -> &'static str {
            "/p2p/chat/request/1.0.0"
        }
    }
}
#[derive(Serialize, Deserialize)]
enum Message {
    Ping(PingMessage),
    Pong(PongMessage),
    Chat(ChatMessage),
    Request(RequestMessage),
    Response(ResponseMessage),
}

#[derive(Serialize, Deserialize)]
struct PingMessage {
    id: u64,
}

#[derive(Serialize, Deserialize)]
struct PongMessage {
    id: u64,
}

#[derive(Serialize, Deserialize)]
struct ChatMessage {
    id: u64,
}

#[derive(Serialize, Deserialize)]
struct RequestMessage {
    id: u64,
}

#[derive(Serialize, Deserialize)]
struct ResponseMessage {
    id: u64,
}