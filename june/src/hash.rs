use argonautica::{Hasher, Verifier};
use futures::compat::Future01CompatExt;

pub struct PasswordHasher {
    secret: String,
}

impl PasswordHasher {
    pub fn new<S: Into<String>>(secret: S) -> Self {
        Self {
            secret: secret.into(),
        }
    }

    pub async fn encode(&self, plain: &str) -> String {
        Hasher::default()
            .with_password(plain)
            .with_secret_key(&self.secret)
            .hash_non_blocking()
            .compat()
            .await
            .unwrap()
    }

    pub async fn verify(&self, hashed: &str, password: &str) -> bool {
        Verifier::default()
            .with_hash(hashed)
            .with_password(password)
            .with_secret_key(&self.secret)
            .verify_non_blocking()
            .compat()
            .await
            .unwrap()
    }
}
