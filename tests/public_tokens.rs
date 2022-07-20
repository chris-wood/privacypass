mod public_memory_stores;

use public_memory_stores::*;

use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use tokio::sync::Mutex;

use async_trait::async_trait;
use blind_rsa_signatures::KeyPair;
use privacypass::{
    auth::TokenChallenge,
    public_tokens::{client::*, server::*},
    KeyId, Nonce, NonceStore, TokenType,
};

#[tokio::test]
async fn public_tokens_cycle() {
    // Server: Instantiate in-memory keystore and nonce store.
    let mut key_store = MemoryKeyStore::default();
    let mut nonce_store = MemoryNonceStore::default();

    // Server: Create server
    let mut server = Server::new();

    // Server: Create a new keypair
    let key_pair = server.create_keypair(&mut key_store, 1).await.unwrap();

    // Client: Create client
    let mut client = Client::new(1, key_pair.pk);

    // Generate a challenge
    let challenge = TokenChallenge::new(
        TokenType::BlindRSA,
        "example.com",
        None,
        vec!["example.com".to_string()],
    );

    let challenge_digest = Sha256::digest(challenge.serialize()).to_vec();

    // Client: Prepare a TokenRequest after having received a challenge
    let (token_request, token_state) = client.issue_token_request(&challenge).unwrap();

    // Server: Issue a TokenResponse
    let token_response = server
        .issue_token_response(&key_store, token_request)
        .await
        .unwrap();

    // Client: Turn the TokenResponse into a Token
    let token = client.issue_token(token_response, token_state).unwrap();

    // Server: Compare the challenge digest
    assert_eq!(token.challenge_digest(), &challenge_digest);

    // Server: Redeem the token
    assert!(server
        .redeem_token(&mut key_store, &mut nonce_store, token.clone())
        .await
        .is_ok());

    // Server: Test double spend protection
    assert_eq!(
        server
            .redeem_token(&mut key_store, &mut nonce_store, token)
            .await,
        Err(RedeemTokenError::DoubleSpending)
    );
}
