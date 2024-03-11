use minecraft_server::io::fast_map::FastMap;

use criterion::{criterion_group, criterion_main, Criterion};
use num_derive::FromPrimitive;

fn criterion_benchmark(c: &mut Criterion) {
    let something = MyEnum::E;
    c.bench_function("match_branching", |b| {
        b.iter(|| match something {
            MyEnum::A { foo } => 1 + 1,
            MyEnum::B(_) => 1 + 1,
            MyEnum::C(_) => 1 + 1,
            MyEnum::D => 1 + 1,
            MyEnum::E => 1 + 1,
        });
    });
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

enum MyEnum {
    A { foo: i32 },
    B(f32),
    C(String),
    D,
    E,
}
pub trait MyTrait {
    fn do_something(&self);
}

pub struct C {
    inner: Box<dyn MyTrait>,
}

pub struct A {}

pub struct B {}

impl MyTrait for A {
    fn do_something(&self) {
        1 + 1;
    }
}

impl MyTrait for B {
    fn do_something(&self) {
        1 + 1;
    }
}
