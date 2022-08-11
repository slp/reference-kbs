use std::fmt;
use std::io;

use kbs_types::{Attestation, Challenge};
use mockall::automock;
use rocket::serde::json::Value;

#[derive(Debug)]
pub enum AttesterError {
    InvalidAttestation(serde_json::Error),
    InvalidMeasurement(io::Error),
    InvalidRequest(serde_json::Error),
    SevChallengeJson(serde_json::Error),
    SevInvalidPolicy(serde_json::Error),
    SevMissingChain,
    SevMissingSession,
    SevMissingVerified,
    SevPolicy(io::Error),
    SevSecret(io::Error),
    SevSecretTooLong,
    SevSession(io::Error),
    SevSessionMeasure(io::Error),
}

impl fmt::Display for AttesterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[automock]
pub trait Attester {
    fn challenge(&mut self) -> Result<Challenge, AttesterError>;
    fn attest(&mut self, attestation: &Attestation, measurement: &str)
        -> Result<(), AttesterError>;
    fn encrypt_secret(&self, plain_secret: &[u8]) -> Result<Value, AttesterError>;
}
