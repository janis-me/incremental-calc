use depends::{UpdateInput, derives::Value};
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

use super::{NumberLike, NumberValueF32, NumberValueI32};

#[derive(Default, Debug, PartialEq)]
pub struct VectorValue<T: NumberLike> {
    pub x: T,
    pub y: T,
}

impl<T: NumberLike> VectorValue<T> {
    pub fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }

    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

pub trait VectorLike {
    fn value(&self) -> VectorValue<NumberValueF32>;
}

#[derive(Value, Default)]
pub struct VectorValueF32 {
    pub value: VectorValue<NumberValueF32>,
}

#[derive(Value, Default)]
pub struct VectorValueI32 {
    pub value: VectorValue<NumberValueI32>,
}

impl Hash for VectorValueF32 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl Hash for VectorValueI32 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl VectorValueF32 {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            value: VectorValue::new(NumberValueF32::new(x), NumberValueF32::new(y)),
        }
    }
}

impl VectorLike for VectorValueF32 {
    fn value(&self) -> VectorValue<NumberValueF32> {
        VectorValue {
            x: NumberValueF32::new(self.value.x.value()),
            y: NumberValueF32::new(self.value.y.value()),
        }
    }
}

impl VectorLike for VectorValueI32 {
    fn value(&self) -> VectorValue<NumberValueF32> {
        VectorValue {
            x: NumberValueF32::new(self.value.x.value()),
            y: NumberValueF32::new(self.value.y.value()),
        }
    }
}

impl UpdateInput for VectorValueF32 {
    type Update = VectorValue<NumberValueF32>;

    fn update_mut(&mut self, update: Self::Update) {
        self.value = update;
    }
}

impl UpdateInput for VectorValueI32 {
    type Update = VectorValue<NumberValueI32>;

    fn update_mut(&mut self, update: Self::Update) {
        self.value = update;
    }
}

impl<T: NumberLike> Display for VectorValue<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x.value(), self.y.value())
    }
}
