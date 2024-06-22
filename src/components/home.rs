use dioxus::prelude::*;

use crate::{
    components::{BackupKeypairWarning, Balance},
    hooks::use_show_backup_warning,
};

pub fn Home() -> Element {
    let show_backup_warning = use_show_backup_warning();
    rsx! {
        div {
            class: "flex flex-col gap-16 overflow-visible",
            if cfg!(feature = "web") && show_backup_warning.read().0 {
                BackupKeypairWarning {}
            }
            Balance {}
            // MI
            // Activity {}
        }
    }
}
