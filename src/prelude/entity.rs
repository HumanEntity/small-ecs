use super::Component;

/**Entity struct implementation*/
#[derive(Debug, Default)]
pub struct Entity{
    components: Vec<Box<dyn Component>>,
    children: Vec<Entity>,
}
impl Entity{
    /**Constructor*/
    pub fn new() -> Self{
        Self{
            components: Vec::new(),
            children: Vec::new(),
        }
    }
    
    /**Adding component*/
    pub fn add_component<T>(&mut self, component: T )
    where T: Component + 'static{
        self.components.push(Box::new(component));
    }

    /**Removing component*/
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

    /**Adding children*/
    pub fn add_child(&mut self, child: Entity) -> usize{
        self.children.push(child);
        self.children.len() - 1
    }
    
    /**Removing children*/
    pub fn remove_child(&mut self, indx: usize) -> Entity{
        self.children.remove(indx)
    }

    /**Updating children and components*/
    pub fn update(&mut self) {
        for child in &mut self.children {
            child.update();
        }
        for component in &mut self.components{
            component.update();
        }
    }

    /**Setup*/
    pub fn start(&mut self) {
        for child in &mut self.children {
            child.start();
        }
        for component in &mut self.components{
            component.start();
        }
    }
}
