use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum JobStatus {
    Open,
    InProgress,
    Completed,
    Cancelled,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Job {
    pub poster: Addr,
    pub title: String,
    pub description: String,
    pub budget: Uint128,
    pub status: JobStatus,
    pub assigned_freelancer: Option<Addr>,
    pub created_at: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Proposal {
    pub freelancer: Addr,
    pub bid_amount: Uint128,
    pub cover_letter: String,
}

// Storage keys
pub const JOBS: Map<u64, Job> = Map::new("jobs");
pub const JOB_PROPOSALS: Map<u64, Vec<Proposal>> = Map::new("job_proposals");
pub const JOB_COUNTER: Item<u64> = Item::new("job_counter");
