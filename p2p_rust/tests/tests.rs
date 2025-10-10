use p2p_rust::utils::chat::*;
use futures::io::Cursor;

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

// #[tokio::test]
// async fn test_ping();