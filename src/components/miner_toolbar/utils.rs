use std::rc::Rc;

use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::signer::Signer;

use crate::{
    gateway::{signer, Gateway, GatewayResult},
    hooks::{MinerStatusMessage, MinerToolbarState, UpdateMinerToolbarState},
    miner::Miner,
};

// TODO Move this somewhere

pub async fn try_start_mining(
    gateway: Rc<Gateway>,
    miner: Signal<Miner>,
    toolbar_state: &mut Signal<MinerToolbarState>,
) -> GatewayResult<()> {
    // Create proof account, if needed

    toolbar_state.set_status_message(MinerStatusMessage::GeneratingChallenge);
    loop {
        if gateway.register_mars().await.is_ok() {
            break;
        }
    }

    // Start mining
    let signer = signer();
    if let Ok(treasury) = gateway.get_treasury().await {
        if let Ok(proof) = gateway.get_proof(signer.pubkey()).await {
            toolbar_state.set_status_message(MinerStatusMessage::Searching);
            miner
                .read()
                .start_mining(
                    proof.hash.into(),
                    treasury.difficulty.into(),
                    signer.pubkey(),
                )
                .await;
        }
    }

    Ok(())
}
