#![allow(incomplete_features)]
#![feature(return_position_impl_trait_in_trait)]
#![feature(iter_intersperse)]
#![feature(structural_match)]
#[allow(incomplete_features)]
mod resource_idents;
pub use resource_idents::*;

mod resource_provider;
pub use resource_provider::*;

// mod resource_manager;
// pub use resource_manager::*;

#[cfg(test)]
mod test;
