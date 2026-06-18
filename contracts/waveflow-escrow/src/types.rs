// Domain structs serialized in Soroban persistent storage.
use soroban_sdk::{contracttype, Address, Symbol};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProgramStatus {
    Active,
    Paused,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Program {
    pub maintainer: Address,
    pub github_repo: Symbol,
    pub reward_per_point: i128,
    pub escrow_balance: i128,
    pub milestone_cap: Option<i128>,
    pub milestone_spent: i128,
    pub status: ProgramStatus,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Contributor {
    pub stellar_address: Address,
}
