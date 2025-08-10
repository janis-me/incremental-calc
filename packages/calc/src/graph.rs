use std::rc::Rc;

use depends::{Dependencies2, DerivedNode, DiagnosticVisitor, InputNode, Resolve as _};
use wasm_bindgen::prelude::*;

use crate::internal::{Add, Multiply, NumberValueF32, NumberValueI32};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen(getter_with_clone)]
pub struct RunResponse {
    pub value: f32,
    pub visited: Vec<usize>,
    pub recalculated: Vec<usize>,
}

#[wasm_bindgen]
pub fn run() -> RunResponse {
    let a = InputNode::new(NumberValueF32::new(1_f32));
    let b = InputNode::new(NumberValueF32::new(2_f32));
    let c = InputNode::new(NumberValueF32::new(32_f32));

    let a_plus_b = DerivedNode::new(
        Dependencies2::new(Rc::clone(&a), Rc::clone(&b)),
        Add,
        NumberValueF32::default(),
    );

    let times_c = DerivedNode::new(
        Dependencies2::new(Rc::clone(&a_plus_b), Rc::clone(&c)),
        Multiply,
        NumberValueF32::default(),
    );

    let mut diagnostics_visitor = DiagnosticVisitor::new();

    let res = times_c.resolve(&mut diagnostics_visitor).unwrap();

    let visited = diagnostics_visitor
        .visitor
        .into_iter()
        .collect::<Vec<usize>>();

    let recalculated = diagnostics_visitor
        .recalculated
        .into_iter()
        .collect::<Vec<usize>>();

    return RunResponse {
        value: res.value().value,
        visited,
        recalculated,
    };
}
