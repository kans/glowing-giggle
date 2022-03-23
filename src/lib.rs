#![allow(clippy::unused_unit)]

use std::collections::HashMap;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn get_mrf() -> Mrf {
    Mrf {
        ..Default::default()
    }
}

#[derive(Default)]
#[wasm_bindgen]
pub struct Mrf {
    stuff: HashMap<String, String>,
}

#[wasm_bindgen]
impl Mrf {
    pub fn stuff(&mut self, s: String) {
        self.stuff.insert(s.clone(), s);
    }

    pub fn what(&self) {
        log(&format!("stuffed: {}", self.stuff.len()));
    }
}
