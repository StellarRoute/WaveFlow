// Shared crate root: re-exports types, errors, and config for gateway and API.
pub mod config;
pub mod error;
pub mod types;

pub use config::AppConfig;
pub use error::{WaveFlowError, WaveFlowResult};
pub use types::{
    AttestationRequest, ContributorRecord, GitHubPullRequestEvent, PayoutRecord, ProgramRecord,
    ProgramStatus, validate_stellar_address,
};
