use futures::{AsyncRead, AsyncWrite, AsyncReadExt, AsyncWriteExt};
use serde::{Serialize, Deserialize};
use futures::future::BoxFuture; // Pour les futures retournées par le Codec libp2p
use std::{io::ErrorKind, iter};


pub struct ChatCodec;



impl PortocolName for ChatProtocol {
    fn protocol_name(&self) -> &'static str {
        "/p2p/chat/request/1.0.0"
    }
}


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
}

#[derive(Serialize, Deserialize)]
pub enum Message {
    Ping(PingMessage),
    Pong(PongMessage),
    Chat(ChatMessage),
    Request(RequestMessage),
    Response(ResponseMessage),
}

#[derive(Serialize, Deserialize)]
pub struct PingMessage {
    id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct PongMessage {
    id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ChatMessage {
    id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct RequestMessage {
    id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseMessage {
    id: u64,
}