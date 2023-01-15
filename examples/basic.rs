use smecs::prelude::*;



fn main() {
    // Container initialization
    let mut container = Container::new();

    // Entity initialization
    let mut entity = Entity::new();
    entity.add_component(HelloWorld);

    // Storing entity in container
    container.add(entity);

    container.start();
    container.update();
}

#[derive(Debug)]
pub struct HelloWorld;
impl Component for HelloWorld{
    fn start(&mut self) {
        print!("Hello ");
    }
    fn update(&mut self) {
        println!("World!");
    }
}
