//! `small-ecs` called `smecs` is designed to be a simple and lightweight ECS
//! implementation. It is designed to be easy to use and maintain.
//! As main inspiration of implementation I borrowed from [@UnitOfTime](https://github.com/unitoftime)'s `golang` ecs package.

use std::{
    any::{type_name, Any},
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};

/**
 * The __Id__ is a unique identifier for a entity.
 * */
pub type Id = usize;

/// __BasicStorage__ is simple struct for holding components of one type.
#[derive(Debug, Default)]
pub struct BasicStorage {
    components: HashMap<Id, Rc<RefCell<dyn Any>>>,
}

impl BasicStorage {
    /// Default constructor.
    #[inline]
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    /// This method writes to the HashMap overriding its value. e. g.
    /// ```rust
    ///use smecs::BasicStorage;
    /// let mut storage = BasicStorage::new();
    /// storage.write(0, 1);
    /// ```
    /// This example writes the value 1 to the HashMap with key 0.
    pub fn write(&mut self, id: Id, val: impl Any) {
        self.components.insert(id, Rc::new(RefCell::new(val)));
    }

    /// This method reads value from HashMap with certain entity id. e. g.
    /// ```rust
    ///use smecs::BasicStorage;
    ///let mut storage = BasicStorage::new();
    ///storage.read::<i32>(0);
    /// ```
    /// This example reads the value from the HashMap with key 0.
    pub fn read<T: 'static + std::clone::Clone>(&self, id: Id) -> Option<T> {
        self.components
            .get(&id)
            .map(|component| component.borrow().downcast_ref::<T>().cloned().unwrap())
        // if let Some(component) = self.components.get(&id) {
        //     Some((*component.borrow().downcast_ref::<T>().unwrap()).clone())
        // } else {
        //     None
        // }
    }
}

/// __World__ is a container for storages of all entities.
#[derive(Debug, Default)]
pub struct World {
    id_counter: Id,
    reg: HashMap<String, Rc<RefCell<BasicStorage>>>,
}

impl World {
    /// Default constructor.
    #[inline]
    pub fn new() -> Self {
        Self {
            id_counter: 0,
            reg: HashMap::new(),
        }
    }

    /// `new_id` returns a new unique identifier for entity.
    /// ```rust
    ///use smecs::World;
    ///let mut world = World::new();
    ///world.new_id(); // 0
    /// ```
    pub fn new_id(&mut self) -> Id {
        let id = self.id_counter;
        self.id_counter += 1;
        id
    }

    /// `get_storage` returns a storage for certain component type.
    /// ```rust
    ///use smecs::World;
    ///let mut world = World::new();
    ///world.get_storage::<i32>(); // Empty BasicStorage
    /// ```
    /// This return `Rc<RefCell<BasicStorage>>` for operations on that storage.
    pub fn get_storage<T>(&mut self) -> Rc<RefCell<BasicStorage>> {
        let name = type_name::<T>();
        self.get_storage_by_name(name)
    }

    /// `get_storage_by_name` returns a storage for certain component type using *name* parameter.
    /// ```rust
    ///use smecs::World;
    ///let mut world = World::new();
    ///world.get_storage_by_name("ComponentType"); // Empty BasicStorage
    /// ```
    pub fn get_storage_by_name(&mut self, name: &str) -> Rc<RefCell<BasicStorage>> {
        if self.reg.get(name).is_none() {
            self.reg
                .insert(name.to_string(), Rc::new(RefCell::new(BasicStorage::new())));
        }
        self.reg.get(name).unwrap().clone()
    }

    /// `write` writes value to storage of certain type using entity id.
    /// ```rust
    /// use smecs::World;
    ///let mut world = World::new();
    ///world.write(0, 5);
    /// ```
    pub fn write<T: 'static>(&mut self, id: Id, val: T) {
        let storage = self.get_storage::<T>();
        storage.borrow_mut().write(id, val);
    }

    /// `read` function reads value from storage of certain type using entity id.
    /// ```rust
    /// use smecs::World;
    ///let mut world = World::new();
    ///world.write(0, 5);
    ///world.read::<i32>(0); // 5
    /// ```
    pub fn read<T: Clone + 'static>(&mut self, id: Id) -> Option<T> {
        let name = std::any::type_name::<T>();
        let storage = self.get_storage_by_name(name);

        let x = storage.borrow().read::<T>(id);
        x
    }

    /// `each` function iterates over all entities and applies `f` for it.
    /// ```rust
    ///use smecs::World;
    ///let mut world = World::new();
    ///world.write(0, 5);
    ///world.write(1, 3);
    ///world.each::<i32>(|world, id, component| {
    ///     println!("{component:?}");
    ///     component
    ///     }) // 5, 3
    /// ```
    pub fn each<T: Clone + 'static>(&mut self, mut f: impl FnMut(&mut World, Id, T) -> T) {
        let storage = self.get_storage::<T>();

        let keys = storage
            .borrow()
            .components
            .keys()
            .map(|k| k.clone())
            .collect::<Vec<_>>();
        for k in keys {
            let v = self.read::<T>(k).unwrap();
            let v = f(self, k, v);
            self.write(k, v);
        }
        // keys.iter().for_each(|k| {
        //     let k = k.clone();
        //     let v = self.read::<T>(k);
        //     let v = f(self, k, v);
        //     self.write(k, v);
        // });
        //     let v = (*comp.borrow().downcast_ref::<T>().unwrap()).clone();
        //     vals.push((k.clone(), f(self, k.clone(), v)));
        // }
        // vals.iter().for_each(|x| self.write(x.0, x.1.clone()));
    }
}

pub trait System {
    fn call_system(&self, world: &mut World) -> ();
}

impl<T: std::clone::Clone + 'static> System for fn(&mut World, Id, T) -> T {
    fn call_system(&self, world: &mut World) {
        world.each::<T>(*self);
    }
}

// impl<T: std::clone::Clone + 'static> System for dyn FnMut(&mut World, Id, T) -> T {
//     fn call_system(&self, world: &mut World) {
//         let f = Box::new(*self);
//         world.each::<T>();
//     }
// }

impl<T> System for T
where
    T: Fn(),
{
    fn call_system(&self, _world: &mut World) -> () {
        self()
    }
}

#[cfg(test)]
mod tests {
    use std::time;

    use super::*;

    #[derive(Debug, Clone)]
    struct Component {
        x: i32,
    }
    #[derive(Debug, Clone)]
    struct Component2 {
        x: f32,
    }

    #[test]
    fn add_component() {
        let mut world = World::new();
        let entity = world.new_id();
        world.write(entity, Component { x: 5 });
        println!("{:?}", world);
    }

    #[test]
    fn iter_over_components() {
        let mut world = World::new();
        let entity = world.new_id();
        world.write(entity, Component { x: 5 });
        world.write(entity, Component2 { x: 5.0 });
        world.each::<Component>(|world, id, component| {
            let v = world.read::<Component2>(id);
            println!("{component:?}\n{v:?}");
            component
        });
    }
    #[test]
    fn iter_over_components2() {
        let mut world = World::new();
        let mut entities = Vec::new();
        for _ in 0..100 {
            entities.push(world.new_id());
            world.write(entities.last().cloned().unwrap(), Component { x: 0 });
            world.write(entities.last().cloned().unwrap(), Component2 { x: 5.0 });
        }
        world.each::<Component>(|world, id1, mut component| {
            world.each::<Component2>(|_, id2, c| {
                if id1 == id2 {
                    component.x += c.x as i32;
                }
                c
            });
            println!("{component:#?}");
            component
        });
    }

    #[test]
    fn iter_over_components3() {
        let mut world = World::new();
        let mut entities = Vec::new();
        for _ in 0..100 {
            entities.push(world.new_id());
            world.write(entities.last().cloned().unwrap(), Component { x: 0 });
        }

        let start = time::Instant::now();
        world.each::<Component>(|w, i, c| {
            w.each::<Component>(|_, i2, c2| c2);
            c
        });
        let full = start.elapsed().as_secs_f32();
        println!("{full:?}");
    }
}
