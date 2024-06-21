use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::{
    bs58, native_token::lamports_to_sol, signature::Keypair, signer::Signer,
};

use crate::{
    components::EyeSlashIcon,
    hooks::{use_gateway, use_keypair_persistent, use_sol_balance},
    route::Route,
};

#[derive(Copy, Clone)]
pub enum ImportKeyStep {
    Loading,
    Warning,
    Import,
}

// MI
#[component]
pub fn ImportKey() -> Element {
    let mut step = use_signal(|| ImportKeyStep::Loading);
    let sol_balance = use_sol_balance();

    use_effect(move || {
        let current_step = *step.read();
        if let ImportKeyStep::Loading = current_step {
            if let Some(Ok(sol_balance)) = *sol_balance.read() {
                if sol_balance.gt(&0) {
                    step.set(ImportKeyStep::Warning)
                } else {
                    step.set(ImportKeyStep::Import)
                }
            }
        }
    });

    let e = match *step.read() {
        ImportKeyStep::Loading => {
            rsx! {
                ImportKeyLoading {}
            }
        }
        ImportKeyStep::Warning => {
            if let Some(Ok(balance)) = *sol_balance.read() {
                rsx! {
                    ImportKeyWarning { step, balance }
                }
            } else {
                // TODO This should never happen. Display error
                None
            }
        }
        ImportKeyStep::Import => {
            rsx! {
                ImportKeyImport {}
            }
        }
    };

    e
}

// MI
#[component]
fn ImportKeyLoading() -> Element {
    rsx! {
        div {
            class: "flex flex-row h-64 w-full loading rounded",
        }
    }
}

#[component]
fn ImportKeyWarning(step: Signal<ImportKeyStep>, balance: u64) -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-16 grow w-full h-full",
            ImportKeyHeader {}
            LossOfFundsWarning {
                balance
            }
            button {
                onclick: move |_| {
                    log::info!("Handlerr....");
                    step.set(ImportKeyStep::Import)
                },
                class: "text-red-500 hover:bg-red-500 active:bg-red-600 hover:text-white mt-auto py-3 w-full rounded text-center font-semibold transition-colors",
                "I understand I will lose access to my funds if I have not backed up my keypair"
            }
        }
    }
}

// MI
#[component]
fn ImportKeyHeader() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-3",
            h2 {
                "Import key"
            }
            p {
                class: "text-lg",
                "Recover a prior mining session from a backed up keypair. "
            }
            p {
                class: "text-sm text-gray-300 dark:text-gray-700",
                "Never import a private key generated by another app or wallet."
            }
        }
    }
}

const KEY_LENGTH: usize = 64;

// MI
#[component]
fn ImportKeyImport() -> Element {
    let mut sol_balance = use_signal::<Option<u64>>(|| None);
    let mut keypair_persistent = use_keypair_persistent();
    let mut err_msg = use_signal::<Option<String>>(|| None);
    let mut enable_import_button = use_signal(|| false);
    let mut private_key_input = use_signal(|| "".to_string());
    let gateway = use_gateway();
    // let nav = navigator(); // MI
    let nav = use_navigator();
    log::info!("OK: ... {}", private_key_input.read().clone()); // MI

    use_future(move || {
        // let private_key_input = private_key_input.clone();
        // let sol_balance = sol_balance.clone();
        // let enable_import_button = enable_import_button.clone();
        // let err_msg = err_msg.clone();
        let gateway = gateway.clone();
        async move {
            if let Ok(bytes) = bs58::decode(private_key_input.read().clone()).into_vec() {
                if bytes.len().eq(&KEY_LENGTH) {
                    if let Ok(kp) = Keypair::from_bytes(&bytes) {
                        enable_import_button.set(true);
                        match gateway.rpc.get_balance(&kp.pubkey()).await {
                            Ok(b) => sol_balance.set(Some(b)),
                            Err(_err) => sol_balance.set(None),
                        }
                    }
                } else if bytes.len().eq(&0) {
                    sol_balance.set(None);
                    enable_import_button.set(false);
                    err_msg.set(None);
                } else {
                    sol_balance.set(None);
                    enable_import_button.set(false);
                    err_msg.set(Some("Invalid length".to_string()));
                }
            } else {
                sol_balance.set(None);
                enable_import_button.set(false);
                err_msg.set(Some("Invalid format".to_string()));
            }
        }
    });

    rsx! {
        div {
            class: "flex flex-col gap-16 grow w-full h-full",
            ImportKeyHeader {}
            EyeSlashIcon {
                class: "w-12 h-12 mx-auto opacity-50"
            }
            div {
                class: "flex flex-col gap-2",
                input {
                    class: "mx-auto w-full py-2 text-center placeholder-gray-200 dark:placeholder-gray-700 bg-transparent",
                    autofocus: true,
                    placeholder: "Private key",
                    value: "{*private_key_input.read()}",
                    oninput: move |e| {
                        private_key_input.set(e.value());
                    },
                }
                if let Some(err_msg) = err_msg.read().clone() {
                    p {
                        class: "text-red-500 text-sm font-right",
                        "{err_msg}"
                    }
                }
            }
            if let Some(sol_balance) = *sol_balance.read() {
                p {
                    class: "text-nowrap mx-auto text-center font-semibold",
                    "Balance: {lamports_to_sol(sol_balance)} SOL"
                }
            } else {
                div {
                    class: "flex flex-row w-24 h-16 loading rounded-full",
                }
            }
            button {
                disabled: !*enable_import_button.read(),
                onclick: move |_| {
                    keypair_persistent.set(private_key_input.read().clone());
                    nav.push(Route::Settings {});
                },
                class: "bg-orange-500 disabled:opacity-50 hover:bg-orange-600 active:bg-orange-700 transition-colors text-white rounded text-center font-semibold py-3 mt-auto",
                "Import"
            }
        }
    }
}

#[component]
fn LossOfFundsWarning(balance: u64) -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-3 bg-red-500 w-full rounded px-4 py-5 mt-8 text-white",
            p {
                class: "font-bold text-xl",
                "Your current keypair has funds on it!"
            }
            ul {
                class: "list-disc list-outside pl-4 space-y-1.5",
                li {
                    "Your current keypair has a balance of {lamports_to_sol(balance)} SOL. "
                }
                li {
                    "Importing a new keypair will replace your current one. "
                }
                li {
                    "Please ensure you have safely backed up your keypair to avoid losing access to your funds."
                }
            }
        }
    }
}
