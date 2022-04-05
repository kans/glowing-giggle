#![allow(clippy::unused_unit)]
use std::{cell::RefCell, collections::HashMap, sync::Mutex};
use once_cell::sync::Lazy;

#[cfg(feature="neon-bindings")]
use neon::prelude::*;

#[cfg(not(feature="neon-bindings"))]
use wasm_bindgen::prelude::*;

static MRF: Lazy<Mutex<RefCell<Mrf>>> = Lazy::new(|| {
    Mutex::new(RefCell::new(Mrf {
        ..Default::default()
    }))
});

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

#[cfg(not(feature="neon-bindings"))]
#[wasm_bindgen]
pub fn submit_input(s: String) {
    MRF.lock().unwrap().borrow_mut().stuff(s);
}

#[cfg(not(feature="neon-bindings"))]
#[wasm_bindgen]
pub fn get_results() -> usize {
    MRF.lock().unwrap().borrow_mut().total()
}

#[cfg(feature = "neon-bindings")]
fn submit_input_neon(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let s = cx.argument::<JsString>(0)?.value(&mut cx);
    MRF.lock().unwrap().borrow_mut().stuff(s);
    Ok(cx.undefined())
}

#[cfg(feature = "neon-bindings")]
fn get_results_neon(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let total = MRF.lock().unwrap().borrow_mut().total();
    Ok(cx.number(total as f64))
}

#[cfg(feature = "neon-bindings")]
#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("submit_input", submit_input_neon)?;
    cx.export_function("get_results", get_results_neon)?;
    Ok(())
}
