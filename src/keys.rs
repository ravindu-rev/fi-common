use crate::error::Error;
use serde::{Deserialize, Serialize};
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
    pub context: Option<String>,
    #[wasm_bindgen(skip)]
    pub public_key_base58: Option<String>,
    #[wasm_bindgen(skip)]
    pub private_key_base58: Option<String>,
    #[wasm_bindgen(skip)]
    pub public_key_multibase: Option<String>,
    #[wasm_bindgen(skip)]
    pub private_key_multibase: Option<String>,
    #[wasm_bindgen(skip)]
    pub revoked: bool,
    #[wasm_bindgen(skip)]
    pub controller: Option<String>,
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

    #[wasm_bindgen(getter)]
    pub fn _type(&self) -> String {
        self._type.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_type(&mut self, _type: String) {
        self._type = _type;
    }

    #[wasm_bindgen(getter)]
    pub fn context(&self) -> Option<String> {
        self.context.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_context(&mut self, context: Option<String>) {
        self.context = context;
    }

    #[wasm_bindgen(getter)]
    pub fn public_key_base58(&self) -> Option<String> {
        self.public_key_base58.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_public_key_base58(&mut self, public_key_base58: Option<String>) {
        self.public_key_base58 = public_key_base58;
    }

    #[wasm_bindgen(getter)]
    pub fn private_key_base58(&self) -> Option<String> {
        self.private_key_base58.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_private_key_base58(&mut self, private_key_base58: Option<String>) {
        self.private_key_base58 = private_key_base58;
    }

    #[wasm_bindgen(getter)]
    pub fn public_key_multibase(&self) -> Option<String> {
        self.public_key_multibase.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_public_key_multibase(&mut self, public_key_multibase: Option<String>) {
        self.public_key_multibase = public_key_multibase;
    }

    #[wasm_bindgen(getter)]
    pub fn private_key_multibase(&self) -> Option<String> {
        self.private_key_multibase.clone()
    }

    #[wasm_bindgen(setter)]
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
