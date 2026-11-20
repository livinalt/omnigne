#![no_std]
//! Contract scaffold only. Intent authorization and settlement are not implemented.
use soroban_sdk::{Env, contract, contractimpl};

#[contract]
pub struct OmnigneStellarSettler;

#[contractimpl]
impl OmnigneStellarSettler {
    pub fn scaffold_version(_env: Env) -> u32 {
        1
    }
}
