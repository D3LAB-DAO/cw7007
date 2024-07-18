use cw721_base::state::TokenInfo;
use serde::de::DeserializeOwned;
use serde::Serialize;

use cosmwasm_std::{CustomMsg, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::msg::{ExecuteMsg, InstantiateMsg, PromptInfoResponse, RequestIdsResponse};
use crate::state::{Cw7007Contract, Extension};
use crate::traits::Cw7007Execute;
use cw721_base::{
    ContractError, ExecuteMsg as Cw721ExecuteMsg, InstantiateMsg as Cw721InstantiateMsg,
};

impl<'a, T, C, E, Q> Cw7007Contract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
{
    pub fn instantiate(
        &self,
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response<C>> {
        self.cw721.instantiate(
            DepsMut {
                storage: deps.storage,
                api: deps.api,
                querier: deps.querier,
            },
            _env.clone(),
            _info.clone(),
            Cw721InstantiateMsg {
                name: msg.name.clone(),
                symbol: msg.symbol.clone(),
                minter: msg.minter.clone(),
            },
        )?;

        let prompt_info_data = PromptInfoResponse { prompt: msg.prompt };
        let request_ids_data = RequestIdsResponse { ids: Vec::new() };
        self.prompt_info.save(deps.storage, &prompt_info_data)?;
        self.request_ids.save(deps.storage, &request_ids_data)?;

        Ok(Response::default())
    }
}

impl<'a, C, E, Q> Cw7007Contract<'a, Extension, C, E, Q>
where
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
{
    pub fn execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg<Extension, E>,
    ) -> Result<Response<C>, ContractError> {
        match msg {
            ExecuteMsg::Mint {
                token_id: _,
                owner,
                token_uri,
                extension,
            } => self.mint_anyone(deps, info, owner, token_uri, extension),
            ExecuteMsg::Response { token_id, output } => {
                self.response(deps, env, info, token_id, output)
            }
            _ => self.cw721.execute(deps, env, info, msg.into()),
        }
    }
}

impl<'a, C, E, Q> Cw7007Execute<Extension, C> for Cw7007Contract<'a, Extension, C, E, Q>
where
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
{
    type Err = ContractError;

    fn response(
        &self,
        deps: cosmwasm_std::DepsMut,
        env: Env,
        info: cosmwasm_std::MessageInfo,
        token_id: String,
        output: String,
    ) -> Result<Response<C>, Self::Err> {
        cw_ownable::assert_owner(deps.storage, &info.sender)?;

        let mut token: TokenInfo<Extension> = self.cw721.tokens.load(deps.storage, &token_id)?;
        if let Some(mut extension) = token.extension {
            if extension.image.is_some() {
                return Err(ContractError::Std(cosmwasm_std::StdError::generic_err(
                    "image field is already filled.",
                )));
            }
            extension.image = Some(output.clone());
            token.extension = Some(extension);
        } else {
            return Err(ContractError::Std(cosmwasm_std::StdError::generic_err(
                "image field is required in extension.",
            )));
        }
        self.cw721.tokens.save(deps.storage, &token_id, &token)?;

        // request update
        let request_ids = self
            .request_ids
            .load(deps.storage)
            .unwrap_or(RequestIdsResponse { ids: Vec::new() });
        let new_ids: Vec<String> = request_ids
            .ids
            .into_iter()
            .filter(|x| x != &token_id)
            .collect();
        self.request_ids
            .save(deps.storage, &RequestIdsResponse { ids: new_ids })?;

        Ok(Response::new()
            .add_attribute("action", "response")
            .add_attribute("sender", info.sender)
            .add_attribute("token_id", token_id)
            .add_attribute("output", output))
    }

    fn verify(
        &self,
        deps: cosmwasm_std::DepsMut,
        env: Env,
        info: cosmwasm_std::MessageInfo,
        token_id: String,
        proof: String,
    ) -> Result<Response<C>, Self::Err> {
        todo!() // TODO
    }
}

impl<'a, T, C, E, Q> Cw7007Contract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
{
    pub fn mint_anyone(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        // token_id: String,
        owner: String,
        token_uri: Option<String>,
        extension: T,
    ) -> Result<Response<C>, ContractError> {
        // cw_ownable::assert_owner(deps.storage, &info.sender)?;

        // create the token
        let token = TokenInfo {
            owner: deps.api.addr_validate(&owner)?,
            approvals: vec![],
            token_uri,
            extension,
        };
        let token_id = self.cw721.token_count(deps.storage)?.to_string(); // counter

        self.cw721
            .tokens
            .update(deps.storage, &token_id, |old| match old {
                Some(_) => Err(ContractError::Claimed {}),
                None => Ok(token),
            })?;

        self.cw721.increment_tokens(deps.storage)?;

        // request update
        let request_ids = self.request_ids.load(deps.storage).unwrap();
        // .unwrap_or(RequestIdsResponse { ids: Vec::new() });
        self.request_ids.save(
            deps.storage,
            &RequestIdsResponse {
                ids: {
                    let mut ids = request_ids.ids;
                    ids.push(token_id.clone());
                    ids
                },
            },
        )?;

        Ok(Response::new()
            .add_attribute("action", "mint")
            .add_attribute("minter", info.sender)
            .add_attribute("owner", owner)
            .add_attribute("token_id", token_id))
    }
}

impl<T, E> From<ExecuteMsg<T, E>> for Cw721ExecuteMsg<T, E> {
    fn from(item: ExecuteMsg<T, E>) -> Self {
        match item {
            ExecuteMsg::Mint {
                token_id,
                owner,
                token_uri,
                extension,
            } => Cw721ExecuteMsg::Mint {
                token_id,
                owner,
                token_uri,
                extension,
            },
            ExecuteMsg::Approve {
                spender,
                token_id,
                expires,
            } => Cw721ExecuteMsg::Approve {
                spender,
                token_id,
                expires,
            },
            ExecuteMsg::Revoke { spender, token_id } => {
                Cw721ExecuteMsg::Revoke { spender, token_id }
            }
            ExecuteMsg::ApproveAll { operator, expires } => {
                Cw721ExecuteMsg::ApproveAll { operator, expires }
            }
            ExecuteMsg::RevokeAll { operator } => Cw721ExecuteMsg::RevokeAll { operator },
            ExecuteMsg::TransferNft {
                recipient,
                token_id,
            } => Cw721ExecuteMsg::TransferNft {
                recipient,
                token_id,
            },
            ExecuteMsg::SendNft {
                contract,
                token_id,
                msg,
            } => Cw721ExecuteMsg::SendNft {
                contract,
                token_id,
                msg,
            },
            ExecuteMsg::Burn { token_id } => Cw721ExecuteMsg::Burn { token_id },
            ExecuteMsg::UpdateOwnership(action) => Cw721ExecuteMsg::UpdateOwnership(action),
            ExecuteMsg::Extension { msg } => Cw721ExecuteMsg::Extension { msg },
            _ => panic!("Unsupported execute message"), // This should not happen if handled correctly
        }
    }
}
