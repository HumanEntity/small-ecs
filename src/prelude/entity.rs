use super::Component;


#[derive(Debug, Default)]
pub struct Entity{
    components: Vec<Box<dyn Component>>,
    childs: Vec<Entity>,
}
impl Entity{
    pub fn new() -> Self{
        Self{
            components: Vec::new(),
            childs: Vec::new(),
        }
    }
    
    pub fn add_component<T>(&mut self, component: T )
    where T: Component + 'static{
        self.components.push(Box::new(component));
    }

    pub fn remove_component<T>(&mut self, component: T) -> Result<Box<dyn Component>, &str>
    where T: Component {
        let t = component.type_str();
        for i in 0..self.components.len(){
            if self.components[i].type_str() == t{
                return Ok(self.components.remove(i));
            }
        }
        Err("Cannot remove component of this type")
    }

    pub fn add_child(&mut self, child: Entity) {
        self.childs.push(child);
    }

    pub fn remove_child(&mut self, indx: usize) -> Entity{
        self.childs.remove(indx)
    }
    pub fn update(&mut self) {
        for child in &mut self.childs {
            child.update();
        }
        for component in &mut self.components{
            component.update();
        }
    }
}
