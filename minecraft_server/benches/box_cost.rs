use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("box cost", |b| {
        b.iter(|| consume_value(A{}));
    });
}

#[inline(never)]
#[no_mangle]
fn consume_value<T>(value: T) {
    let _ = &value;
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
