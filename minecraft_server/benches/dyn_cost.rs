use criterion::{criterion_group, criterion_main, BatchSize::PerIteration, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("dyn cost", |b| {
        b.iter(|| {
            let variable: Box<dyn MyTrait> = Box::new(B { asdf: 1234 });
            variable.do_something();
        });
    });
    c.bench_function("normal", |b| {
        b.iter(|| {
            let variable: Box<B> = Box::new(B { asdf: 1234 });
            variable.do_something();
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

#[derive(Default)]
pub struct A {
    foo: i32,
    foo2: i32,
    foo3: i32,
    foo4: i32,
    foo5: i32,
}

#[derive(Default)]
pub struct B {
    asdf: i32,
}

#[derive(Default)]
pub struct C {
    asdf: i128,
}

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

impl MyTrait for C {
    fn do_something(&self) {
        1 + 1;
    }
}
