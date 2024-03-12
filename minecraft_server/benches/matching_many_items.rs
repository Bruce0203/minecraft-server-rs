use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("matching many items", |b| {
        b.iter(|| {
            for i in 0..100 {
                match_test(E::z);
                match_test(E::y);
                match_test(E::y);
                match_test(E::y);
                match_test(E::y);
                match_test(E::y);
                match_test(E::y);
            }
        });
    });
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

#[no_mangle]
fn match_test(input: E) -> i32 {
    let mut i = 1000;
    match input {
        E::a => i + 1,
        E::b => i + 2,
        E::c => i + 3,
        E::d => i + 4,
        E::e => i + 5,
        E::f => i + 6,
        E::g => i + 7,
        E::h => i + 8,
        E::i => i + 9,
        E::j => i + 10,
        E::k => i + 11,
        E::l => i + 12,
        E::m => i + 13,
        E::n => i + 14,
        E::o => i + 15,
        E::p => i + 16,
        E::q => i + 17,
        E::r => i + 18,
        E::s => i + 19,
        E::t => i + 20,
        E::u => i + 21,
        E::v => i + 22,
        E::w => i + 23,
        E::x => i + 24,
        E::y => i + 25,
        E::z => i + 26,
    }
}

enum E {
    a,
    b,
    c,
    d,
    e,
    f,
    g,
    h,
    i,
    j,
    k,
    l,
    m,
    n,
    o,
    p,
    q,
    r,
    s,
    t,
    u,
    v,
    w,
    x,
    y,
    z,
}
