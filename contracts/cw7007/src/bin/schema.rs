use cosmwasm_schema::write_api;

use cosmwasm_std::Empty;
use cw7007::msg::InstantiateMsg;
use cw721_base::Extension;

pub type ExecuteMsg = cw7007::msg::ExecuteMsg<Extension, Empty>;
pub type QueryMsg = cw7007::msg::QueryMsg<Empty>;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}
