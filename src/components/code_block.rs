use dioxus::prelude::*;
#[cfg(feature = "desktop")]
use dioxus_std::clipboard::use_clipboard;
use crate::components::CopyIcon;
#[cfg(feature = "web")]
use crate::hooks::use_clipboard;

#[component]
pub fn CodeBlock(text: String) -> Element {
    let clipboard = use_clipboard();
    let mut solid = use_signal(|| false);
    // MI, use_future ==> use_resource
    let _ = use_resource(move || async move {
        if *solid.read() {
            async_std::task::sleep(std::time::Duration::from_secs(3)).await;
            solid.set(false);
        }
    });
    let strs = text.split('\n');
    rsx! {
        div {
            class: "flex flex-row justify-between py-2 pl-4 pr-2 bg-gray-100 text-black dark:bg-gray-900 dark:text-white font-mono rounded",
            div {
                class: "flex flex-col",
                for s in strs {
                    p {
                        "{s}"
                    }
                }
            }
            button {
                class: "flex shrink-0 px-2 py-1 mb-auto rounded hover-100 active-200 transition-colors",
                onclick: move |_e| {
                    // clipboard.set(text.clone()).ok();

                    #[cfg(feature = "web")]
                    if let Some(cb) = clipboard.clone() {
                        let _ = cb.write_text(&text);
                    }

                    #[cfg(feature = "desktop")]
                    clipboard.set(text.clone()).ok();
                    
                    solid.set(true);
                },
                CopyIcon {
                    class: "w-4 h-4 my-auto",
                    solid: *solid.read(),
                }
            }
        }
    }
}
