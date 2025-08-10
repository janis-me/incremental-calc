use depends::{derives::Operation, error::EarlyExit, DepRef2, DepRef3, UpdateDerived, UpdateInput};

use crate::internal::{NumberLike, NumberValueF32, NumberValueI32, VectorLike, VectorValueF32};

#[derive(Operation)]
pub struct Add;

impl<A: NumberLike, B: NumberLike> UpdateDerived<DepRef2<'_, A, B>, Add> for NumberValueF32 {
    fn update(&mut self, value: DepRef2<'_, A, B>) -> Result<(), EarlyExit> {
        self.value = value.0.data().value() + value.1.data().value();
        Ok(())
    }
}

impl<A: NumberLike, B: NumberLike> UpdateDerived<DepRef2<'_, A, B>, Add> for NumberValueI32 {
    fn update(&mut self, value: DepRef2<'_, A, B>) -> Result<(), EarlyExit> {
        self.value = value.0.data().value() as i32 + value.1.data().value() as i32;
        Ok(())
    }
}

impl<A: NumberLike, B: NumberLike, C: NumberLike> UpdateDerived<DepRef3<'_, A, B, C>, Add>
    for NumberValueF32
{
    fn update(&mut self, value: DepRef3<'_, A, B, C>) -> Result<(), EarlyExit> {
        self.value = value.0.data().value() + value.1.data().value() + value.2.data().value();
        Ok(())
    }
}

impl<A: NumberLike, B: NumberLike, C: NumberLike> UpdateDerived<DepRef3<'_, A, B, C>, Add>
    for NumberValueI32
{
    fn update(&mut self, value: DepRef3<'_, A, B, C>) -> Result<(), EarlyExit> {
        self.value = value.0.data().value() as i32
            + value.1.data().value() as i32
            + value.2.data().value() as i32;
        Ok(())
    }
}

impl<A: VectorLike, B: NumberLike> UpdateDerived<DepRef2<'_, A, B>, Add> for VectorValueF32 {
    fn update(&mut self, value: DepRef2<'_, A, B>) -> Result<(), EarlyExit> {
        let vector = value.0.data().value();
        let number = value.1.data();

        self.value.x.update_mut(vector.x.value() + number.value());
        self.value.y.update_mut(vector.y.value() + number.value());

        // Update the Add operation result or store it somewhere
        // This depends on what Add is supposed to represent
        Ok(())
    }
}
