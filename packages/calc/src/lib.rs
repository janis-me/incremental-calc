use core::panic;
use std::{hash::Hash, vec};

use neal::{ComputationStats, DerivedNode, Graph, GraphError, InputNode, Operation, OutputNode};

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
#[derive(Clone, PartialEq, Debug)]
struct NumberVector(Vec<f32>);

impl Hash for NumberVector {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for num in &self.0 {
            num.to_bits().hash(state);
        }
    }
}

struct Add;
impl Operation<(NumberVector, NumberVector), NumberVector> for Add {
    fn execute(&self, inputs: &[&dyn neal::Value]) -> Result<NumberVector, GraphError> {
        let a = inputs[0]
            .as_any()
            .downcast_ref::<NumberVector>()
            .ok_or_else(|| GraphError::InvalidOperation("Expected NumberVector".into()))?;
        let b = inputs[1]
            .as_any()
            .downcast_ref::<NumberVector>()
            .ok_or_else(|| GraphError::InvalidOperation("Expected NumberVector".into()))?;
        if a.0.len() != b.0.len() {
            return Err(GraphError::InvalidOperation(
                "Mismatched vector lengths".into(),
            ));
        }
        let result = a.0.iter().zip(&b.0).map(|(x, y)| x + y).collect();
        Ok(NumberVector(result))
    }
}

struct Multiply;
impl Operation<(NumberVector, NumberVector), NumberVector> for Multiply {
    fn execute(&self, inputs: &[&dyn neal::Value]) -> Result<NumberVector, GraphError> {
        let a = inputs[0]
            .as_any()
            .downcast_ref::<NumberVector>()
            .ok_or_else(|| GraphError::InvalidOperation("Expected NumberVector".into()))?;
        let b = inputs[1]
            .as_any()
            .downcast_ref::<NumberVector>()
            .ok_or_else(|| GraphError::InvalidOperation("Expected NumberVector".into()))?;
        if a.0.len() != b.0.len() {
            return Err(GraphError::InvalidOperation(
                "Mismatched vector lengths".into(),
            ));
        }
        let result = a.0.iter().zip(&b.0).map(|(x, y)| x * y).collect();
        Ok(NumberVector(result))
    }
}

#[derive(Serialize, Deserialize)]
pub enum ComputationError {
    NodeNotFound(String),
    InvalidOperation(String),
    ComputationError(String),
}

#[derive(Serialize, Deserialize)]
pub struct ComputeResult {
    pub nodes_processed: usize,
    pub duration_ms: f64,
    pub iterations: u32,
    pub output_value: Vec<f32>,
    pub first_error: Option<ComputationError>,
    pub error_node: Option<u64>,
    pub aborted: bool,
}

#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn run() -> JsValue {
    let res = execute_graph();

    match res {
        Ok(compute_stats) => {
            match compute_stats
                .output_value
                .as_ref()
                .and_then(|val| val.as_any().downcast_ref::<NumberVector>())
            {
                Some(output_vector) => {
                    log(&format!("Output: {:?}", output_vector.0));
                    let res = ComputeResult {
                        nodes_processed: compute_stats.nodes_processed,
                        duration_ms: compute_stats.duration_ms,
                        iterations: compute_stats.iterations,
                        output_value: output_vector.0.clone(),
                        first_error: compute_stats
                            .first_error
                            .map(|e| ComputationError::ComputationError(e.to_string())),
                        error_node: compute_stats.error_node.map(|id| id.0),
                        aborted: compute_stats.aborted,
                    };

                    serde_wasm_bindgen::to_value(&res).unwrap()
                }
                None => {
                    panic!("Output is not a NumberVector");
                }
            }
        }
        Err(err) => panic!("Graph computation failed: {}", err.to_string()),
    }
}

fn execute_graph() -> Result<ComputationStats, GraphError> {
    let mut graph = Graph::new();

    // Build a simple computation: (5 + 3) * 2 using new API
    let a = graph.add_node(InputNode::new(NumberVector(vec![5.0, 3.0])));
    let b = graph.add_node(InputNode::new(NumberVector(vec![3.0, 3.0])));
    let c = graph.add_node(InputNode::new(NumberVector(vec![2.0, 2.0])));

    let sum1 = graph.add_node(DerivedNode::new(vec![a.id(), b.id()], Add));
    let sum2 = graph.add_node(DerivedNode::new(vec![sum1.id(), c.id()], Multiply));
    let _output: neal::NodeHandle<NumberVector> = graph.add_node(OutputNode::new(sum2.id()));

    graph.compute()
}
