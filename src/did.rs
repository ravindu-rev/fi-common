use serde::{Deserialize, Serialize};

use crate::{
    error::Error,
    keys::{KeyPair, VerificationKey},
};

pub const DID_CONTEXT_URL: &str = "https://www.w3.org/ns/did/v1";

#[derive(Serialize, Deserialize, Debug)]
pub struct DidDocument {
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    pub id: String,
    #[serde(rename = "verificationMethod")]
    pub verification_method: Vec<KeyPair>,
    pub authentication: Vec<String>,
    #[serde(rename = "assertionMethod")]
    pub assertion_method: Vec<String>,
    #[serde(rename = "capabilityDelegation")]
    pub capability_delegation: Vec<String>,
    #[serde(rename = "capabilityInvocation")]
    pub capability_invocation: Vec<String>,
    #[serde(rename = "keyAgreement")]
    pub key_agreement: Vec<KeyPair>,
}

pub trait KeyPairToDidDocument {
    fn key_pair_to_did_doc(
        key_pair: &Box<dyn VerificationKey>,
        fingerprint: &str,
    ) -> Result<DidDocument, Error>;
}

impl DidDocument {
    pub fn get_key(&self, key_id_fragment: &str) -> Result<KeyPair, Error> {
        let key_id = format!("{}#{}", self.id, key_id_fragment);

        let v_id = self.verification_method[0].id.clone();

        let public_key: KeyPair;
        if v_id.is_some_and(|val| val.eq(&key_id)) {
            public_key = self.verification_method[0].clone();
        } else {
            public_key = self.key_agreement[0].clone();
        }

        Ok(public_key)
    }
}
