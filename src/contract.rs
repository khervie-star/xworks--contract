use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Job, JobStatus, Proposal, JOBS, JOB_COUNTER, JOB_PROPOSALS};
use cosmwasm_std::{
    entry_point, to_json_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult, Uint128,
};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    // Initialize job counter
    JOB_COUNTER.save(deps.storage, &0)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("admin", info.sender.to_string()))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::PostJob {
            title,
            description,
            budget,
        } => execute_post_job(deps, env, info, title, description, budget),
        ExecuteMsg::SubmitProposal {
            job_id,
            bid_amount,
            cover_letter,
        } => execute_submit_proposal(deps, env, info, job_id, bid_amount, cover_letter),
        ExecuteMsg::AcceptProposal { job_id, freelancer } => {
            let freelancer_addr = deps.api.addr_validate(&freelancer)?;
            execute_accept_proposal(deps, env, info, job_id, freelancer_addr)
        }
        ExecuteMsg::CompleteJob { job_id } => execute_complete_job(deps, env, info, job_id),
    }
}

fn execute_post_job(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    title: String,
    description: String,
    budget: Uint128,
) -> Result<Response, ContractError> {
    // Validate inputs
    if title.is_empty() {
        return Err(ContractError::InvalidInput {
            error: "Title cannot be empty".to_string(),
        });
    }

    // Get and increment job counter
    let job_id = JOB_COUNTER.load(deps.storage)?;
    JOB_COUNTER.save(deps.storage, &(job_id + 1))?;

    // Create and save job
    let job = Job {
        poster: info.sender.clone(),
        title,
        description,
        budget,
        status: JobStatus::Open,
        assigned_freelancer: None,
        created_at: env.block.time.seconds(),
    };

    JOBS.save(deps.storage, job_id, &job)?;

    Ok(Response::new()
        .add_attribute("method", "post_job")
        .add_attribute("job_id", job_id.to_string())
        .add_attribute("poster", info.sender.to_string()))
}

fn execute_submit_proposal(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    job_id: u64,
    bid_amount: Uint128,
    cover_letter: String,
) -> Result<Response, ContractError> {
    // Validate inputs
    if cover_letter.is_empty() {
        return Err(ContractError::InvalidInput {
            error: "Cover letter cannot be empty".to_string(),
        });
    }

    // Load job to ensure it exists
    let job = JOBS.load(deps.storage, job_id)?;

    // Ensure the job is open
    if job.status != JobStatus::Open {
        return Err(ContractError::InvalidInput {
            error: "Job is not open for proposals".to_string(),
        });
    }

    // Save the proposal
    let mut proposals = JOB_PROPOSALS
        .may_load(deps.storage, job_id)?
        .unwrap_or_default();
    let proposal = Proposal {
        freelancer: info.sender.clone(),
        bid_amount,
        cover_letter,
    };
    proposals.push(proposal);
    JOB_PROPOSALS.save(deps.storage, job_id, &proposals)?;

    Ok(Response::new()
        .add_attribute("method", "submit_proposal")
        .add_attribute("job_id", job_id.to_string())
        .add_attribute("freelancer", info.sender.to_string()))
}

fn execute_accept_proposal(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    job_id: u64,
    freelancer: Addr,
) -> Result<Response, ContractError> {
    // Load the job
    let mut job = JOBS.load(deps.storage, job_id)?;

    // Ensure the sender is the job poster
    if job.poster != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    // Ensure the job is open
    if job.status != JobStatus::Open {
        return Err(ContractError::InvalidInput {
            error: "Job is not open".to_string(),
        });
    }

    // Assign the freelancer and update the job status
    job.assigned_freelancer = Some(freelancer.clone());
    job.status = JobStatus::InProgress;
    JOBS.save(deps.storage, job_id, &job)?;

    Ok(Response::new()
        .add_attribute("method", "accept_proposal")
        .add_attribute("job_id", job_id.to_string())
        .add_attribute("freelancer", freelancer.to_string()))
}

fn execute_complete_job(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    job_id: u64,
) -> Result<Response, ContractError> {
    // Load the job
    let mut job = JOBS.load(deps.storage, job_id)?;

    // Ensure the sender is the assigned freelancer
    if job.assigned_freelancer != Some(info.sender.clone()) {
        return Err(ContractError::Unauthorized {});
    }

    // Ensure the job is in progress
    if job.status != JobStatus::InProgress {
        return Err(ContractError::InvalidInput {
            error: "Job is not in progress".to_string(),
        });
    }

    // Mark the job as completed
    job.status = JobStatus::Completed;
    JOBS.save(deps.storage, job_id, &job)?;

    Ok(Response::new()
        .add_attribute("method", "complete_job")
        .add_attribute("job_id", job_id.to_string())
        .add_attribute("freelancer", info.sender.to_string()))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetJobDetails { job_id } => {
            let job = JOBS.load(deps.storage, job_id)?;
            to_json_binary(&job)
        }
        QueryMsg::GetJobProposals { job_id } => {
            let proposals = JOB_PROPOSALS
                .may_load(deps.storage, job_id)?
                .unwrap_or_default();
            to_json_binary(&proposals)
        }
    }
}
