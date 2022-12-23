mod entity;
mod component;

pub use entity::*;
pub use component::*;

pub trait TypeStr{
    fn type_str(&self) -> &'static str{
        core::any::type_name::<Self>()
    }
}
