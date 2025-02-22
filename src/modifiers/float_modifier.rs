use crate::values::BaseValues;

use super::{
    operation::Operation,
    {Modifier, ModifierBase},
};

pub struct FloatModifier {
    raw_point: Option<f32>,
    values: Option<Vec<Box<dyn BaseValues>>>,
    modifiers: Vec<Box<dyn ModifierBase<Value = f32>>>,
    operation: Operation,
}

impl FloatModifier {
    pub fn new(
        point: Option<f32>,
        values: Option<Vec<Box<dyn BaseValues>>>,
        modifiers: Vec<Box<dyn ModifierBase<Value = f32>>>,
        operation: Operation,
    ) -> Self {
        Self {
            raw_point: point,
            values,
            modifiers,
            operation,
        }
    }
}

impl ModifierBase for FloatModifier {
    type Value = f32;

    fn get_point(&self) -> f32 {
        let original_point = self
            .raw_point
            .unwrap_or_else(|| self.convert(self.values.as_ref().unwrap()));
        self.modifiers
            .iter()
            .fold(original_point, |acc, x| match x.get_operation() {
                Operation::Add => acc + x.get_point(),
                Operation::Sub => acc - x.get_point(),
                Operation::Mul => acc * x.get_point(),
                Operation::Div => acc / x.get_point(),
                Operation::None => x.get_point(),
            })
    }

    fn get_raw_point(&self) -> f32 {
        self.raw_point.unwrap_or(0.0)
    }

    fn translate(&self, values: &[f32]) -> f32 {
        values[0]
    }

    fn get_operation(&self) -> Operation {
        self.operation
    }
}

impl Modifier for FloatModifier {
    const VALUE_COUNT: usize = 1;
}
