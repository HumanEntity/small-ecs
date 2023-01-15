use comps::SimpleComponent;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use smecs::prelude::*;

mod comps{
    use crate::*;

    #[derive(Debug)]
    pub struct SimpleComponent{
        pub foo: u8,
    }
    impl Component for SimpleComponent {
        fn start(&mut self) {
            self.foo = 0;
        }
        fn update(&mut self) {
            self.foo += 1;
        }
    }
}

fn hundred_entities_benchmark(c: &mut Criterion) {
    c.bench_function("hundred_entities_adding_benchmark", |b| {
        b.iter(|| {
            let mut container = black_box(Container::new());
            for _ in 0..100 {
                container.add(Entity::new());
            }
        })
    })
    .bench_function("hundred_entities_updating_benchmark", |b| b.iter(|| {
        let mut container = black_box(Container::new());
        for _ in 0..100 {
            let mut entity = Entity::new();
            entity.add_component(SimpleComponent{foo: 0});
            container.add(entity);
        }
        container.update();
    }));
}

criterion_group!(benches, hundred_entities_benchmark);
criterion_main!(benches);
