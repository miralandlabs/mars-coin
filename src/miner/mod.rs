mod web_worker;

pub use web_worker::*;

use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_std::utils::channel::UseChannel;
use lazy_static::lazy_static;
use mars::{state::Proof, state::Treasury, BUS_COUNT, EPOCH_DURATION};
use rand::Rng;
use serde_wasm_bindgen::to_value;
use solana_client_wasm::solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    keccak::Hash as KeccakHash,
    pubkey::Pubkey,
    signature::Signature,
    signer::Signer,
};
use web_sys::{window, Worker};
use web_time::Duration;

use crate::{
    gateway::{signer, Gateway, GatewayResult, CU_LIMIT_MINE, CU_LIMIT_RESET},
    hooks::{
        MinerStatus, MinerStatusMessage, MinerToolbarState, PowerLevel, PriorityFee,
        ReadMinerToolbarState, UpdateMinerToolbarState,
    },
};

// Number of physical cores on machine
lazy_static! {
    pub static ref WEB_WORKERS: usize = fetch_logical_processors();
}

// MI, only 1 web worker for current algorithm
fn fetch_logical_processors() -> usize {
    // MI
    let _window = window().expect("should have a window");
    // let navigator = window.navigator();
    // navigator.hardware_concurrency() as usize
    1
}

/// Miner encapsulates the logic needed to efficiently mine for valid hashes according to the application runtime and hardware.
pub struct Miner {
    _power_level: Signal<PowerLevel>,
    priority_fee: Signal<PriorityFee>,
    web_workers: Vec<Worker>,
}

impl Miner {
    pub fn new(
        cx: UseChannel<WebWorkerResponse>,
        _power_level: Signal<PowerLevel>,
        priority_fee: Signal<PriorityFee>,
    ) -> Self {
        Self {
            _power_level: _power_level.clone(),
            priority_fee: priority_fee.clone(),
            // MI, only 1 web worker for current algorithm
            // web_worker: create_wweb_orker(cx.clone()),
            // web_worker: std::array::from_fn(|_| create_web_worker(cx.clone())),
            web_workers: (0..*WEB_WORKERS)
                .map(|_| create_web_worker(cx.clone()))
                .collect(),
        }
    }

    pub async fn start_mining(&self, hash: KeccakHash, difficulty: KeccakHash, signer: Pubkey) {
        self.start_mining_web(hash, difficulty, signer).await;
    }

    pub async fn start_mining_web(&self, hash: KeccakHash, difficulty: KeccakHash, signer: Pubkey) {
        let interval = u64::MAX.saturating_div(self.web_workers.len() as u64);
        for (i, web_worker) in self.web_workers.iter().enumerate() {
            let nonce = interval.saturating_mul(i as u64);
            web_worker
                .post_message(
                    &to_value(
                        &(WebWorkerRequest {
                            hash,
                            difficulty,
                            nonce,
                            pubkey: signer,
                        }),
                    )
                    .unwrap(),
                )
                .unwrap();
        }
    }

    pub async fn process_web_worker_results(
        &self,
        messages: &Vec<WebWorkerResponse>,
        toolbar_state: &mut Signal<MinerToolbarState>,
        proof: &mut Resource<GatewayResult<Proof>>,
        treasury: &Resource<GatewayResult<Treasury>>,
        gateway: Rc<Gateway>,
        pubkey: Pubkey,
    ) {
        log::info!("web worker response: {:?}", messages);

        let solution = messages[0].clone();
        // Update toolbar state
        toolbar_state.set_display_hash(solution.clone().hash);
        toolbar_state.set_status_message(MinerStatusMessage::Submitting);
        let priority_fee = self.priority_fee.read().0;

        // Submit solution
        match submit_solution(&gateway, &solution, priority_fee, &treasury.clone()).await {
            // Start mining again
            Ok(sig) => {
                log::info!("Success: {}", sig);
                proof.restart();
                if let MinerStatus::Active = toolbar_state.status() {
                    toolbar_state.set_status_message(MinerStatusMessage::Searching);
                        if let Ok(proof) = gateway.get_proof(pubkey).await {
                            if let Some(treasury) = *treasury.read() {
                                if let Ok(treasury) = treasury {
                                    self.start_mining(proof.hash.into(), treasury.difficulty.into(), pubkey)
                                    .await;
                                }
                            }
                        }
                }
            }

            // Display error
            Err(err) => {
                toolbar_state.set_status_message(MinerStatusMessage::Error);
                log::error!("Failed to submit hash: {:?}", err);
            }
        }
    }
}

pub async fn submit_solution(
    gateway: &Rc<Gateway>,
    res: &WebWorkerResponse,
    priority_fee: u64,
    treasury: &Resource<GatewayResult<Treasury>>,
) -> GatewayResult<Signature> {
    // Submit mine tx.
    let next_hash = res.hash;
    let nonce = res.nonce;
    let signer = signer();

    // Find a valid bus
    let mut rng = rand::thread_rng();
    loop {
        // Check if epoch needs to be reset
        if let Ok(clock) = gateway.get_clock().await {
            if let Some(treasury) = *treasury.read() {
                if let Ok(treasury) = treasury {
                    let epoch_end_at = treasury.last_reset_at.saturating_add(EPOCH_DURATION);

                    // Submit restart epoch tx, if needed
                    if clock.unix_timestamp.ge(&epoch_end_at) {
                        // There are a lot of miners right now, randomize who tries the reset
                        let selected_to_reset = rng.gen_range(0..10).eq(&0);
                        if selected_to_reset {
                            let cu_limit_ix =
                                ComputeBudgetInstruction::set_compute_unit_limit(CU_LIMIT_RESET);
                            let cu_price_ix =
                                ComputeBudgetInstruction::set_compute_unit_price(priority_fee);
                            let ix = mars::instruction::reset(signer.pubkey());
                            gateway
                                .send_and_confirm(&[cu_limit_ix, cu_price_ix, ix], false, true)
                                .await
                                .ok();
                        }
                    }
                }
            }
        }

        // Submit mine tx
        // let bus_id = pick_bus();
        let bus_id = if let Some(treasury) = *treasury.read() {
            if let Ok(treasury) = treasury {
                find_open_bus(gateway, treasury.reward_rate).await
            } else {
                pick_bus()
            }
        } else {
            pick_bus()
        };
        log::info!("Using bus {}", bus_id);
        let cu_limit_ix = ComputeBudgetInstruction::set_compute_unit_limit(CU_LIMIT_MINE);
        let cu_price_ix = ComputeBudgetInstruction::set_compute_unit_price(priority_fee);
        let ix = mars::instruction::mine(
            signer.pubkey(),
            mars::BUS_ADDRESSES[bus_id],
            next_hash.into(),
            nonce,
        );

        match gateway
            .send_and_confirm(&[cu_limit_ix, cu_price_ix, ix], false, false)
            .await
        {
            Ok(sig) => return Ok(sig),
            Err(err) => {
                // TODO Retry
                // TODO It seems this can error can occur sometimes, even while tx was submitted
                log::error!("Error submitting: {:?}", err);
            }
        }
    }
}

fn pick_bus() -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..BUS_COUNT)
}

async fn find_open_bus(gateway: &Rc<Gateway>, reward_rate: u64) -> usize {
    // Find a valid bus
    let mut rng = rand::thread_rng();
    loop {
        let bus_id = rng.gen_range(0..BUS_COUNT);
        if let Ok(bus) = gateway.get_bus(bus_id).await {
            // MI
            // if bus.rewards.gt(&reward_rate.saturating_mul(4)) {
            if bus.rewards.ge(&reward_rate.saturating_mul(1)) {
                return bus_id;
            }
        }
        async_std::task::sleep(Duration::from_secs(1)).await;
    }
}
