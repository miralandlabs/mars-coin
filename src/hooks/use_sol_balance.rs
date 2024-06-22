use dioxus::prelude::*;

use crate::gateway::{GatewayError, GatewayResult};

use super::{use_gateway, use_pubkey};

pub fn use_sol_balance() -> Resource<GatewayResult<u64>> {
    let address = use_pubkey();
    let gateway = use_gateway();
    use_resource(move || {
        let gateway = gateway.clone();
        async move {
            // MI
            // gateway
            //     .rpc
            //     .get_balance(&address)
            //     .await
            //     .map_err(GatewayError::from)

            match gateway
                .rpc
                .get_balance(&address)
                .await
            {
                Ok(sol_balance) => {
                    GatewayResult::Ok(sol_balance)
                }
                Err(err) => {
                    let err = GatewayError::from(err);
                    match err {
                        GatewayError::AccountNotFound => {
                            GatewayResult::Ok(0u64)
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
