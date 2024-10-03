use crate::error::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use wasm_bindgen::prelude::wasm_bindgen;
#[cfg(feature = "wasm")]
use wasm_bindgen::JsValue;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[wasm_bindgen]
pub struct KeyPair {
    #[wasm_bindgen(skip)]
    pub id: Option<String>,
    #[serde(rename = "type")]
    #[wasm_bindgen(skip)]
    pub _type: String,
    #[serde(rename = "@context")]
    #[wasm_bindgen(skip)]
    pub context: Option<Vec<String>>,
    #[wasm_bindgen(skip)]
    #[serde(rename = "publicKeyBase58")]
    pub public_key_base58: Option<String>,
    #[wasm_bindgen(skip)]
    #[serde(rename = "privateKeyBase58")]
    pub private_key_base58: Option<String>,
    #[wasm_bindgen(skip)]
    #[serde(rename = "publicKeyMultibase")]
    pub public_key_multibase: Option<String>,
    #[wasm_bindgen(skip)]
    #[serde(rename = "privateKeyMultibase")]
    pub private_key_multibase: Option<String>,
    pub revoked: Option<bool>,
    #[wasm_bindgen(skip)]
    pub controller: Option<String>,
    #[wasm_bindgen(skip)]
    #[serde(rename = "blockchainAccountId")]
    pub blockchain_account_id: Option<String>,
    #[wasm_bindgen(skip)]
    #[serde(rename = "publicKeyHex")]
    pub public_key_hex: Option<String>,
    #[wasm_bindgen(skip)]
    #[serde(rename = "ethereumAddress")]
    pub ethereum_address: Option<String>,
    #[wasm_bindgen(skip)]
    #[serde(rename = "publicKeyBase64")]
    pub public_key_base64: Option<String>,
    #[wasm_bindgen(skip)]
    #[serde(rename = "publicKeyPem")]
    pub public_key_pem: Option<String>,
    #[wasm_bindgen(skip)]
    #[serde(rename = "publicKeyJwk")]
    pub public_key_jwk: Option<Value>,
    #[wasm_bindgen(skip)]
    #[serde(rename = "privateKeyHex")]
    pub private_key_hex: Option<String>,
    #[wasm_bindgen(skip)]
    #[serde(rename = "privateKeyBase64")]
    pub private_key_base64: Option<String>,
    #[wasm_bindgen(skip)]
    #[serde(rename = "privateKeyPem")]
    pub private_key_pem: Option<String>,
    #[wasm_bindgen(skip)]
    pub value: Option<String>,
}

#[wasm_bindgen]
#[cfg(feature = "wasm")]
impl KeyPair {
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> Option<String> {
        self.id.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_id(&mut self, id: Option<String>) {
        self.id = id;
    }

    #[wasm_bindgen(getter, js_name = "type")]
    pub fn _type(&self) -> String {
        self._type.clone()
    }

    #[wasm_bindgen(setter, js_name = "type")]
    pub fn set_type(&mut self, _type: String) {
        self._type = _type;
    }

    #[wasm_bindgen(getter)]
    pub fn context(&self) -> Option<Vec<String>> {
        self.context.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_context(&mut self, context: Option<Vec<String>>) {
        self.context = context;
    }

    #[wasm_bindgen(getter, js_name = "public")]
    pub fn public_key_base58(&self) -> Option<String> {
        self.public_key_base58.clone()
    }

    #[wasm_bindgen(setter, js_name = "publicKeyBase58")]
    pub fn set_public_key_base58(&mut self, public_key_base58: Option<String>) {
        self.public_key_base58 = public_key_base58;
    }

    #[wasm_bindgen(getter, js_name = "privateKeyBase58")]
    pub fn private_key_base58(&self) -> Option<String> {
        self.private_key_base58.clone()
    }

    #[wasm_bindgen(setter, js_name = "privateKeyBase58")]
    pub fn set_private_key_base58(&mut self, private_key_base58: Option<String>) {
        self.private_key_base58 = private_key_base58;
    }

    #[wasm_bindgen(getter, js_name = "publicKeyJwk")]
    pub fn public_key_jwk(&self) -> JsValue {
        match self.public_key_jwk.clone() {
            Some(jwk) => match serde_wasm_bindgen::to_value(&jwk) {
                Err(_error) => return JsValue::null(),
                Ok(val) => val,
            },
            None => return JsValue::null(),
        }
    }

    #[wasm_bindgen(setter, js_name = "publicKeyJwk")]
    pub fn set_public_key_jwk(&mut self, public_key_jwk: JsValue) -> Result<(), Error> {
        match public_key_jwk.is_object() {
            true => match serde_wasm_bindgen::from_value(public_key_jwk) {
                Err(error) => return Err(Error::new(error.to_string().as_str())),
                Ok(val) => self.public_key_jwk = Some(val),
            },
            false => self.public_key_jwk = None,
        }

        Ok(())
    }

    #[wasm_bindgen(getter, js_name = "ethereumAddress")]
    pub fn ethereum_address(&self) -> Option<String> {
        self.ethereum_address.clone()
    }

    #[wasm_bindgen(setter, js_name = "ethereumAddress")]
    pub fn set_ethereum_address(&mut self, ethereum_address: Option<String>) {
        self.ethereum_address = ethereum_address;
    }

    #[wasm_bindgen(getter, js_name = "publicKeyMultibase")]
    pub fn public_key_multibase(&self) -> Option<String> {
        self.public_key_multibase.clone()
    }

    #[wasm_bindgen(setter, js_name = "publicKeyMultibase")]
    pub fn set_public_key_multibase(&mut self, public_key_multibase: Option<String>) {
        self.public_key_multibase = public_key_multibase;
    }

    #[wasm_bindgen(getter, js_name = "privateKeyMultibase")]
    pub fn private_key_multibase(&self) -> Option<String> {
        self.private_key_multibase.clone()
    }

    #[wasm_bindgen(setter, js_name = "privateKeyMultibase")]
    pub fn set_private_key_multibase(&mut self, public_key_multibase: Option<String>) {
        self.public_key_multibase = public_key_multibase;
    }

    #[wasm_bindgen(getter)]
    pub fn controller(&self) -> Option<String> {
        self.controller.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_controller(&mut self, controller: Option<String>) {
        self.controller = controller;
    }

    #[wasm_bindgen(getter, js_name = "blockchainAccountId")]
    pub fn blockchain_account_id(&self) -> Option<String> {
        self.blockchain_account_id.clone()
    }

    #[wasm_bindgen(setter, js_name = "blockchainAccountId")]
    pub fn set_blockchain_account_id(&mut self, blockchain_account_id: Option<String>) {
        self.blockchain_account_id = blockchain_account_id;
    }

    #[wasm_bindgen(getter, js_name = "publicKeyHex")]
    pub fn public_key_hex(&self) -> Option<String> {
        self.public_key_hex.clone()
    }

    #[wasm_bindgen(setter, js_name = "publicKeyHex")]
    pub fn set_public_key_hex(&mut self, public_key_hex: Option<String>) {
        self.public_key_hex = public_key_hex;
    }
    #[wasm_bindgen(getter, js_name = "publicKeyBase64")]
    pub fn public_key_base64(&self) -> Option<String> {
        self.public_key_base64.clone()
    }

    #[wasm_bindgen(setter, js_name = "publicKeyBase64")]
    pub fn set_public_key_base64(&mut self, public_key_base64: Option<String>) {
        self.public_key_base64 = public_key_base64;
    }
    #[wasm_bindgen(getter, js_name = "publicKeyPem")]
    pub fn public_key_pem(&self) -> Option<String> {
        self.public_key_pem.clone()
    }

    #[wasm_bindgen(setter, js_name = "publicKeyPem")]
    pub fn set_public_key_pem(&mut self, public_key_pem: Option<String>) {
        self.public_key_pem = public_key_pem;
    }

    #[wasm_bindgen(getter, js_name = "privateKeyHex")]
    pub fn private_key_hex(&self) -> Option<String> {
        self.private_key_hex.clone()
    }

    #[wasm_bindgen(setter, js_name = "privateKeyHex")]
    pub fn set_private_key_hex(&mut self, private_key_hex: Option<String>) {
        self.private_key_hex = private_key_hex;
    }
    #[wasm_bindgen(getter, js_name = "privateKeyBase64")]
    pub fn private_key_base64(&self) -> Option<String> {
        self.private_key_base64.clone()
    }

    #[wasm_bindgen(setter, js_name = "privateKeyBase64")]
    pub fn set_private_key_base64(&mut self, private_key_base64: Option<String>) {
        self.private_key_base64 = private_key_base64;
    }
    #[wasm_bindgen(getter, js_name = "privateKeyPem")]
    pub fn private_key_pem(&self) -> Option<String> {
        self.private_key_pem.clone()
    }

    #[wasm_bindgen(setter, js_name = "privateKeyPem")]
    pub fn set_private_key_pem(&mut self, private_key_pem: Option<String>) {
        self.private_key_pem = private_key_pem;
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> Option<String> {
        self.value.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_value(&mut self, value: Option<String>) {
        self.value = value;
    }

    #[wasm_bindgen(js_name = "toObject")]
    pub fn to_object(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        serde_wasm_bindgen::to_value(self)
    }
}

pub trait VerificationKey {
    fn from_fingerprint(fingerprint: &str) -> Result<Self, Error>
    where
        Self: Sized;

    fn get_suite_id() -> &'static str
    where
        Self: Sized;

    fn get_current_suite_id(&self) -> &'static str;

    fn get_suite_context() -> &'static str
    where
        Self: Sized;

    fn get_current_suite_context(&self) -> &'static str;

    fn get_controller(&self) -> &Option<String>;

    fn get_type(&self) -> String;

    fn get_private_key_content(&self) -> &Option<String>;
    fn get_public_key_content(&self) -> &String;

    fn export(&self, public_key: bool, private_key: bool, include_context: bool) -> KeyPair;
}

pub trait AgreementKey {
    fn get_suite_context() -> &'static str
    where
        Self: Sized;

    fn get_current_suite_context(&self) -> &'static str;

    fn get_controller(&self) -> &Option<String>;

    fn get_private_key_content(&self) -> &Option<String>;
    fn get_public_key_content(&self) -> &String;

    fn export(&self, public_key: bool, private_key: bool, include_context: bool) -> KeyPair;
}
