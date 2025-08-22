use crate::api::{DerivedNode, InputNode, OutputNode, ResultNode, ResultNodeKind};
use crate::graph::{Add, Multiply, Number};

use chrono::prelude::DateTime;
use chrono::Utc;
use neal::{
    ComputationStats, DerivedNode as NealDerivedNode, GraphError, InputNode as NealInputNode, Neal,
    NodeId, NodeType, Observer, OutputNode as NealOutputNode,
};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

const FPS: i32 = 1;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn set_timeout(f: &Closure<dyn FnMut()>, timeout: i32) {
    window()
        .set_timeout_with_callback_and_timeout_and_arguments_0(f.as_ref().unchecked_ref(), timeout)
        .expect("should register `setTimeout` OK");
}

pub struct SimpleObserver;

impl Observer for SimpleObserver {
    fn on_value_changed(&mut self, _node_id: NodeId, _old: Option<u64>, _new: u64) {}
    fn on_node_added(&mut self, _node_id: NodeId) {}
    fn on_node_removed(&mut self, _node_id: NodeId) {}
    fn on_computation_started(&mut self) {}
    fn on_computation_finished(&mut self, _stats: ComputationStats) {}
    fn on_error(&mut self, error: &GraphError) {
        println!("Error: {}", error);
    }
    fn on_node_computation_started(&mut self, node_id: NodeId) {
        println!("Node computation started: {:?}", node_id);
    }
    fn on_node_computation_finished(&mut self, _node_id: NodeId, _success: bool) {}
}

#[wasm_bindgen]
pub struct Graph {
    graph: Rc<RefCell<Neal>>,
    nodes_subscription: Option<js_sys::Function>,
}

#[wasm_bindgen]
impl Graph {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            graph: Rc::new(RefCell::new(Neal::with_observer(SimpleObserver))),
            nodes_subscription: None,
        }
    }

    pub fn run(&mut self) -> Result<(), JsError> {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();
        let graph = self.graph.clone();
        let nodes_subscription = self.nodes_subscription.clone();

        *g.borrow_mut() = Some(Closure::new(move || {
            log(&format!(
                "Running for nodes: {}",
                graph.borrow().nodes.as_str()
            ));
            let res = graph.borrow_mut().compute();

            match res {
                Ok(stats) => {
                    log(&format!("Computation successful: {:?}", stats));
                    if let Some(ref callback) = nodes_subscription {
                        graph.borrow().with_nodes(|nodes| {
                            let result_nodes: Vec<ResultNode> = nodes
                                .iter()
                                .map(|(_, node)| ResultNode {
                                    value: node.get_value::<i32>().unwrap().clone(),
                                    kind: match node.kind() {
                                        NodeType::Input { .. } => ResultNodeKind::Input,
                                        NodeType::Derived { .. } => ResultNodeKind::Derived,
                                        NodeType::Output { .. } => ResultNodeKind::Output,
                                    },
                                    last_updated: DateTime::<Utc>::from_timestamp_nanos(
                                        node.last_update().unix_timestamp_nanos() as i64,
                                    ),
                                })
                                .collect();

                            let _ = callback.call1(
                                &JsValue::NULL,
                                &JsValue::from(format!("{:?}", result_nodes)),
                            );
                        });
                    }
                }
                Err(e) => {
                    log(&format!("Computation error: {}", e));
                }
            }

            // Schedule ourself for another setTimeout callback.
            set_timeout(f.borrow().as_ref().unwrap(), 1000 / FPS);
        }));

        set_timeout(g.borrow().as_ref().unwrap(), 1000 / FPS);
        Ok(())
    }

    #[wasm_bindgen(js_name = "addInput")]
    pub fn add_input(&mut self, node: InputNode) {
        let input_node = NealInputNode::new(Number(node.value));
        self.graph.borrow_mut().add_node(input_node);
    }

    #[wasm_bindgen(js_name = "addDerived")]
    pub fn add_derived(&mut self, node: DerivedNode) {
        let inputs: Vec<NodeId> = node.inputs().into_iter().map(|id| NodeId(id)).collect();

        let node = match node.operation().as_str() {
            "Add" => NealDerivedNode::new(inputs, Add),
            "Multiply" => NealDerivedNode::new(inputs, Multiply),
            _ => panic!("Unknown operation"),
        };

        self.graph.borrow_mut().add_node(node);
    }

    #[wasm_bindgen(js_name = "addOutput")]
    pub fn add_output(&mut self, node: OutputNode) {
        let output_node = NealOutputNode::<Number>::new(NodeId(node.input));
        self.graph.borrow_mut().add_node(output_node);
    }

    // TODO: properly type the callback
    #[wasm_bindgen(js_name = "subscribeToNodes")]
    pub fn subscribe_to_nodes(&mut self, callback: js_sys::Function) {
        self.nodes_subscription = Some(callback);
    }

    #[wasm_bindgen(js_name = "getNodes")]
    pub fn get_nodes(&self) -> Vec<ResultNode> {
        self.graph.borrow().with_nodes(|nodes| {
            nodes
                .iter()
                .map(|(_, node)| ResultNode {
                    value: node.get_value::<i32>().unwrap().clone(),
                    kind: match node.kind() {
                        NodeType::Input { .. } => ResultNodeKind::Input,
                        NodeType::Derived { .. } => ResultNodeKind::Derived,
                        NodeType::Output { .. } => ResultNodeKind::Output,
                    },
                    last_updated: DateTime::<Utc>::from_timestamp_nanos(
                        node.last_update().unix_timestamp_nanos() as i64,
                    ),
                })
                .collect()
        })
    }
}
