use depends::{derives::Operation, error::EarlyExit, DepRef, UpdateDerived};

use crate::internal::{NumberLike, NumberValueF32};

#[derive(Operation)]
pub struct Square;

impl<A: NumberLike> UpdateDerived<DepRef<'_, A>, Square> for NumberValueF32 {
    fn update(&mut self, value: DepRef<'_, A>) -> Result<(), EarlyExit> {
        self.value = value.data().value().powi(2);
        Ok(())
    }
}
