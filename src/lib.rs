#![feature(trait_alias, iter_array_chunks)]
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    missing_debug_implementations
)]
#![allow(unused, clippy::cast_sign_loss, clippy::cast_possible_wrap)]
pub mod year_2015;
pub mod year_2016;

mod shared;
