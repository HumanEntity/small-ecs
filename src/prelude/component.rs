use super::TypeStr;


pub trait Component: std::fmt::Debug + TypeStr{
    fn update(&mut self);
}
