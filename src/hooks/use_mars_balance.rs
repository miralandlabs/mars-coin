use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::pubkey::Pubkey;
use solana_extra_wasm::account_decoder::parse_token::UiTokenAmount;

use crate::gateway::{mars_token_account_address, GatewayError, GatewayResult};

use super::{use_gateway, use_pubkey};

pub fn use_mars_balance() -> Resource<GatewayResult<UiTokenAmount>> {
    let gateway = use_gateway();
    let pubkey = use_pubkey();
    let token_account_address = mars_token_account_address(pubkey);
    use_resource(move || {
        let gateway = gateway.clone();
        async move {
            // MI
            // gateway
            //     .rpc
            //     .get_token_account_balance(&token_account_address)
            //     .await
            //     .map_err(GatewayError::from)

            match gateway
                .rpc
                .get_token_account_balance(&token_account_address)
                .await
            {
                Ok(token_account_balance) => {
                    GatewayResult::Ok(token_account_balance)
                }
                Err(err) => {
                    let err = GatewayError::from(err);
                    match err {
                        GatewayError::AccountNotFound => {
                            GatewayResult::Ok(UiTokenAmount {
                                ui_amount: Some(0f64),
                                decimals: mars::TOKEN_DECIMALS,
                                amount: "0.00".to_string(),
                                ui_amount_string: "0.00".to_string(),
                            })
                        }
                        _ => {
                            GatewayResult::Err(err)
                        }
                    }
                }
            }
        }
    })
}

pub fn use_mars_balance_user(pubkey: Pubkey) -> Resource<GatewayResult<UiTokenAmount>> {
    let gateway = use_gateway();
    let token_account_address = mars_token_account_address(pubkey);
    use_resource(move || {
        let gateway = gateway.clone();
        async move {
            // MI
            // gateway
            //     .rpc
            //     .get_token_account_balance(&token_account_address)
            //     .await
            //     .map_err(GatewayError::from)

            match gateway
                .rpc
                .get_token_account_balance(&token_account_address)
                .await
            {
                Ok(token_account_balance) => {
                    GatewayResult::Ok(token_account_balance)
                }
                Err(err) => {
                    let err = GatewayError::from(err);
                    match err {
                        GatewayError::AccountNotFound => {
                            GatewayResult::Ok(UiTokenAmount {
                                ui_amount: Some(0f64),
                                decimals: mars::TOKEN_DECIMALS,
                                amount: "0.00".to_string(),
                                ui_amount_string: "0.00".to_string(),
                            })
                        }
                        _ => {
                            GatewayResult::Err(err)
                        }
                    }
                }
            }
        }
    })
}

pub trait UiTokenAmountBalance {
    fn balance(&self) -> u64;
}

impl UiTokenAmountBalance for UiTokenAmount {
    fn balance(&self) -> u64 {
        self.amount.parse().unwrap_or(0)
    }
}
