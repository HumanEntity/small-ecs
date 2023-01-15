use super::TypeStr;

pub trait Containable: std::fmt::Debug + std::any::Any + TypeStr{
    fn start(&mut self);
    fn update(&mut self);
}

/**Container implementation*/
#[derive(Debug, Default)]
pub struct Container{
    containables: Vec<Box<dyn Containable>>
}
impl Container{
    /**Constructor*/
    pub fn new() -> Self {
        Self {
            containables: Vec::new(),
        }
    }
    
    /**Adding Entities and other Containers*/
    #[cfg(not(feature = "static-dispatch"))]
    pub fn add<T>(&mut self, containable: T) where T: Containable + 'static {
        self.containables.push(Box::new(containable));
    }
    #[cfg(feature = "static-dispatch")]
    pub fn add(&mut self, containable: impl Containable) {
        self.containables.push(Box::new(containable));
    }
    /**Removing Entities and other Containers*/
    /*pub fn remove<T>(&mut self, containable: T) -> Option<T> where T: Containable + 'static + From<Box<dyn Containable>> {
        let t = containable.type_str();
        for i in 0..self.containables.len(){
            if self.containables[i].type_str() == t{
                return Some(self.containables.remove(i).into());
            }
        }
        None
    }*/
    pub fn remove(&mut self, indx: usize) -> Box<dyn Containable> {
        self.containables.remove(indx)
    }
}
impl Containable for Container {
    /**Updating Entities*/
    fn update(&mut self) {
        for containable in &mut self.containables{
            containable.update();
        }
    }
    /**Setup*/
    fn start(&mut self) {
        for containable in &mut self.containables{
            containable.start();
        }
    }
}
