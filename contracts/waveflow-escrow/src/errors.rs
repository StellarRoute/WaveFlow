// Contract error codes returned to callers and emitted in failed invocations.
use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ContractError {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    Unauthorized = 3,
    InvalidRepo = 4,
    ProgramNotFound = 5,
    ContributorNotFound = 6,
    ContributorAlreadyRegistered = 7,
    InsufficientEscrow = 8,
    ProgramPaused = 9,
    PrAlreadyProcessed = 10,
    MilestoneCapExceeded = 11,
    InsufficientDeposit = 12,
    InvalidRatio = 13,
    Overflow = 14,
    ProgramNotPaused = 15,
}
