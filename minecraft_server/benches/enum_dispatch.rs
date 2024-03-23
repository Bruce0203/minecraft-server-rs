use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use enum_dispatch::enum_dispatch;
use std::{collections::VecDeque, io::Cursor};

#[enum_dispatch(Packets)]
pub trait EncodePacket {
    fn do_it(&self);
}

#[enum_dispatch]
#[repr(i32)]
pub enum Packets {
    Handshake,
}

pub struct Handshake {}

impl Handshake {
    fn new() -> Handshake {
        Handshake {}
    }
}
impl EncodePacket for Handshake {
    fn do_it(&self) {
        println!("handshake");
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let batch_size = BatchSize::LargeInput;
    let vec = vec![1; 1000];
    c.bench_function("push vec", |b| {
        b.iter(|| {
            let packet: Packets = Handshake::new().into();
            packet.do_it();
        });
    });
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
