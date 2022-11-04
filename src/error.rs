use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Too many poll options")]
    TooManyOptions {},

    #[error("The Poll does not exist")]
    PollNotFound {},

    #[error("The option does not exist")]
    OptionNonExistent {},
}
