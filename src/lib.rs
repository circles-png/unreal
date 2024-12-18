//! # unreal
//!
//! Fake data generator. Fork of the [fakeit](https://github.com/PumpkinSeed/fakeit) crate.

#![warn(
    clippy::nursery,
    clippy::pedantic,
    clippy::unwrap_used,
    clippy::allow_attributes_without_reason,
    missing_docs
)]

mod data;
/// Use a global instance of the Unreal struct to generate fake data.
pub mod global;
mod impls;
mod macros;
mod unreal;
pub(crate) use macros::{array_consts, choose};
pub use unreal::Unreal;
