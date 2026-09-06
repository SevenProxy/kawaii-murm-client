use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::{rand_core::UnwrapErr, rngs::SysRng};

// NAME
const STORAGE_KEY: &str = "murm_identity";

#[cfg(target_arch = "wasm32")]
use gloo_storage::Storage;

#[derive(Clone, PartialEq)]
pub struct Identity {
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
}

impl Identity {
    pub fn generate() -> Self {
        let signing_key = SigningKey::generate(&mut UnwrapErr(SysRng));
        let verifying_key = signing_key.verifying_key();
        Self {
            signing_key,
            verifying_key,
        }
    }

    pub fn from_secret_bytes(secret: &[u8; 32]) -> Self {
        let signing_key = SigningKey::from_bytes(secret);
        let verifying_key = signing_key.verifying_key();
        Self {
            signing_key,
            verifying_key,
        }
    }

    pub fn pubkey_hex(&self) -> String {
        hex::encode(self.verifying_key.to_bytes())
    }

    pub fn sign(&self, message: &[u8]) -> Signature {
        self.signing_key.sign(message)
    }

    pub fn verify(
        &self,
        message: &[u8],
        signature: &Signature,
    ) -> Result<(), ed25519_dalek::SignatureError> {
        self.verifying_key.verify(message, signature)
    }

    pub fn secret_hex(&self) -> String {
        hex::encode(self.signing_key.to_bytes())
    }

    pub fn from_secret_hex(hex_str: &str) -> anyhow::Result<Self> {
        let bytes = hex::decode(hex_str.trim())?;
        let secret: [u8; 32] = bytes.as_slice().try_into().map_err(|_| {
            anyhow::anyhow!(
                "identity must contain exactly 32 bytes, got {}",
                bytes.len()
            )
        })?;
        Ok(Self::from_secret_bytes(&secret))
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn save(&self, path: impl AsRef<std::path::Path>) -> std::io::Result<()> {
        std::fs::write(path, self.secret_hex())
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn load(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        let raw = std::fs::read_to_string(&path)?;
        Self::from_secret_hex(&raw)
    }

    #[cfg(target_arch = "wasm32")]
    pub fn save_local() -> anyhow::Result<Self> {
        let identity = Self::generate();
        gloo_storage::LocalStorage::set(STORAGE_KEY, identity.secret_hex())
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        Ok(identity)
    }

    #[cfg(target_arch = "wasm32")]
    pub fn load_local() -> Option<Self> {
        let hex_str: String = gloo_storage::LocalStorage::get(STORAGE_KEY).ok()?;
        Self::from_secret_hex(&hex_str).ok()
    }
}
