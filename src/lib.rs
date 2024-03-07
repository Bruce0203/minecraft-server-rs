#![allow(warnings)]
#![feature(associated_type_defaults)]
#![feature(try_blocks)]
#![feature(trait_alias)]

pub mod io;
pub mod net;
pub mod protocol;
pub mod server;

#[cfg(test)]
pub mod test;
