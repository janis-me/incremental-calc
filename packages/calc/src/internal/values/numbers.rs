use depends::{UpdateInput, derives::Value};
use std::hash::{Hash, Hasher};

pub trait NumberLike {
    fn value(&self) -> f32;

    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value().to_bits().hash(state);
    }
}

#[derive(Value, Default)]
pub struct NumberValueF32 {
    pub value: f32,
}

#[derive(Value, Default)]
pub struct NumberValueI32 {
    pub value: i32,
}

impl NumberValueF32 {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}

impl NumberValueI32 {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}

impl NumberLike for NumberValueF32 {
    fn value(&self) -> f32 {
        self.value
    }
}

impl NumberLike for NumberValueI32 {
    fn value(&self) -> f32 {
        self.value as f32
    }
}

// By implementing UpdateInput, we can change the value of this node from
// outside of the graph.
impl UpdateInput for NumberValueF32 {
    type Update = f32;

    fn update_mut(&mut self, update: Self::Update) {
        self.value = update;
    }
}

impl UpdateInput for NumberValueI32 {
    type Update = i32;

    fn update_mut(&mut self, update: Self::Update) {
        self.value = update;
    }
}
