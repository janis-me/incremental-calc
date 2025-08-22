use neal::{GraphError, Operation};
use std::hash::Hash;

#[derive(Clone, PartialEq, Hash, Debug)]
pub struct Number(pub i32);

// Operations for our Number type
pub struct Add;
impl Operation<(Number, Number), Number> for Add {
    fn execute(&self, inputs: &[&dyn neal::Value]) -> Result<Number, GraphError> {
        let a = inputs[0]
            .as_any()
            .downcast_ref::<Number>()
            .ok_or_else(|| GraphError::InvalidOperation("Expected Number".into()))?;
        let b = inputs[1]
            .as_any()
            .downcast_ref::<Number>()
            .ok_or_else(|| GraphError::InvalidOperation("Expected Number".into()))?;
        Ok(Number(a.0 + b.0))
    }

    fn input_count(&self) -> Option<usize> {
        Some(2)
    }
}

pub struct Multiply;
impl Operation<(Number, Number), Number> for Multiply {
    fn execute(&self, inputs: &[&dyn neal::Value]) -> Result<Number, GraphError> {
        let a = inputs[0]
            .as_any()
            .downcast_ref::<Number>()
            .ok_or_else(|| GraphError::InvalidOperation("Expected Number".into()))?;
        let b = inputs[1]
            .as_any()
            .downcast_ref::<Number>()
            .ok_or_else(|| GraphError::InvalidOperation("Expected Number".into()))?;
        Ok(Number(a.0 * b.0))
    }

    fn input_count(&self) -> Option<usize> {
        Some(2)
    }
}
