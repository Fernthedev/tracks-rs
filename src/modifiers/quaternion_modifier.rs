use std::{any::Any, cell::RefCell};

use super::{
    operation::Operation,
    {Modifier, ModifierBase},
};
use crate::values::{base_provider_context::BaseProviderContext, AbstractValueProvider, ValueProvider};
use glam::{EulerRot, Quat, Vec3};

pub struct QuaternionModifier {
    raw_point: Option<Quat>,
    raw_vector_point: Option<Vec3>,
    reusable_array: RefCell<Vec<f32>>,
    values: Option<Vec<ValueProvider>>,
    modifiers: Vec<Box<dyn ModifierBase<Value = Quat>>>,
    operation: Operation,
}

impl QuaternionModifier {
    pub fn new(
        point: Option<Quat>,
        vector_point: Option<Vec3>,
        values: Option<Vec<ValueProvider>>,
        modifiers: Vec<Box<dyn ModifierBase<Value = Quat>>>,
        operation: Operation,
    ) -> Self {
        Self {
            raw_point: point,
            raw_vector_point: vector_point,
            reusable_array: RefCell::new(vec![0.0; Self::VALUE_COUNT]),
            values,
            modifiers,
            operation,
        }
    }

    fn translate_euler(&self, values: &Vec<ValueProvider>, context: &BaseProviderContext) -> Vec3 {
        let mut i = 0;
        for value in values {
            for v in value.values(context) {
                self.reusable_array.borrow_mut()[i] = v;
                i += 1;
                if i >= Self::VALUE_COUNT {
                    break;
                }
            }
        }
        Vec3::new(
            self.reusable_array.borrow()[0],
            self.reusable_array.borrow()[1],
            self.reusable_array.borrow()[2],
        )
    }

    pub fn get_vector_point(&self, context: &BaseProviderContext) -> Vec3 {
        let original_point = self
            .raw_vector_point
            .unwrap_or_else(|| self.translate_euler(self.values.as_ref().unwrap(), context));
        self.modifiers.iter().fold(original_point, |acc, x| {
            let y = x.get_point(context);
            let quat_point = x.as_any().downcast_ref::<QuaternionModifier>().unwrap();
            match x.get_operation() {
                Operation::Add => acc + quat_point.get_vector_point(context),
                Operation::Sub => acc - quat_point.get_vector_point(context),
                Operation::Mul => acc * quat_point.get_vector_point(context),
                Operation::Div => acc / quat_point.get_vector_point(context),
                Operation::None => quat_point.get_vector_point(context),
            }
        })
    }
}

impl ModifierBase for QuaternionModifier {
    type Value = Quat;

    fn get_point(&self, context: &BaseProviderContext) -> Quat {
        if self.modifiers.len() > 0 {
            self.get_raw_point()
        } else {
            let vector_point = self.get_vector_point(context);
            Quat::from_euler(
                EulerRot::ZXY,
                vector_point.x,
                vector_point.y,
                vector_point.z,
            )
        }
    }

    fn get_raw_point(&self) -> Quat {
        self.raw_point.unwrap_or(Quat::IDENTITY)
    }

    fn translate(&self, values: &[f32]) -> Quat {
        Quat::from_euler(EulerRot::ZXY, values[0], values[1], values[2])
    }

    fn get_operation(&self) -> Operation {
        self.operation
    }
    

 
}

impl Modifier for QuaternionModifier {
    const VALUE_COUNT: usize = 3;
}
