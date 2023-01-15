use smecs::prelude::*;

fn main() {
    // Containers initialization
    let mut first_container = Container::new();
    let mut second_container = Container::new();

    let mut entity = Entity::new();
    entity.add_component(HelloWorld);
    
    second_container.add(entity);
    first_container.add(second_container);

    first_container.start();
    first_container.update();
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
