use super::TypeStr;

/**Component trait*/
pub trait Component: std::fmt::Debug + TypeStr{
    /**Function for updating*/
    fn update(&mut self){}
    /**Function for setup*/
    fn start(&mut self){}
}
