#![allow(clippy::unused_unit)]

use std::{cell::RefCell, collections::HashMap, sync::Mutex};

use wasm_bindgen::prelude::*;

use once_cell::sync::Lazy;

static MRF: Lazy<Mutex<RefCell<Mrf>>> = Lazy::new(|| {
    Mutex::new(RefCell::new(Mrf {
        ..Default::default()
    }))
});

#[wasm_bindgen]
pub fn submit_input(s: String) {
    MRF.lock().unwrap().borrow_mut().stuff(s);
}

#[wasm_bindgen]
pub fn get_results() -> usize {
    MRF.lock().unwrap().borrow_mut().total()
}

#[derive(Default)]
pub struct Mrf {
    stuff: HashMap<String, String>,
}

impl Mrf {
    pub fn stuff(&mut self, s: String) {
        self.stuff.insert(s.clone(), s);
    }

    pub fn total(&mut self) -> usize {
        let r = self.stuff.len();
        // this could be cleaner but illustrates the thought
        self.stuff.clear();
        r
    }
}
