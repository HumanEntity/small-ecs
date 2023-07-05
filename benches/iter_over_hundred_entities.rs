use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use smecs::*;

fn bench(c: &mut Criterion) {
    let mut world = black_box(World::new());
    let mut entities = vec![];
    for _ in 0..100 {
        entities.push(world.new_id());
        world.write(entities.last().unwrap().clone(), 1);
    }
    c.bench_function("iter1", |b| {
        b.iter(|| world.each::<i32>(|world, id, comp| comp + 1))
    });
}
criterion_group!(benches, bench);
criterion_main!(benches);
