use smecs::prelude::*;

fn main() {
    // Entity initialization
    let mut parent = Entity::new();
    let mut child = Entity::new();

    child.add_component(Hello);

    parent.add_child(child);
    parent.add_component(World);

    parent.start();
}

#[derive(Debug)]
struct Hello;
impl Component for Hello{
    fn start(&mut self) {
        print!("Hello ");
    }
    fn update(&mut self) {}
}

#[derive(Debug)]
struct World;
impl Component for World{
    fn start(&mut self) {
        println!("World!")
    }
}
