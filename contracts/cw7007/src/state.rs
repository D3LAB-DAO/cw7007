use cosmwasm_schema::cw_serde;
use cosmwasm_std::CustomMsg;
use cw_storage_plus::Item;

use serde::de::DeserializeOwned;
use serde::Serialize;

use cw721_base::Cw721Contract;

use crate::msg::{PromptInfoResponse, RequestIdsResponse};
// use crate::traits::Cw7007;

#[cw_serde]
pub struct Trait {
    pub display_type: Option<String>,
    pub trait_type: String,
    pub value: String,
}

// see: https://docs.opensea.io/docs/metadata-standards
#[cw_serde]
#[derive(Default)]
pub struct Metadata {
    pub image: Option<String>, // response in svg
    pub image_data: Option<String>,
    pub external_url: Option<String>,
    pub description: Option<String>, // query from user
    pub name: Option<String>,        // name
    pub attributes: Option<Vec<Trait>>,
    pub background_color: Option<String>,
    pub animation_url: Option<String>,
    pub youtube_url: Option<String>,
}

pub type Extension = Option<Metadata>;

pub struct Cw7007Contract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    Q: CustomMsg,
    E: CustomMsg,
{
    pub cw721: cw721_base::Cw721Contract<'a, T, C, E, Q>,

    pub prompt_info: Item<'a, PromptInfoResponse>,
    pub request_ids: Item<'a, RequestIdsResponse>,
}

impl<T, C, E, Q> Default for Cw7007Contract<'static, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    E: CustomMsg,
    Q: CustomMsg,
{
    fn default() -> Self {
        Self::new("prompt", "requestids")
    }
}

impl<'a, T, C, E, Q> Cw7007Contract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    E: CustomMsg,
    Q: CustomMsg,
{
    fn new(prompt_info_key: &'a str, request_ids_key: &'a str) -> Self {
        Self {
            prompt_info: Item::new(prompt_info_key),
            request_ids: Item::new(request_ids_key),
            cw721: Cw721Contract::default(),
        }
    }
}
