use chrono::prelude::DateTime;
use chrono::Utc;
use std::any::TypeId;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct InputNode {
    pub value: i32,
}

#[wasm_bindgen]
impl InputNode {
    #[wasm_bindgen(constructor)]
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}

#[wasm_bindgen]
pub struct DerivedNode {
    inputs: Vec<i32>,
    operation: String,
}

#[wasm_bindgen]
impl DerivedNode {
    #[wasm_bindgen(constructor)]
    pub fn new(inputs: Vec<i32>, operation: String) -> Self {
        Self { inputs, operation }
    }

    #[wasm_bindgen(getter)]
    pub fn inputs(&self) -> Vec<i32> {
        self.inputs.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn operation(&self) -> String {
        self.operation.clone()
    }
}

#[wasm_bindgen]
pub struct OutputNode {
    pub input: i32,
}

#[wasm_bindgen]
impl OutputNode {
    #[wasm_bindgen(constructor)]
    pub fn new(input: i32) -> Self {
        Self { input }
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum ResultNodeKind {
    Input,
    Derived,
    Output,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct ResultNode {
    pub value: i32,
    pub kind: ResultNodeKind,
    pub(crate) last_updated: DateTime<Utc>,
}

#[wasm_bindgen]
impl ResultNode {
    #[wasm_bindgen(getter)]
    pub fn last_updated(&self) -> String {
        self.last_updated.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}
