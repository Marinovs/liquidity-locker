use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")] Std(#[from] StdError),
    #[error("Unauthorized")] Unauthorized {},
    #[error("Invalid CW721 Receive Message")] InvalidCw721Msg {},
    #[error("InvalidCw20Token")] InvalidCw20Token {},
    #[error("InvalidCw721Token")] InvalidCw721Token {},
    #[error("Locktime. Send Unstake Fee")] Locktime {},
    #[error("Locked Period Wrong")] LockedPeriodWrong {},
    #[error("Already Locked")] AlreadyLocked {},
    #[error("LP Not found")] NoLPFound {},
    #[error("Payment Failed")] PaymentFailed {},
    #[error("Fee Payment Failed")] FeePaymentFailed {},
    #[error("Disabled")] Disabled {},
    #[error("Missmatched Payment")] MissmatchedPayment {},
}
