use crate::{
    error::Error,
    keys::{KeyPair, VerificationKey},
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;
#[cfg(feature = "wasm")]
use wasm_bindgen::JsValue;

pub const DID_CONTEXT_URL: &str = "https://www.w3.org/ns/did/v1";

#[derive(Serialize, Deserialize, Debug)]
#[wasm_bindgen]
pub struct DidDocument {
    #[serde(rename = "@context")]
    #[wasm_bindgen(skip)]
    pub context: Vec<String>,
    #[wasm_bindgen(skip)]
    pub id: String,
    #[wasm_bindgen(skip)]
    #[serde(rename = "verificationMethod")]
    pub verification_method: Vec<KeyPair>,
    #[wasm_bindgen(skip)]
    pub authentication: Vec<String>,
    #[wasm_bindgen(skip)]
    #[serde(rename = "assertionMethod")]
    pub assertion_method: Vec<String>,
    #[wasm_bindgen(skip)]
    #[serde(rename = "capabilityDelegation")]
    pub capability_delegation: Vec<String>,
    #[wasm_bindgen(skip)]
    #[serde(rename = "capabilityInvocation")]
    pub capability_invocation: Vec<String>,
    #[serde(rename = "keyAgreement")]
    #[wasm_bindgen(skip)]
    pub key_agreement: Vec<KeyPair>,
}

pub trait KeyPairToDidDocument {
    fn key_pair_to_did_doc(
        key_pair: &Box<dyn VerificationKey>,
        fingerprint: &str,
    ) -> Result<DidDocument, Error>;
}

#[wasm_bindgen]
impl DidDocument {
    #[wasm_bindgen(js_name = "getKeyPair")]
    pub fn get_key_pair(&self, key_id_fragment: &str) -> Result<KeyPair, Error> {
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

    #[cfg(feature = "wasm")]
    #[wasm_bindgen(js_name = "toObject")]
    pub fn to_object(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        serde_wasm_bindgen::to_value(self)
    }

    #[cfg(feature = "wasm")]
    #[wasm_bindgen(getter)]
    pub fn context(&self) -> Vec<String> {
        self.context.clone()
    }

    #[cfg(feature = "wasm")]
    #[wasm_bindgen(setter)]
    pub fn set_context(&mut self, context: Vec<String>) {
        self.context = context;
    }

    #[cfg(feature = "wasm")]
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.id.clone()
    }

    #[cfg(feature = "wasm")]
    #[wasm_bindgen(setter)]
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    #[cfg(feature = "wasm")]
    #[wasm_bindgen(setter, js_name = "verificationMethod")]
    pub fn set_verification_method(&mut self, verification_method: Vec<KeyPair>) {
        self.verification_method = verification_method;
    }

    #[cfg(feature = "wasm")]
    #[wasm_bindgen(getter, js_name = "verificationMethod")]
    pub fn verification_method(&self) -> Vec<KeyPair> {
        self.verification_method.clone()
    }

    #[cfg(feature = "wasm")]
    #[wasm_bindgen(setter)]
    pub fn set_authentication(&mut self, authentication: Vec<String>) {
        self.authentication = authentication;
    }

    #[cfg(feature = "wasm")]
    #[wasm_bindgen(getter)]
    pub fn authentication(&self) -> Vec<String> {
        self.authentication.clone()
    }

    #[cfg(feature = "wasm")]
    #[wasm_bindgen(setter, js_name = "assertionMethod")]
    pub fn set_assertion_method(&mut self, assertion_method: Vec<String>) {
        self.assertion_method = assertion_method;
    }

    #[cfg(feature = "wasm")]
    #[wasm_bindgen(getter, js_name = "assertionMethod")]
    pub fn assertion_method(&self) -> Vec<String> {
        self.assertion_method.clone()
    }

    #[cfg(feature = "wasm")]
    #[wasm_bindgen(setter, js_name = "capabilityDelegation")]
    pub fn set_capability_delegation(&mut self, capability_delegation: Vec<String>) {
        self.capability_delegation = capability_delegation;
    }

    #[cfg(feature = "wasm")]
    #[wasm_bindgen(getter, js_name = "capabilityDelegation")]
    pub fn capability_delegation(&self) -> Vec<String> {
        self.capability_delegation.clone()
    }

    #[cfg(feature = "wasm")]
    #[wasm_bindgen(setter, js_name = "capabilityInvocation")]
    pub fn set_capability_invocation(&mut self, capability_invocation: Vec<String>) {
        self.capability_invocation = capability_invocation;
    }

    #[cfg(feature = "wasm")]
    #[wasm_bindgen(getter, js_name = "capabilityInvocation")]
    pub fn capability_invocation(&self) -> Vec<String> {
        self.capability_invocation.clone()
    }

    #[cfg(feature = "wasm")]
    #[wasm_bindgen(setter, js_name = "keyAgreement")]
    pub fn set_key_agreement(&mut self, key_agreement: Vec<KeyPair>) {
        self.key_agreement = key_agreement;
    }

    #[cfg(feature = "wasm")]
    #[wasm_bindgen(getter, js_name = "keyAgreement")]
    pub fn key_agreement(&self) -> Vec<KeyPair> {
        self.key_agreement.clone()
    }
}
