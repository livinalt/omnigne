#![no_std]
//! Contract scaffold only. There are no swap, deposit or withdrawal entrypoints.
use soroban_sdk::{Env, contract, contractimpl};

#[contract]
pub struct OmnigneStellarExecutor;

#[contractimpl]
impl OmnigneStellarExecutor {
    pub fn scaffold_version(_env: Env) -> u32 {
        1
    }
}
