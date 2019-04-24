use std::collections::HashMap;

use wasm_bindgen::prelude::*;
use rbx_dom_weak::{RbxTree, RbxId, RbxValue, RbxInstance, RbxInstanceProperties};
use serde_derive::{Serialize, Deserialize};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct JsRbxId(RbxId);

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct JsRbxValue(RbxValue);

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct JsRbxTree(RbxTree);

#[wasm_bindgen]
impl JsRbxTree {
    pub fn get_root_id(&self) -> JsValue {
        JsValue::from_serde(&self.0.get_root_id()).unwrap()
    }

    pub fn get_instance(&self, id_source: &JsValue) -> Option<JsRbxInstance> {
        let id: RbxId = id_source.into_serde().unwrap();
        self.0.get_instance(id).map(|value| JsRbxInstance(value.clone()))
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct JsRbxInstance(RbxInstance);

#[wasm_bindgen]
impl JsRbxInstance {
    pub fn name(&self) -> String {
        self.0.name.to_string()
    }

    pub fn class_name(&self) -> String {
        self.0.class_name.to_string()
    }

    pub fn properties(&self) -> JsValue {
        let properties: HashMap<String, JsRbxValue> = self.0.properties
            .iter()
            .map(|(key, value)| (key.clone(), JsRbxValue(value.clone())))
            .collect();

        JsValue::from_serde(&properties).unwrap()
    }

    pub fn get_children_ids(&self) -> JsValue {
        let ids: Vec<JsRbxId> = self.0
            .get_children_ids()
            .iter()
            .cloned()
            .map(|value| JsRbxId(value))
            .collect();

        JsValue::from_serde(&ids).unwrap()
    }
}

#[wasm_bindgen]
pub fn parse_xml(source: &str) -> JsRbxTree {
    let root_instance = RbxInstanceProperties {
        name: String::from("Root"),
        class_name: String::from("DataModel"),
        properties: HashMap::new(),
    };

    let mut tree = RbxTree::new(root_instance);
    let root_id = tree.get_root_id();

    rbx_xml::decode_str(&mut tree, root_id, source)
        .expect("Couldn't decode!");

    JsRbxTree(tree)
}