#![feature(negative_impls)]
#![feature(auto_traits)]

#![allow(warnings)]

pub mod io;
pub mod net;

#[macro_export]
#[macro_use]
pub mod protocol;
pub mod server;

#[cfg(test)]
mod test;

