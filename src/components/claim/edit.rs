use dioxus::prelude::*;
use solana_extra_wasm::program::spl_token::amount_to_ui_amount;

use crate::components::WarningIcon;

use super::ClaimStep;

#[component]
pub fn ClaimEdit(
    claim_step: Signal<ClaimStep>,
    amount_input: Signal<String>,
    parsed_amount: u64,
    max_rewards: u64,
) -> Element {
    let nav = navigator();
    let error_text: Option<String> = if parsed_amount.gt(&max_rewards) {
        Some("Amount too large".to_string())
    } else {
        None
    };
    let is_disabled = amount_input.read().len().eq(&0)
        || amount_input.read().parse::<f64>().is_err()
        || error_text.is_some();

    rsx! {
        div {
            class: "flex flex-col h-full grow justify-between",
            div {
                class: "flex flex-col gap-3",
                h2 {
                    "Claim"
                }
                p {
                    class: "text-lg",
                    "Select an amount of rewards to claim."
                }
                p {
                    class: "text-sm text-gray-300 dark:text-gray-700",
                    "Upon claiming, this amount will be added to your balance in the dashboard."
                }
            }
            div {
                class: "flex flex-col gap-8",
                if let Some(error_text) = error_text {
                    p {
                        class: "flex flex-row flex-nowrap gap-2 text-white w-min mx-auto text-nowrap bg-red-500 text-center font-semibold text-sm rounded py-1 px-2",
                        WarningIcon {
                            class: "w-3.5 h-3.5 my-auto"
                        }
                        "{error_text}"
                    }
                }
                input {
                    autofocus: true,
                    class: "mx-auto w-full text-center focus:ring-0 outline-none placeholder-gray-200 dark:placeholder-gray-800 bg-transparent text-3xl sm:text-4xl md:text-5xl font-medium",
                    value: "{amount_input}",
                    placeholder: "0",
                    oninput: move |evt| {
                        let s = evt.value();
                        if s.len().eq(&0) || s.parse::<f64>().is_ok() {
                            amount_input.set(s);
                        } else {
                            amount_input.set(s[..s.len()-1].to_string());
                        }
                    },
                }
                button {
                    class: "flex transition-colors shrink text-nowrap py-2 px-4 mx-auto text-center text-nowrap rounded-full font-medium hover-100 active-200",
                    onclick: move |_| {
                        amount_input.set(amount_to_ui_amount(max_rewards, mars::TOKEN_DECIMALS).to_string())
                    },
                    "Max: {amount_to_ui_amount(max_rewards, mars::TOKEN_DECIMALS)}"
                }
            }
            div {
                class: "flex flex-col sm:flex-row gap-2",
                button {
                    class: "w-full py-3 rounded font-semibold transition-colors hover-100 active-200",
                    onclick: move |_| {
                        nav.go_back();
                    },
                    "Cancel"
                }
                button {
                    class: "w-full py-3 rounded font-semibold transition-colors transition-opacity text-white bg-orange-500 hover:bg-orange-600 active:bg-orange-700 disabled:opacity-20",
                    disabled: is_disabled,
                    onclick: move |_| {
                        claim_step.set(ClaimStep::Confirm);
                    },
                    "Review"
                }
            }
        }
    }
}
