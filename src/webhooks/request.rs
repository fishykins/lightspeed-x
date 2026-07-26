use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;
use std::collections::HashMap;

type HmacSha256 = Hmac<Sha256>;
const X_SIGNATURE: &'static str = "X-Signature";

use crate::{
    LsError, LsResult,
    webhooks::{Signature, SignatureAlgorithm, WebhookForm},
};

#[derive(Debug)]
pub struct WebhookRequest {
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl WebhookRequest {
    pub fn new(headers: HashMap<String, String>, body: Vec<u8>) -> Self {
        Self { headers, body }
    }

    pub fn can_verify(&self) -> bool {
        self.headers.get(X_SIGNATURE).is_some()
    }

    pub fn verify(&self, client_secret: &str) -> LsResult<()> {
        let signature: Signature = self
            .headers
            .get(X_SIGNATURE)
            .ok_or(LsError::MissingSignature)?
            .parse()?;

        match signature.algorithm {
            SignatureAlgorithm::HmacSha256 => {
                let expected = hex::decode(signature.signature)
                    .map_err(|e| LsError::Other(format!("Invalid signature encoding: {e}")))?;

                let mut mac = HmacSha256::new_from_slice(client_secret.as_bytes())
                    .map_err(|e| LsError::Other(format!("Unable to initialise HMAC: {e}")))?;

                mac.update(&self.body);

                mac.verify_slice(&expected)
                    .map_err(|_| LsError::InvalidSignature)?;
            }
            SignatureAlgorithm::Other(a) => {
                return Err(LsError::UnsupportedSignatureAlgorithm(a));
            }
        }

        Ok(())
    }

    pub fn form(&self) -> LsResult<WebhookForm> {
        let form: WebhookForm = serde_urlencoded::from_bytes(&self.body)?;

        if form.payload.is_empty() {
            return Err(LsError::MissingPayload);
        }

        Ok(form)
    }
}
