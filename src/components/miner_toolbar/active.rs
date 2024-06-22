use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::keccak::Hash as KeccakHash;

use crate::{
    components::{ActivityIndicator, Spinner, StopButton},
    hooks::{
        use_miner_toolbar_state, use_power_level, use_priority_fee, MinerStatusMessage, PowerLevel,
        PriorityFee, ReadMinerToolbarState, UpdateMinerToolbarState,
    },
    miner::{Miner, WEB_WORKERS},
};

#[component]
pub fn MinerToolbarActive(miner: Signal<Miner>) -> Element {
    let mut toolbar_state = use_miner_toolbar_state();

    // Animate the hash in the miner toolbar to visualize mining.
    // MI, use_future ==> use_resource
    let _ = use_resource(move || async move {
        loop {
            async_std::task::sleep(std::time::Duration::from_millis(125)).await;
            if let MinerStatusMessage::Searching = toolbar_state.status_message() {
                toolbar_state.set_display_hash(KeccakHash::new_unique());
            }
        }
    });

    if toolbar_state.is_open() {
        rsx! {
            div {
                class: "flex flex-col grow w-full gap-4 px-4 py-6 sm:px-8",
                div {
                    class: "flex flex-col w-full gap-2",
                    div {
                        class: "flex flex-row w-full justify-between",
                        h2 {
                            class: "text-3xl md:text-4xl lg:text-5xl text-white font-bold",
                            "Mining"
                        }
                        div {
                            class: "my-auto",
                            StopButton {}
                        }
                    }
                    match toolbar_state.status_message() {
                        MinerStatusMessage::Searching => {
                            rsx! {
                                p {
                                    class: "text-lg text-white",
                                    "Searching for a valid hash... "
                                }
                            }
                        }
                        MinerStatusMessage::Submitting => {
                            rsx! {
                                div {
                                    class: "flex flex-row gap-2",
                                    p {
                                        class: "text-lg text-white",
                                        "Submitting hash for validation..."
                                    }
                                    Spinner {
                                        class: "my-auto"
                                    }
                                }
                            }
                        }
                        MinerStatusMessage::Error => {
                            rsx! {
                                p {
                                    class: "text-lg text-white",
                                    "Error submitting transaction"
                                }
                            }
                        }
                        _ => None
                    }
                    match toolbar_state.status_message() {
                        MinerStatusMessage::Searching | MinerStatusMessage::Submitting => {
                            rsx! {
                                p {
                                    class: "font-mono text-sm truncate shrink opacity-80",
                                    "{toolbar_state.display_hash()}"
                                }
                            }
                        }
                        _ => None
                    }
                }
                PriorityFeeConfig {}
                PowerLevelConfig {}
                DownloadLink {}
            }
        }
    } else {
        rsx! {
            div {
                class: "flex flex-row gap-2 max-w-screen w-screen justify-start my-auto px-4 sm:px-8 object-contain",
                div {
                    class: "flex-shrink-0 flex-none my-auto",
                    ActivityIndicator {}
                }
                p {
                    class: "font-semibold text-white flex-shrink-0 flex-none my-auto",
                    "Mining"
                }
                div {
                    class: "flex-shrink flex-auto truncate my-auto",
                    match toolbar_state.status_message() {
                        MinerStatusMessage::Searching => {
                            rsx! {
                                p {
                                    class: "font-mono text-sm truncate flex-shrink flex-auto opacity-80 my-auto ml-2",
                                    "{toolbar_state.display_hash()}"
                                }
                            }
                        }
                        MinerStatusMessage::Submitting => {
                            rsx! {
                                p {
                                    class: "truncate flex-shrink flex-auto text-sm text-white opacity-80 my-auto ml-2",
                                    "Submitting hash for validation..."
                                }
                            }
                        }
                        MinerStatusMessage::Error => {
                            rsx! {
                                p {
                                    class: "truncate flex-shrink flex-auto text-sm text-white opacity-80 my-auto ml-2",
                                    "Error submitting transaction"
                                }
                            }
                        }
                        _ => None
                    }
                }
                div {
                    class: "flex-shrink-0 flex-none ml-auto my-auto",
                    StopButton {}
                }
            }
        }
    }
}

pub fn PriorityFeeConfig() -> Element {
    let mut priority_fee = use_priority_fee();

    rsx! {
        div {
            class: "flex flex-row gap-8 justify-between mt-8",
            div {
                class: "flex flex-col gap-1",
                p {
                    class: "text-white font-semibold",
                    "Priority fee"
                }
                p {
                    class: "text-white text-xs opacity-80 max-w-96",
                    "Add a priority fee to increase your chances of landing a transaction."
                }
           }
           div {
                class: "flex flex-row flex-shrink h-min gap-1 shrink mb-auto",
                input {
                    class: "bg-transparent text-white text-right px-1 mb-auto rounded font-semibold hover:bg-orange-600 transition-colors",
                    dir: "rtl",
                    step: 100_000,
                    min: 0,
                    max: 10_000_000,
                    r#type: "number",
                    value: "{priority_fee.read().0}",
                    oninput: move |e| {
                        if let Ok(v) = e.value().parse::<u64>() {
                            priority_fee.set(PriorityFee(v));
                        }
                    }
                }
                p {
                    class: "my-auto",
                    "microlamports"
                }
            }
        }
    }
}

pub fn PowerLevelConfig() -> Element {
    let mut power_level = use_power_level();
    let max = *WEB_WORKERS as i64;
    if max == 1 {
        // MI, for legacy algorithm, only 1 web worker, no choices
        return None;
    }

    rsx! {
        div {
            class: "flex flex-row gap-8 justify-between mt-8",
            div {
                class: "flex flex-col gap-1",
                p {
                    class: "text-white font-semibold",
                    "Power level"
                }
                p {
                    class: "text-white text-xs opacity-80 max-w-96",
                    "Select how many computer cores to dedicate to mining."
                }
            }
            div {
                class: "flex flex-row flex-shrink h-min gap-1 shrink mb-auto",
                input {
                    class: "bg-transparent text-white text-right px-1 mb-auto rounded font-semibold hover:bg-orange-600 transition-colors",
                    dir: "rtl",
                    step: 1,
                    min: 1,
                    max: max,
                    r#type: "number",
                    value: "{power_level.read().0}",
                    oninput: move |e| {
                        if let Ok(v) = e.value().parse::<u64>() {
                            power_level.set(PowerLevel(v));
                        }
                    }
                }
                p {
                    class: "my-auto",
                    "of {max} cores"
                }
            }
        }
    }
}

fn DownloadLink() -> Element {
    // if cfg!(feature = "web") {
    //     rsx! {
    //         div {
    //             class: "flex flex-row gap-2 mt-8 p-2.5 rounded bg-orange-600",
    //             WarningIcon {
    //                 class: "w-4 h-4 mt-0.5 shrink-0"
    //             }
    //             p {
    //                 class: "text-sm my-auto",
    //                 "You are mining from a web browser. For better performance, "
    //                 Link {
    //                     to: Route::Download {},
    //                     class: "font-medium underline",
    //                     "download the app."
    //                 }
    //             }
    //         }
    //     }
    // } else {
    //     None
    // }
    None
}
