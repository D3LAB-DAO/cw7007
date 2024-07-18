mod execute;
pub mod msg;
mod query;
mod state;
mod traits;

use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};
use cw721_base::ContractError;
use msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use state::Cw7007Contract;
use state::Extension;

// Version info for migration
const CONTRACT_NAME: &str = "crates.io:cw7007";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(not(feature = "library"))]
pub mod entry {
    use super::*;

    // This makes a conscious choice on the various generics used by the contract
    #[entry_point]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
        cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        let contract = Cw7007Contract::<Extension, Empty, Empty, Empty>::default();
        contract.instantiate(deps, env, info, msg)
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg<Extension, Empty>,
    ) -> Result<Response, ContractError> {
        let should_check_extension = matches!(msg, ExecuteMsg::Mint { .. });
        let extension = if let ExecuteMsg::Mint { extension, .. } = &msg {
            extension.clone()
        } else {
            None
        };
        if should_check_extension {
            if let Some(ext) = &extension {
                println!("Checking extension: {:?}", ext);

                if ext.description.is_none() {
                    return Err(ContractError::Std(cosmwasm_std::StdError::generic_err(
                        "Description is required in extension.",
                    )));
                }
            }
        }

        let contract = Cw7007Contract::<Extension, Empty, Empty, Empty>::default();
        contract.execute(deps, env, info, msg)
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg<Empty>) -> StdResult<Binary> {
        let contract = Cw7007Contract::<Extension, Empty, Empty, Empty>::default();
        contract.query(deps, env, msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use cosmwasm_std::{
        from_binary,
        testing::{mock_dependencies, mock_env, mock_info},
    };
    use cw721::NftInfoResponse;
    use msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
    use state::Metadata;

    const CREATOR: &str = "creator";

    /// Make sure cw2 version info is properly initialized during instantiation,
    /// and NOT overwritten by the base contract.
    #[test]
    fn proper_cw2_initialization() {
        let mut deps = mock_dependencies();

        entry::instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info("larry", &[]),
            InstantiateMsg {
                name: "".into(),
                symbol: "".into(),
                minter: "larry".into(),
                prompt: "You are a cat. Just answer with 'MEOW'.".into(),
            },
        )
        .unwrap();

        let version = cw2::get_contract_version(deps.as_ref().storage).unwrap();
        assert_eq!(version.contract, CONTRACT_NAME);
        assert_ne!(version.contract, cw721_base::CONTRACT_NAME);
    }

    #[test]
    fn use_metadata_extension() {
        let mut deps = mock_dependencies();
        let contract = Cw7007Contract::<Extension, Empty, Empty, Empty>::default();

        let info = mock_info(CREATOR, &[]);
        let init_msg = InstantiateMsg {
            name: "SpaceShips".to_string(),
            symbol: "SPACE".to_string(),
            minter: CREATOR.to_string(),
            prompt: "You are a cat. Just answer with 'MEOW'.".into(),
        };
        contract
            .instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg)
            .unwrap();

        let token_uri = Some("https://starships.example.com/Starship/Enterprise.json".into());
        let extension = Some(Metadata {
            description: Some("Spaceship with Warp Drive".into()),
            name: Some("Starship USS Enterprise".to_string()),
            ..Metadata::default()
        });
        let exec_msg = ExecuteMsg::Mint {
            token_id: "Not used".to_string(),
            owner: "john".to_string(),
            token_uri: token_uri.clone(),
            extension: extension.clone(),
        };
        contract
            .execute(deps.as_mut(), mock_env(), info, exec_msg)
            .unwrap();

        let query_msg: QueryMsg<Empty> = QueryMsg::NftInfo {
            token_id: "0".to_string(),
        };
        let res = contract.query(deps.as_ref(), mock_env(), query_msg);
        match res {
            Ok(binary_res) => {
                let res: NftInfoResponse<Metadata> =
                    from_binary(&binary_res).expect("Failed to parse binary response");
                assert_eq!(res.token_uri, token_uri, "Token URI does not match");
                assert_eq!(
                    res.extension,
                    extension.expect("Extension is None"),
                    "Extension does not match"
                );
            }
            Err(err) => panic!("Query failed: {:?}", err),
        };
    }
}
