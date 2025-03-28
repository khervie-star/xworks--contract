use cosmwasm_std::{Addr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admin: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    PostJob {
        title: String,
        description: String,
        budget: Uint128,
    },
    SubmitProposal {
        job_id: u64,
        bid_amount: Uint128,
        cover_letter: String,
    },
    AcceptProposal {
        job_id: u64,
        freelancer: String,
    },
    CompleteJob {
        job_id: u64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
    GetJobDetails { job_id: u64 },
    GetJobProposals { job_id: u64 },
}
