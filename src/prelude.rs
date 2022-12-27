mod entity;
mod component;
mod container;

pub use entity::*;
pub use component::*;
pub use container::*;

/**Trait for getting struct name at runtime*/
pub trait TypeStr{
    fn type_str(&self) -> &'static str{
        core::any::type_name::<Self>()
    }
}
