mod entity;
mod component;
mod container;

pub use entity::*;
pub use component::*;
pub use container::*;

pub trait TypeStr{
    fn type_str(&self) -> &'static str{
        core::any::type_name::<Self>()
    }
}
