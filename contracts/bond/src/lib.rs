// lib.rs
#![no_std]

extern crate soroban_sdk;

mod owner;
mod contract;

pub use contract::MyContract;
