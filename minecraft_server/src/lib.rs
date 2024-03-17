#![feature(associated_const_equality)]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![feature(const_mut_refs)]
#![feature(effects)]
#![feature(const_trait_impl)]
#![feature(generic_arg_infer)]
#![feature(inline_const)]
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

