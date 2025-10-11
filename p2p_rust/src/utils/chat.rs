use futures::{AsyncRead, AsyncWrite, AsyncReadExt, AsyncWriteExt};
use serde::{Serialize, Deserialize};

// Note: RequestResponseCodec n'est pas disponible sans la feature "request-response"
// Les fonctions chat() et chat_dialer() sont commentées car elles utilisent des dépendances manquantes
pub trait RequestResponseCodec {
    type Protocol;
    type Request;
    type Response;
    
    fn read_request<T: AsyncRead + Unpin + Send>(
        &mut self,
        protocol: &Self::Protocol,
        io: &mut T,
    ) -> std::io::Result<Self::Request>;
    
    fn read_response<T: AsyncRead + Unpin + Send>(
        &mut self,
        protocol: &Self::Protocol,
        io: &mut T,
    ) -> std::io::Result<Self::Response>;
    
    fn write_request<T: AsyncWrite + Unpin + Send>(
        &mut self,
        protocol: &Self::Protocol,
        io: &mut T,
        req: Self::Request,
    ) -> std::io::Result<()>;
    
    fn write_response<T: AsyncWrite + Unpin + Send>(
        &mut self,
        protocol: &Self::Protocol,
        io: &mut T,
        res: Self::Response,
    ) -> std::io::Result<()>;
}

pub struct ChatCodec;

#[derive(Debug, Clone)]
pub struct ChatProtocol;

#[derive(Clone)]
pub struct ChatRequest(pub Vec<u8>);
#[derive(Clone)]
pub struct ChatResponse(pub Vec<u8>);

impl RequestResponseCodec for ChatCodec {
    type Protocol = ChatProtocol;
    type Request = ChatRequest;
    type Response = ChatResponse;

    fn read_request<T: AsyncRead + Unpin + Send>(
        &mut self,
        _: &ChatProtocol,
        io: &mut T,
    ) -> std::io::Result<Self::Request> {
        let mut buf = Vec::new();
        futures::executor::block_on(async {
            io.read_to_end(&mut buf).await?;
            Ok(ChatRequest(buf))
        })
    }

    fn read_response<T: AsyncRead + Unpin + Send>(
        &mut self,
        _: &ChatProtocol,
        io: &mut T,
    ) -> std::io::Result<Self::Response> {
        let mut buf = Vec::new();
        futures::executor::block_on(async {
            io.read_to_end(&mut buf).await?;
            Ok(ChatResponse(buf))
        })
    }

    fn write_request<T: AsyncWrite + Unpin + Send>(
        &mut self,
        _: &ChatProtocol,
        io: &mut T,
        ChatRequest(data): ChatRequest,
    ) -> std::io::Result<()> {
        futures::executor::block_on(async {
            io.write_all(&data).await?;
            io.close().await?;
            Ok(())
        })
    }

    fn write_response<T: AsyncWrite + Unpin + Send>(
        &mut self,
        _: &ChatProtocol,
        io: &mut T,
        ChatResponse(data): ChatResponse,
    ) -> std::io::Result<()> {
        futures::executor::block_on(async {
            io.write_all(&data).await?;
            io.close().await?;
            Ok(())
        })
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