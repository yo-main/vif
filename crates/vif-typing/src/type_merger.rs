use std::any::Any;

use vif_objects::ast::Type;

pub trait TypeMerger {
    fn merge(&self, left: &Type, right: &Type) -> Option<Type>;
}

pub struct HardTypeMerger {}
pub struct SoftTypeMerger {}

impl TypeMerger for HardTypeMerger {
    fn merge(&self, left: &Type, right: &Type) -> Option<Type> {
        match left {
            Type::Int => match right {
                Type::Int => Some(Type::Int),
                Type::Float => Some(Type::Float),
                Type::Bool => Some(Type::Int),
                Type::Callable(_) => self.merge(right, left),
                _ => None,
            },
            Type::Float => match right {
                Type::Int => Some(Type::Float),
                Type::Float => Some(Type::Float),
                Type::Bool => Some(Type::Float),
                Type::Callable(_) => self.merge(right, left),
                _ => None,
            },
            Type::Bool => match right {
                Type::Bool => Some(Type::Bool),
                Type::Int => Some(Type::Int),
                Type::Float => Some(Type::Float),
                Type::Callable(_) => self.merge(right, left),
                _ => None,
            },
            Type::String => match right {
                Type::String => Some(Type::String),
                Type::Callable(_) => self.merge(right, left),
                _ => None,
            },
            Type::None => match right {
                Type::None => Some(Type::None),
                Type::Callable(_) => self.merge(right, left),
                _ => None,
            },
            Type::Callable(c1) => match right {
                Type::Callable(c2) => self.merge(
                    &c1.output.r#type.get_concrete_type(),
                    &c2.output.r#type.get_concrete_type(),
                ),
                _ => self.merge(&c1.output.r#type.get_concrete_type(), right),
            },
            Type::Unknown => None,
            Type::KeyWord => match right {
                _ => unreachable!(),
            },
        }
    }
}

impl TypeMerger for SoftTypeMerger {
    fn merge(&self, left: &Type, right: &Type) -> Option<Type> {
        match left {
            Type::Int => match right {
                Type::Int => Some(Type::Int),
                Type::Float => Some(Type::Float),
                Type::Bool => Some(Type::Int),
                Type::Callable(_) => self.merge(right, left),
                _ => Some(Type::Unknown),
            },
            Type::Float => match right {
                Type::Int => Some(Type::Float),
                Type::Float => Some(Type::Float),
                Type::Bool => Some(Type::Float),
                Type::Callable(_) => self.merge(right, left),
                _ => Some(Type::Unknown),
            },
            Type::Bool => match right {
                Type::Bool => Some(Type::Bool),
                Type::Int => Some(Type::Int),
                Type::Float => Some(Type::Float),
                Type::Callable(_) => self.merge(right, left),
                _ => Some(Type::Unknown),
            },
            Type::String => match right {
                Type::String => Some(Type::String),
                Type::Callable(_) => self.merge(right, left),
                _ => Some(Type::Unknown),
            },
            Type::None => match right {
                Type::None => Some(Type::None),
                Type::Callable(_) => self.merge(right, left),
                _ => Some(Type::Unknown),
            },
            Type::Callable(c1) => match right {
                Type::Callable(c2) => self.merge(
                    &c1.output.r#type.get_concrete_type(),
                    &c2.output.r#type.get_concrete_type(),
                ),
                _ => self.merge(&c1.output.r#type.get_concrete_type(), right),
            },
            Type::Unknown => Some(Type::Unknown),
            Type::KeyWord => match right {
                _ => unreachable!(),
            },
        }
    }
}
