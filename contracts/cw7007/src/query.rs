use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::Serialize;

use cosmwasm_std::{to_binary, Binary, CustomMsg, Deps, Env, StdResult};

use crate::msg::{PromptInfoResponse, QueryMsg};
use crate::state::Cw7007Contract;
use crate::traits::Cw7007Query;
use cw721_base::QueryMsg as Cw721QueryMsg;

impl<'a, T, C, E, Q> Cw7007Query<T> for Cw7007Contract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
{
    fn prompt_info(&self, deps: Deps) -> StdResult<PromptInfoResponse> {
        self.prompt_info.load(deps.storage)
    }

    fn request_ids(&self, deps: Deps) -> StdResult<crate::msg::RequestIdsResponse> {
        self.request_ids.load(deps.storage)
    }
}

impl<'a, T, C, E, Q> Cw7007Contract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
{
    pub fn query(&self, deps: Deps, env: Env, msg: QueryMsg<Q>) -> StdResult<Binary> {
        match msg {
            QueryMsg::Prompt {} => to_binary(&self.prompt_info(deps)?),
            QueryMsg::RequestIds {} => to_binary(&self.request_ids(deps)?),
            _ => self.cw721.query(deps, env, msg.into()),
        }
    }
}

impl<Q: JsonSchema> From<QueryMsg<Q>> for Cw721QueryMsg<Q> {
    fn from(item: QueryMsg<Q>) -> Self {
        match item {
            QueryMsg::OwnerOf {
                token_id,
                include_expired,
            } => Cw721QueryMsg::OwnerOf {
                token_id,
                include_expired,
            },
            QueryMsg::Approval {
                token_id,
                spender,
                include_expired,
            } => Cw721QueryMsg::Approval {
                token_id,
                spender,
                include_expired,
            },
            QueryMsg::Approvals {
                token_id,
                include_expired,
            } => Cw721QueryMsg::Approvals {
                token_id,
                include_expired,
            },
            QueryMsg::Operator {
                owner,
                operator,
                include_expired,
            } => Cw721QueryMsg::Operator {
                owner,
                operator,
                include_expired,
            },
            QueryMsg::AllOperators {
                owner,
                include_expired,
                start_after,
                limit,
            } => Cw721QueryMsg::AllOperators {
                owner,
                include_expired,
                start_after,
                limit,
            },
            QueryMsg::NumTokens {} => Cw721QueryMsg::NumTokens {},
            QueryMsg::ContractInfo {} => Cw721QueryMsg::ContractInfo {},
            QueryMsg::NftInfo { token_id } => Cw721QueryMsg::NftInfo { token_id },
            QueryMsg::AllNftInfo {
                token_id,
                include_expired,
            } => Cw721QueryMsg::AllNftInfo {
                token_id,
                include_expired,
            },
            QueryMsg::Tokens {
                owner,
                start_after,
                limit,
            } => Cw721QueryMsg::Tokens {
                owner,
                start_after,
                limit,
            },
            QueryMsg::AllTokens { start_after, limit } => {
                Cw721QueryMsg::AllTokens { start_after, limit }
            }
            QueryMsg::Minter {} => Cw721QueryMsg::Minter {},
            QueryMsg::Extension { msg } => Cw721QueryMsg::Extension { msg },
            _ => panic!("Unsupported query message"), // This should not happen if handled correctly
        }
    }
}
