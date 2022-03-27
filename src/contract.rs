#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::data::{AttackResult, BuildBoard, InstantiateMsg, GeneralAttack};
use crate::state::BOARD1;
use crate::state::BOARD2;


// version info for migration info
const CONTRACT_NAME: &str = "crates.io:my-first-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("result", msg.result.to_string()))
}
//sets the boards with the selected arrangement of ships, splits into set_board1 and set_board2
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    msg: BuildBoard,
) -> Result<Response, ContractError> {
    match msg {
        BuildBoard::TakeBoard1 { board1 } => set_board1(deps, board1),
        BuildBoard::TakeBoard2 { identity, board2 } => set_board2(deps, board2),
    }
}

pub fn set_board1(deps: DepsMut, board: [i32;100]) -> Result<Response, ContractError> {
    BOARD1.save(deps.storage, &board)?;

    Ok(Response::new().add_attribute("method", "try_increment"))
}
pub fn set_board2(deps: DepsMut, board: [i32;100]) -> Result<Response, ContractError> {
    BOARD2.save(deps.storage, &board)?;
    Ok(Response::new().add_attribute("method", "reset"))
}
//chooses which board is being targeted, splits into query board1 and query board2
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: GeneralAttack) -> StdResult<AttackResult> {
    match msg {
        GeneralAttack::GetResultBoard1 {} => query_board1(deps),
        GeneralAttack::GetResultBoard2 {identity: String} => query_board2(deps),
    }
}

fn query_board1(deps: Deps) -> StdResult<AttackResult> {
    let result = BOARD1.load(deps.storage)?;
    Ok(AttackResult { result: result.board1 })
}

fn query_board2(deps: Deps) -> StdResult<AttackResult> {
    let result = BOARD1.load(deps.storage)?;
    Ok(AttackResult { result: result.board1 })
}



