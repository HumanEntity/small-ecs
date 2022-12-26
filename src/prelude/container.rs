use super::Entity;


#[derive(Debug, Default)]
pub struct Container{
    entities: Vec<Entity>,
}
impl Container{
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }
    pub fn add(&mut self, entity: Entity) -> usize {
        self.entities.push(entity);
        self.entities.len() - 1
    }
    pub fn remove(&mut self, index: usize) -> Entity {
        self.entities.remove(index)
    }
    pub fn update(&mut self) {
        for entity in &mut self.entities{
            entity.update();
        }
    }
}
