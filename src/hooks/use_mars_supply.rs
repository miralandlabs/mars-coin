use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;
use solana_extra_wasm::account_decoder::parse_token::UiTokenAmount;
use std::str::FromStr;

use crate::gateway::{GatewayError, GatewayResult};

use super::use_gateway;

pub fn use_mars_supply() -> Resource<GatewayResult<UiTokenAmount>> {
    let gateway = use_gateway();
    use_resource(move || {
        let gateway = gateway.clone();
        async move {
            gateway
                .rpc
                .get_token_supply(&mars::MINT_ADDRESS)
                .await
                .map_err(GatewayError::from)
        }
    })
}

// MI
// use treasury mars token account address directly
pub fn use_treasury_mars_balance() -> Resource<GatewayResult<UiTokenAmount>> {
    let gateway = use_gateway();
    use_resource(move || {
        let gateway = gateway.clone();
        async move {
            gateway
                .rpc
                .get_token_account_balance(&Pubkey::from_str("Ezd7bvR36uDJ7RSKrDXe1AJPrAK6zcXY6SMnUF29c5EK").unwrap())
                .await
                .map_err(GatewayError::from)
        }
    })
}
