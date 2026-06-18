// Soroban storage keys and typed accessors for WaveFlow escrow state.
use crate::errors::ContractError;
use crate::types::{Contributor, Program, ProgramStatus};
use soroban_sdk::{contracttype, Address, Env, Symbol};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Gateway,
    Token,
    ProgramCounter,
    Program(u64),
    Contributor(u64, Symbol),
    ProcessedPr(u64, u64),
}

pub fn read_admin(env: &Env) -> Result<Address, ContractError> {
    env.storage()
        .instance()
        .get(&DataKey::Admin)
        .ok_or(ContractError::NotInitialized)
}

pub fn read_gateway(env: &Env) -> Result<Address, ContractError> {
    env.storage()
        .instance()
        .get(&DataKey::Gateway)
        .ok_or(ContractError::NotInitialized)
}

pub fn read_token(env: &Env) -> Result<Address, ContractError> {
    env.storage()
        .instance()
        .get(&DataKey::Token)
        .ok_or(ContractError::NotInitialized)
}

pub fn read_program(env: &Env, program_id: u64) -> Result<Program, ContractError> {
    env.storage()
        .persistent()
        .get(&DataKey::Program(program_id))
        .ok_or(ContractError::ProgramNotFound)
}

pub fn write_program(env: &Env, program_id: u64, program: &Program) {
    env.storage()
        .persistent()
        .set(&DataKey::Program(program_id), program);
}

pub fn next_program_id(env: &Env) -> u64 {
    let current: u64 = env
        .storage()
        .instance()
        .get(&DataKey::ProgramCounter)
        .unwrap_or(0);
    let next = current + 1;
    env.storage()
        .instance()
        .set(&DataKey::ProgramCounter, &next);
    next
}

pub fn is_pr_processed(env: &Env, program_id: u64, pr_number: u64) -> bool {
    env.storage()
        .persistent()
        .get(&DataKey::ProcessedPr(program_id, pr_number))
        .unwrap_or(false)
}

pub fn mark_pr_processed(env: &Env, program_id: u64, pr_number: u64) {
    env.storage()
        .persistent()
        .set(&DataKey::ProcessedPr(program_id, pr_number), &true);
}

pub fn read_contributor(
    env: &Env,
    program_id: u64,
    github_username: &Symbol,
) -> Result<Contributor, ContractError> {
    env.storage()
        .persistent()
        .get(&DataKey::Contributor(program_id, github_username.clone()))
        .ok_or(ContractError::ContributorNotFound)
}

pub fn write_contributor(
    env: &Env,
    program_id: u64,
    github_username: &Symbol,
    contributor: &Contributor,
) {
    env.storage().persistent().set(
        &DataKey::Contributor(program_id, github_username.clone()),
        contributor,
    );
}

pub fn validate_repo(repo: &Symbol) -> Result<(), ContractError> {
    let s = repo.to_string();
    let bytes = s.as_bytes();
    let slash_count = bytes.iter().filter(|&&b| b == b'/').count();
    if slash_count != 1 || bytes.is_empty() || bytes[0] == b'/' || bytes[bytes.len() - 1] == b'/' {
        return Err(ContractError::InvalidRepo);
    }
    Ok(())
}

pub fn require_active(program: &Program) -> Result<(), ContractError> {
    if program.status == ProgramStatus::Paused {
        return Err(ContractError::ProgramPaused);
    }
    Ok(())
}
