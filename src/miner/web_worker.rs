use dioxus_std::utils::channel::UseChannel;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
use web_sys::{DedicatedWorkerGlobalScope, MessageEvent, Worker, WorkerOptions, WorkerType};
use solana_client_wasm::solana_sdk::{
    keccak::{self, Hash as KeccakHash},
    pubkey::Pubkey,
};

/// Mining request for web workers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebWorkerRequest {
    pub hash: KeccakHash,
    pub difficulty: KeccakHash,
    pub nonce: u64,
    pub pubkey: Pubkey,
}

/// Mining response for web workers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebWorkerResponse {
    pub hash: KeccakHash,
    pub nonce: u64,
}

#[wasm_bindgen]
pub fn start_worker() {
    log::info!("Starting webworker");

    let self_ = js_sys::global();
    let js_value = std::ops::Deref::deref(&self_);
    let scope = DedicatedWorkerGlobalScope::unchecked_from_js_ref(js_value);
    let scope_ = scope.clone();

    scope.set_onmessage(Some(&js_sys::Function::unchecked_from_js(
        Closure::<dyn Fn(MessageEvent)>::new(move |e: MessageEvent| {
            let req: WebWorkerRequest = from_value(e.data()).unwrap();
            let res = find_next_hash(req.hash, req.difficulty, req.nonce, req.pubkey);
            scope_.post_message(&to_value(&res).unwrap()).unwrap();
        })
        .into_js_value(),
    )))
}

fn worker_options() -> WorkerOptions {
    let mut options = WorkerOptions::new();
    options.type_(WorkerType::Module);
    options
}

pub fn create_web_worker(cx: UseChannel<WebWorkerResponse>) -> Worker {
    log::info!("Creating worker...");
    let worker = Worker::new_with_options("worker.js", &worker_options()).unwrap();

    // On message
    worker.set_onmessage(Some(&js_sys::Function::unchecked_from_js(
        Closure::<dyn Fn(MessageEvent)>::new(move |e: MessageEvent| {
            let res: WebWorkerResponse = from_value(e.data()).unwrap();
            async_std::task::block_on({
                let cx = cx.clone();
                async move {
                    cx.send(res).await.ok();
                }
            });
        })
        .into_js_value(),
    )));

    // On error
    worker.set_onerror(Some(&js_sys::Function::unchecked_from_js(
        Closure::<dyn Fn(MessageEvent)>::new(move |e: MessageEvent| {
            log::info!("Error from worker: {:?}", e.data());
        })
        .into_js_value(),
    )));

    worker
}

pub fn find_next_hash(hash: KeccakHash, difficulty: KeccakHash, nonce: u64, signer: Pubkey) -> WebWorkerResponse {
    let mut next_hash: KeccakHash;
    // let mut nonce = 0u64;
    let mut nonce = nonce;
    loop {
        next_hash = keccak::hashv(&[
            hash.as_ref(),
            signer.as_ref(),
            nonce.to_le_bytes().as_slice(),
        ]);
        if next_hash.le(&difficulty) {
            break;
        }
        nonce += 1;
    }
    WebWorkerResponse {
        hash: next_hash,
        nonce,
    }
}
