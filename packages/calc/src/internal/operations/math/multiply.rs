use depends::{derives::Operation, error::EarlyExit, DepRef2, DepRef3, DepRef4, UpdateDerived};

use crate::internal::{NumberLike, NumberValueF32, NumberValueI32};

#[derive(Operation)]
pub struct Multiply;

impl<A: NumberLike, B: NumberLike> UpdateDerived<DepRef2<'_, A, B>, Multiply> for NumberValueF32 {
    fn update(&mut self, value: DepRef2<'_, A, B>) -> Result<(), EarlyExit> {
        self.value = value.0.data().value() * value.1.data().value();
        Ok(())
    }
}

impl<A: NumberLike, B: NumberLike> UpdateDerived<DepRef2<'_, A, B>, Multiply> for NumberValueI32 {
    fn update(&mut self, value: DepRef2<'_, A, B>) -> Result<(), EarlyExit> {
        self.value = (value.0.data().value() * value.1.data().value()) as i32;
        Ok(())
    }
}

impl<A: NumberLike, B: NumberLike, C: NumberLike> UpdateDerived<DepRef3<'_, A, B, C>, Multiply>
    for NumberValueF32
{
    fn update(&mut self, value: DepRef3<'_, A, B, C>) -> Result<(), EarlyExit> {
        self.value = value.0.data().value() * value.1.data().value() * value.2.data().value();
        Ok(())
    }
}

impl<A: NumberLike, B: NumberLike, C: NumberLike> UpdateDerived<DepRef3<'_, A, B, C>, Multiply>
    for NumberValueI32
{
    fn update(&mut self, value: DepRef3<'_, A, B, C>) -> Result<(), EarlyExit> {
        self.value =
            (value.0.data().value() * value.1.data().value() * value.2.data().value()) as i32;
        Ok(())
    }
}

impl<A: NumberLike, B: NumberLike, C: NumberLike, D: NumberLike>
    UpdateDerived<DepRef4<'_, A, B, C, D>, Multiply> for NumberValueF32
{
    fn update(&mut self, value: DepRef4<'_, A, B, C, D>) -> Result<(), EarlyExit> {
        self.value = value.0.data().value()
            * value.1.data().value()
            * value.2.data().value()
            * value.3.data().value();
        Ok(())
    }
}
