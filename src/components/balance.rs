use dioxus::prelude::*;
use solana_extra_wasm::program::spl_token::amount_to_ui_amount;

use crate::{
    components::MarsIcon,
    hooks::{use_mars_balance, use_proof},
    route::Route,
};

pub fn Balance() -> Element {
    let balance = use_mars_balance();
    if let Some(balance) = balance.cloned() {
        let amount = balance
            .map(|b| b.real_number_string_trimmed())
            .unwrap_or_else(|_| "0.00".to_owned());
        rsx! {
            div {
                class: "flex flex-row w-full min-h-16 rounded justify-between",
                div {
                    class: "flex flex-col grow gap-2 sm:gap-4",
                    h2 {
                        class: "text-lg sm:text-xl md:text-2xl font-bold",
                        "Balance"
                    }
                    div {
                        class: "flex flex-row grow justify-between",
                        div {
                            class: "flex flex-row my-auto gap-2.5 md:gap-4",
                            MarsIcon {
                                class: "my-auto w-7 h-7 sm:w-8 sm:h-8 md:w-10 md:h-10"
                            }
                            h2 {
                                class: "text-3xl sm:text-4xl md:text-5xl",
                                // "{balance.real_number_string_trimmed()}"
                                "{amount}"
                            }
                        }
                        SendButton {}
                    }
                    UnclaimedRewards {}
                }
            }
        }
    } else {
        rsx! {
            div {
                class: "flex flex-row w-full min-h-16 grow loading rounded",
            }
        }
    }
}

pub fn UnclaimedRewards() -> Element {
    let proof = use_proof();

    if let Some(proof) = *proof.read() {
        if let Ok(proof) = proof {
            if proof.claimable_rewards.gt(&0) {
                return rsx! {
                    div {
                        class: "flex flex-row grow justify-between mt-4 -mr-2",
                        div {
                            class: "flex flex-col gap-2",
                            p {
                                class: "font-medium text-xs text-gray-300",
                                "Mining rewards"
                            }
                            div {
                                class: "flex flex-row gap-2",
                                MarsIcon {
                                    class: "my-auto w-4 h-4"
                                }
                                p {
                                    class: "font-semibold",
                                    "{amount_to_ui_amount(proof.claimable_rewards, mars::TOKEN_DECIMALS)}"
                                }
                            }
                        }
                        span {
                            class: "mt-auto",
                            ClaimButton {}
                        }
                    }
                };
            }
        }
    }

    None
}

#[component]
pub fn SendButton(to: Option<String>) -> Element {
    rsx! {
        Link {
            to: Route::Send { to: to.clone().unwrap_or("".to_string()) },
            class: "flex h-10 w-10 my-auto rounded-full justify-center text-2xl font-bold transition-all bg-black text-white hover:shadow hover:scale-110 dark:bg-white dark:text-black",
            span {
                class: "my-auto bg-transparent",
                "↑"
            }
        }
    }
}

pub fn ClaimButton() -> Element {
    rsx! {
        Link {
            class: "flex transition transition-colors font-semibold px-3 h-10 rounded-full hover-100 active-200",
            to: Route::Claim {},
            span {
                class: "my-auto",
                "Claim"
            }
        }
    }
}
