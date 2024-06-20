use dioxus::prelude::*;
use mars::END_AT;
use ore_types::Transfer;
use solana_extra_wasm::program::spl_token::amount_to_ui_amount;
use web_time::{Duration, SystemTime, UNIX_EPOCH};

use crate::{
    components::{ActivityIndicator, Footer, MarsIcon, MarsLogoIcon},
    hooks::{use_is_onboarded, use_mars_supply, use_transfers, use_treasury, ActivityFilter},
    route::Route,
    utils::asset_path,
};

pub fn Landing() -> Element {
    let nav = navigator();
    let is_onboarded = use_is_onboarded();

    // If the user is already onboarded, redirect to home.
    if is_onboarded.read().0 {
        nav.replace(Route::Home {});
    }

    rsx! {
        div {
            class: "flex flex-col",
            Hero {}
            Block {
                title: &"Proof of work",
                title2: &"On Solana blockchain",
                detail: &"Mars uses a novel mining protocol designed for fair token distribution. It guarantees no miner can ever be starved out from earning rewards.",
                section: Section::A
            }
            Block {
                title: &"Stable supply",
                title2: &"Steady growth",
                detail: &"Mars has an limited total supply of less than 26 million tokens. On average, ten(10) new Mars token is mined every minute by miners around the globe.",
                section: Section::B
            }
            Block {
                title: &"Fair launch",
                title2: &"Immutable code",
                detail: &"Mars has no insider token allocation nor pre-mined supply. The smart contract has been frozen and open-sourced to prevent tampering or removal. It's a pure natural wild mine with no owners, no manipulators. Mars will prosper or perish on its own. The clock is ticking and the die has been cast.",
                section: Section::C
            }
            Footer {}
        }
    }
}

fn Navbar() -> Element {
    rsx! {
        div {
            class: "flex flex-row justify-between px-4 sm:px-8 py-8 w-full z-50",
            Link {
                to: Route::Landing {},
                class: "flex flex-row h-10",
                MarsLogoIcon {
                    class: "h-6 md:h-8"
                }
            }
        }
    }
}

fn Hero() -> Element {
    let bg_img = asset_path("smoke.jpg");
    rsx! {
        div {
            class: "bg-white",
            div {
                class: "flex flex-col w-full h-screen z-20 bg-cover bg-center",
                style: "background-image: url({bg_img})",
                Navbar {}
                div {
                    class: "flex flex-col gap-y-8 sm:gap-y-10 md:gap-y-12 mx-auto my-auto pb-24 px-4 sm:px-8",
                    div {
                        class: "flex flex-col gap-y-4 sm:gap-y-6 md:gap-y-8",
                        p {
                            class: "text-center text-4xl min-[480px]:text-5xl min-[600px]:text-6xl md:text-7xl lg:text-8xl font-bold font-hero text-black",
                            // "Mars landing time!"
                            " "
                        }
                        div {
                            // MI
                        }
                        div {
                            // MI
                        }
                        div {
                            // MI
                        }
                        div {
                            // MI
                        }
                        div {
                            // MI
                        }
                        // p {
                        //     class: "text-xl sm:text-2xl md:text-3xl lg:text-4xl text-center max-w-[46rem] font-hero leading-7 text-white",
                        //     "The clock is ticking..."
                        // }
                    }
                    Link {
                        class: "mx-auto sm:text-lg md:text-xl lg:text-2xl font-semibold bg-orange-500 hover:bg-orange-600 active:bg-orange-700 text-white transition-colors rounded-full px-6 py-3",
                        to: Route::Home {},
                        "Get started →"
                    }
                }
            }
        }
    }
}

#[component]
fn Block(title: String, title2: String, detail: String, section: Section) -> Element {
    let colors = match section {
        Section::A => "bg-black text-white",
        Section::B => "bg-white text-black",
        Section::C => "bg-orange-500 text-white",
    };
    let height = match section {
        Section::A | Section::B => "min-h-svh h-full",
        Section::C => "",
    };
    rsx! {
        div {
            class: "flex w-full z-20 {colors} {height}",
            div {
                class: "flex flex-col h-full w-full py-16 gap-24 px-4 sm:px-8",
                div {
                    class: "flex flex-col gap-4 sm:gap-6 md:gap-8",
                    p {
                        class: "text-3xl sm:text-4xl md:text-5xl lg:text-6xl font-bold font-hero",
                        "{title}"
                        br {}
                        span {
                            class: "opacity-50",
                            "{title2}"
                        }
                    }
                    p {
                        class: "text-lg sm:text-xl md:text-2xl lg:text-3xl leading-relaxed max-w-[48rem] font-hero",
                        "{detail}"
                    }
                    BlockCta {
                        section: section.clone()
                    }
                }
                div {
                    class: "flex h-full w-full",
                    match section {
                        // MI
                        // Section::A => rsx! { SectionA {} },
                        Section::B => rsx! { SectionB {} },
                        _ => None
                    }
                }
            }
        }
    }
}

#[component]
fn BlockCta(section: Section) -> Element {
    match section {
        Section::A => rsx! {
            Link {
                class: "font-semibold mt-4",
                to: Route::WhatIsMining {},
                "Learn more →"
            }
        },
        Section::B => rsx! {
            Link {
                class: "font-semibold mt-4",
                to: Route::MarsTokenomics {},
                "Learn more →"
            }
        },
        Section::C => rsx! {
            Link {
                class: "font-semibold mt-4",
                to: "https://github.com/miraland-labs/mars",
                "Read the code →"
            }
        },
    }
}

#[derive(Clone, PartialEq, Eq)]
enum Section {
    A,
    B,
    C,
}

fn SectionA() -> Element {
    rsx! {
        div {
            class: "flex flex-col w-full my-auto gap-4 max-w-[48rem]",
            div {
                class: "flex flex-row gap-2",
                ActivityIndicator {}
                p {
                    class: "font-semibold text-xl opacity-50",
                    "Live transactions"
                }
            }
            div {
                class: "flex flex-col w-full",
                TransfersSection {}
            }
        }
    }
}

fn TransfersSection() -> Element {
    let filter = use_signal(|| ActivityFilter::Global);
    let offset = use_signal(|| 0);
    let transfers = use_transfers(filter, offset);
    let e = if let Some(transfers) = transfers.read().clone() {
        match transfers {
            Ok(transfers) => {
                rsx! {
                    if transfers.data.is_empty() {
                        p {
                            class: "text-sm opacity-50",
                            "No transactions yet"
                        }
                    }
                    for (i, transfer) in transfers.data.iter().enumerate() {
                        if i.le(&5) {
                            SimpleTransferRow {
                                transfer: transfer.clone()
                            }
                        } else {
                            div {}
                        }
                    }
                }
            }
            _ => None,
        }
    } else {
        None
    };
    e
}

#[component]
fn SimpleTransferRow(transfer: Transfer) -> Element {
    let addr = transfer.to_address[..5].to_string();
    let amount = amount_to_ui_amount(transfer.amount as u64, mars::TOKEN_DECIMALS);
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let ts = Duration::from_secs(transfer.ts as u64);
    let time = now.saturating_sub(ts);
    let t = time.as_secs();
    const ONE_MIN: u64 = 60;
    const ONE_HOUR: u64 = ONE_MIN * 60;
    const ONE_DAY: u64 = ONE_HOUR * 24;
    let time_str = if t.gt(&ONE_DAY) {
        format!("{}d ago", t.saturating_div(ONE_DAY))
    } else if t.gt(&ONE_HOUR) {
        format!("{}h ago", t.saturating_div(ONE_HOUR))
    } else if t.gt(&ONE_MIN) {
        format!("{}m ago", t.saturating_div(ONE_MIN))
    } else {
        format!("{}s ago", t)
    };

    rsx! {
        div {
            class: "flex flex-row py-3 gap-3 w-full transition-colors rounded hover:bg-gray-900 px-2 -mx-2",
            div {
                class: "flex flex-col pt-1",
                p {
                    class: "flex flex-row gap-2",
                    span {
                        class: "font-mono font-bold",
                        "{addr}"
                    }
                    "mined "
                    span {
                        class: "flex flex-row font-semibold gap-0.5",
                        MarsIcon {
                            class: "w-3.5 h-3.5 my-auto",
                        }
                        "{amount:.4}"
                    }
                }
            }
            div {
                class: "flex pt-1.5 ml-auto",
                p {
                    class: "opacity-50 text-right text-nowrap text-sm",
                    "{time_str}"
                }
            }
        }
    }
}

fn SectionB() -> Element {
    let treasury = use_treasury();
    let supply = use_mars_supply();

    let circulating_supply = if let Some(treasury) = *treasury.read() {
        if let Ok(treasury) = treasury {
            if treasury.total_claimed_rewards.ge(&0) {
                (treasury.total_claimed_rewards as f64) / 10f64.powf(mars::TOKEN_DECIMALS as f64)
            } else {
                0f64
            }
        } else {
            0f64
        }
    } else {
        0f64
    }
    .to_string();

    let current_supply = supply
        .cloned()
        .and_then(|s| s.ok())
        .map(|s| s.ui_amount_string)
        .unwrap_or_else(|| "Err".to_string());

    let current_supply_amount = supply
        .cloned()
        .and_then(|s| s.ok())
        .map(|s| s.ui_amount)
        .unwrap_or(Some(0.))
        .unwrap();

    let current_unix_timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    let max_supply = if current_unix_timestamp.lt(&(END_AT as u64)) {
            let time_remaining_mins = (END_AT as u64 - current_unix_timestamp) as f64 / 60f64;
            // (current_supply_amount + time_remaining_mins * 10.).to_string()
            format!("{max_supply:.prec$}", max_supply = current_supply_amount + time_remaining_mins * 10.0, prec = mars::TOKEN_DECIMALS as usize)
    } else {
        "Max supply reached".to_string()
    };

    rsx! {
        div {
            class: "flex flex-col gap-12 my-auto",
            MarsValue {
                title: "Circulating supply".to_string(),
                amount: circulating_supply
            }
            MarsValue {
                title: "Current total supply".to_string(),
                amount: current_supply
            }
            MarsValue {
                title: "Max supply forecast".to_string(),
                amount: max_supply
            }
        }
    }
}

#[component]
fn MarsValue(title: String, amount: String) -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-3",
            p {
                class: "text-gray-300 text-sm font-medium",
                "{title}"
            }
            div {
                class: "flex flex-row gap-2",
                MarsIcon {
                    class: "w-6 h-6 md:w-7 md:h-7 lg:w-8 lg:h-8 my-auto"
                }
                p {
                    class: "text-2xl md:text-3xl lg:text-4xl font-bold font-hero",
                    "{amount}"
                }
            }
        }

    }
}

#[component]
fn QuestionBreak() -> Element {
    rsx! {
        div {
            class: "bg-orange-500 text-white w-full py-16",
            p {
                class: "text-xl sm:text-2xl md:text-3xl lg:text-4xl font-bold font-hero text-center",
                "How much will you mine?"
            }
        }
    }
}
