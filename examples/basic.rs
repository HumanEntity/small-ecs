use smecs::World;

#[derive(Debug, Clone)]
struct Component {
    x: i32,
}

/// .
fn main() {
    let mut world = World::new();
    let entity1 = world.new_id();
    world.write(entity1, Component { x: 0 });
    let mut component = world.read::<Component>(entity1).unwrap();
    component.x = 5;
    world.write(entity1, component);
    println!("{:?}", world.read::<Component>(entity1));
}
