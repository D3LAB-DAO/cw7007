use serde::de::DeserializeOwned;
use serde::Serialize;

use cosmwasm_std::{CustomMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::msg::{PromptInfoResponse, RequestIdsResponse};

pub trait Cw7007<T, C>: Cw7007Execute<T, C> + Cw7007Query<T>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
{
}

pub trait Cw7007Execute<T, C>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
{
    type Err: ToString;

    fn response(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id: String,
        output: String,
    ) -> Result<Response<C>, Self::Err>;

    fn verify(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_id: String,
        proof: String,
    ) -> Result<Response<C>, Self::Err>;
}

pub trait Cw7007Query<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn prompt_info(&self, deps: Deps) -> StdResult<PromptInfoResponse>;
    fn request_ids(&self, deps: Deps) -> StdResult<RequestIdsResponse>;
}
