use super::Entity;


/**Container implementation*/
#[derive(Debug, Default)]
pub struct Container{
    entities: Vec<Entity>,
}
impl Container{
    /**Constructor*/
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }
    /**Adding Entities*/
    pub fn add(&mut self, entity: Entity) -> usize {
        self.entities.push(entity);
        self.entities.len() - 1
    }
    /**Removing Entities*/
    pub fn remove(&mut self, index: usize) -> Entity {
        self.entities.remove(index)
    }
    /**Updating Entities*/
    pub fn update(&mut self) {
        for entity in &mut self.entities{
            entity.update();
        }
    }
    /**Setup*/
    pub fn start(&mut self) {
        for entity in &mut self.entities{
            entity.start();
        }
    }
}
