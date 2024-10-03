use crate::{
    error::Error,
    keys::{KeyPair, VerificationKey},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use wasm_bindgen::prelude::wasm_bindgen;
#[cfg(feature = "wasm")]
use wasm_bindgen::JsValue;

pub const DID_CONTEXT_URL: &str = "https://www.w3.org/ns/did/v1";

#[derive(Serialize, Deserialize, Clone)]
#[wasm_bindgen]
pub struct DidDocument {
    #[serde(rename = "@context")]
    #[wasm_bindgen(skip)]
    pub context: Vec<String>,
    #[wasm_bindgen(skip)]
    pub id: String,
    #[wasm_bindgen(skip)]
    #[serde(rename = "verificationMethod")]
    pub verification_method: Option<Vec<KeyPair>>,
    #[wasm_bindgen(skip)]
    pub authentication: Option<Vec<String>>,
    #[wasm_bindgen(skip)]
    #[serde(rename = "assertionMethod")]
    pub assertion_method: Option<Vec<String>>,
    #[wasm_bindgen(skip)]
    #[serde(rename = "capabilityDelegation")]
    pub capability_delegation: Option<Vec<String>>,
    #[wasm_bindgen(skip)]
    #[serde(rename = "capabilityInvocation")]
    pub capability_invocation: Option<Vec<String>>,
    #[serde(rename = "keyAgreement")]
    #[wasm_bindgen(skip)]
    pub key_agreement: Option<Vec<KeyPair>>,
    #[wasm_bindgen(skip)]
    pub services: Option<Vec<Service>>,
}

pub trait KeyPairToDidDocument {
    fn key_pair_to_did_doc(
        key_pair: &Box<dyn VerificationKey>,
        fingerprint: &str,
    ) -> Result<DidDocument, Error>;
}

#[wasm_bindgen]
impl DidDocument {
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
    pub fn set_verification_method(&mut self, verification_method: Option<Vec<KeyPair>>) {
        self.verification_method = verification_method;
    }

    #[cfg(feature = "wasm")]
    #[wasm_bindgen(getter, js_name = "verificationMethod")]
    pub fn verification_method(&self) -> Option<Vec<KeyPair>> {
        self.verification_method.clone()
    }

    #[cfg(feature = "wasm")]
    #[wasm_bindgen(setter)]
    pub fn set_authentication(&mut self, authentication: Option<Vec<String>>) {
        self.authentication = authentication;
    }

    #[cfg(feature = "wasm")]
    #[wasm_bindgen(getter)]
    pub fn authentication(&self) -> Option<Vec<String>> {
        self.authentication.clone()
    }

    #[cfg(feature = "wasm")]
    #[wasm_bindgen(setter, js_name = "assertionMethod")]
    pub fn set_assertion_method(&mut self, assertion_method: Option<Vec<String>>) {
        self.assertion_method = assertion_method;
    }

    #[cfg(feature = "wasm")]
    #[wasm_bindgen(getter, js_name = "assertionMethod")]
    pub fn assertion_method(&self) -> Option<Vec<String>> {
        self.assertion_method.clone()
    }

    #[cfg(feature = "wasm")]
    #[wasm_bindgen(setter, js_name = "capabilityDelegation")]
    pub fn set_capability_delegation(&mut self, capability_delegation: Option<Vec<String>>) {
        self.capability_delegation = capability_delegation;
    }

    #[cfg(feature = "wasm")]
    #[wasm_bindgen(getter, js_name = "capabilityDelegation")]
    pub fn capability_delegation(&self) -> Option<Vec<String>> {
        self.capability_delegation.clone()
    }

    #[cfg(feature = "wasm")]
    #[wasm_bindgen(setter, js_name = "capabilityInvocation")]
    pub fn set_capability_invocation(&mut self, capability_invocation: Option<Vec<String>>) {
        self.capability_invocation = capability_invocation;
    }

    #[cfg(feature = "wasm")]
    #[wasm_bindgen(getter, js_name = "capabilityInvocation")]
    pub fn capability_invocation(&self) -> Option<Vec<String>> {
        self.capability_invocation.clone()
    }

    #[cfg(feature = "wasm")]
    #[wasm_bindgen(setter, js_name = "keyAgreement")]
    pub fn set_key_agreement(&mut self, key_agreement: Option<Vec<KeyPair>>) {
        self.key_agreement = key_agreement;
    }

    #[cfg(feature = "wasm")]
    #[wasm_bindgen(getter, js_name = "keyAgreement")]
    pub fn key_agreement(&self) -> Option<Vec<KeyPair>> {
        self.key_agreement.clone()
    }

    #[cfg(feature = "wasm")]
    pub fn set_services(&mut self, services: Option<Vec<Service>>) {
        self.services = services;
    }

    #[cfg(feature = "wasm")]
    pub fn services(&self) -> Option<Vec<Service>> {
        self.services.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[wasm_bindgen]
pub struct Service {
    #[wasm_bindgen(skip)]
    pub id: String,
    #[serde(rename = "type")]
    #[wasm_bindgen(skip)]
    pub _type: String,
    #[wasm_bindgen(skip)]
    pub service_endpoint: Value,
}

#[wasm_bindgen]
impl Service {
    #[cfg(feature = "wasm")]
    #[wasm_bindgen(setter)]
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    #[cfg(feature = "wasm")]
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.id.clone()
    }

    #[cfg(feature = "wasm")]
    #[wasm_bindgen(setter, js_name = "type")]
    pub fn set_type(&mut self, _type: String) {
        self._type = _type;
    }

    #[cfg(feature = "wasm")]
    #[wasm_bindgen(getter, js_name = "type")]
    pub fn _type(&self) -> String {
        self._type.clone()
    }

    #[cfg(feature = "wasm")]
    #[wasm_bindgen(setter, js_name = "serviceEndpoint")]
    pub fn set_service_endpoint(
        &mut self,
        service_endpoint: JsValue,
    ) -> Result<(), serde_wasm_bindgen::Error> {
        self.service_endpoint = match serde_wasm_bindgen::from_value(service_endpoint) {
            Ok(val) => val,
            Err(error) => return Err(error),
        };

        Ok(())
    }

    #[cfg(feature = "wasm")]
    #[wasm_bindgen(getter, js_name = "serviceEndpoint")]
    pub fn service_endpoint(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        match serde_wasm_bindgen::to_value(&self.service_endpoint) {
            Err(error) => Err(error),
            Ok(val) => Ok(val),
        }
    }
}
