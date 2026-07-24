use std::str::FromStr;

use crate::LsError;

pub struct Signature {
    pub algorithm: SignatureAlgorithm,
    pub signature: String,
}

pub enum SignatureAlgorithm {
    HmacSha256,
    Other(String),
}

impl FromStr for Signature {
    type Err = LsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut signature = None;
        let mut algorithm = None;

        for part in s.split(',') {
            let (key, value) = part
                .split_once('=')
                .ok_or(LsError::InvalidSignatureHeader)?;

            match key.trim() {
                "signature" => signature = Some(value.trim().to_owned()),
                "algorithm" => {
                    algorithm = Some(match value.trim() {
                        "HMAC-SHA256" => SignatureAlgorithm::HmacSha256,
                        other => SignatureAlgorithm::Other(other.to_owned()),
                    });
                }
                _ => {} // Ignore unknown fields for forwards compatibility.
            }
        }

        Ok(Signature {
            signature: signature.ok_or(LsError::InvalidSignatureHeader)?,
            algorithm: algorithm.ok_or(LsError::InvalidSignatureHeader)?,
        })
    }
}
