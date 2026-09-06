use std::time::{SystemTime, UNIX_EPOCH};

use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::identity::Identity;

/// MIP-01: the serialized event JSON MUST NOT exceed 2 MiB.
pub const MAX_EVENT_SIZE: usize = 2 * 1024 * 1024;

/// Unsigned event data, used to compute the canonical payload (MIP-02).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub pubkey: String,
    pub created_at: u64,
    pub kind: u32,
    pub tags: Vec<Vec<String>>,
    pub content: String,
}

/// Signed event ready to be published, matching the MIP-01 event shape.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Payload {
    pub id: String,
    pub pubkey: String,
    pub created_at: u64,
    pub kind: u32,
    pub tags: Vec<Vec<String>>,
    pub content: String,
    pub sig: String,
}

impl Event {
    /// Creates an unsigned event with the current Unix timestamp.
    pub fn new(pubkey: String, kind: u32, tags: Vec<Vec<String>>, content: String) -> Self {
        Self {
            pubkey,
            created_at: now(),
            kind,
            tags,
            content,
        }
    }

    /// Canonical payload (MIP-02): `[pubkey, created_at, kind, tags, content]`
    /// serialized as compact JSON with no whitespace.
    pub fn canonical_payload(&self) -> String {
        serde_json::json!([
            &self.pubkey,
            self.created_at,
            self.kind,
            &self.tags,
            &self.content,
        ])
        .to_string()
    }

    /// Event id: lowercase hex of the SHA-256 over the canonical payload bytes.
    pub fn id(&self) -> String {
        hex::encode(Sha256::digest(self.canonical_payload().as_bytes()))
    }

    /// Signs the event and produces the final `Payload` (MIP-02).
    ///
    /// The signature is computed over the raw 32-byte id hash, not its hex form.
    pub fn sign(&self, identity: &Identity) -> Payload {
        let id = self.id();
        let signature = identity.sign(&hex::decode(&id).expect("id is valid hex"));

        Payload {
            id,
            pubkey: self.pubkey.clone(),
            created_at: self.created_at,
            kind: self.kind,
            tags: self.tags.clone(),
            content: self.content.clone(),
            sig: hex::encode(signature.to_bytes()),
        }
    }
}

impl Payload {
    /// Converts the signed payload back to its unsigned event.
    pub fn unsigned(&self) -> Event {
        Event {
            pubkey: self.pubkey.clone(),
            created_at: self.created_at,
            kind: self.kind,
            tags: self.tags.clone(),
            content: self.content.clone(),
        }
    }

    pub fn canonical_payload(&self) -> String {
        self.unsigned().canonical_payload()
    }

    /// Size of the serialized event JSON, for the MIP-01 size limit.
    pub fn size_bytes(&self) -> usize {
        serde_json::to_vec(self)
            .map(|bytes| bytes.len())
            .unwrap_or(0)
    }

    pub fn size_ok(&self) -> bool {
        self.size_bytes() <= MAX_EVENT_SIZE
    }

    /// Checks that the event id matches the recomputed canonical hash.
    pub fn id_valid(&self) -> bool {
        self.unsigned().id() == self.id
    }

    /// Full validation per MIP-02: id hash and Ed25519 signature.
    pub fn verify(&self) -> bool {
        if !self.id_valid() {
            return false;
        }

        let id_bytes = match hex::decode(&self.id) {
            Ok(bytes) if bytes.len() == 32 => bytes,
            _ => return false,
        };
        let pubkey_bytes = match hex::decode(&self.pubkey) {
            Ok(bytes) if bytes.len() == 32 => bytes,
            _ => return false,
        };
        let sig_bytes = match hex::decode(&self.sig) {
            Ok(bytes) if bytes.len() == 64 => bytes,
            _ => return false,
        };

        let verifying_key = match VerifyingKey::from_bytes(
            pubkey_bytes.as_slice().try_into().expect("checked length"),
        ) {
            Ok(key) => key,
            Err(_) => return false,
        };
        let signature =
            Signature::from_bytes(sig_bytes.as_slice().try_into().expect("checked length"));

        verifying_key.verify(&id_bytes, &signature).is_ok()
    }

    /// Verifies the payload and that it was authored by the given identity.
    pub fn verify_with(&self, identity: &Identity) -> bool {
        identity.pubkey_hex() == self.pubkey && self.verify()
    }
}

/// Current Unix timestamp in seconds.
fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before unix epoch")
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::{kinds, tags};

    #[test]
    fn canonical_payload_is_compact() {
        let identity = Identity::generate();
        let event = Event::new(
            identity.pubkey_hex(),
            1,
            vec![tags::topic("murm")],
            "hello".into(),
        );

        let json = event.canonical_payload();
        assert!(!json.contains(' '));
        assert!(!json.contains('\n'));
        assert_eq!(
            json,
            format!(
                "[\"{}\",{},1,[[\"topic\",\"murm\"]],\"hello\"]",
                event.pubkey, event.created_at
            )
        );
    }

    #[test]
    fn sign_and_verify_roundtrip() {
        let identity = Identity::generate();
        let event = Event::new(identity.pubkey_hex(), kinds::POST, vec![], "hi".into());

        let payload = event.sign(&identity);
        assert!(payload.verify());
        assert!(payload.verify_with(&identity));
        assert!(payload.size_ok());
    }

    #[test]
    fn tampered_event_is_rejected() {
        let identity = Identity::generate();
        let event = Event::new(
            identity.pubkey_hex(),
            kinds::POST,
            vec![],
            "original".into(),
        );

        let mut payload = event.sign(&identity);
        assert!(payload.verify());

        payload.content = "modified".into();
        assert!(!payload.verify());
        assert!(!payload.id_valid());
    }

    #[test]
    fn id_changes_with_content() {
        let identity = Identity::generate();
        let a = Event::new(identity.pubkey_hex(), 1, vec![], "a".into());
        let b = Event::new(identity.pubkey_hex(), 1, vec![], "b".into());

        assert_ne!(a.id(), b.id());
    }
}
