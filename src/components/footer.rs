use dioxus::prelude::*;

use crate::components::{GithubIcon, MarsLogoIcon, XIcon};

pub fn Footer() -> Element {
    rsx! {
        div {
            class: "flex flex-row bg-black text-white w-full py-6 sm:py-10 px-4 sm:px-8 justify-between",
            MarsLogoIcon {
                class: "h-6 md:h-8 my-auto"
            }
            div {
                class: "flex flex-row gap-8",
                Link {
                    to: "https://github.com/miraland-labs/mars",
                    class: "flex h-10 w-10 hover:bg-gray-900 active:bg-gray-800 transition-colors rounded-full text-white",
                    GithubIcon {
                        class: "w-6 h-6 m-auto"
                    }
                }
                Link {
                    to: "https://x.com/carfoopo",
                    class: "flex h-10 w-10 hover:bg-gray-900 active:bg-gray-800 transition-colors rounded-full text-white",
                    XIcon {
                        class: "w-5 h-5 m-auto"
                    }
                }
            }
        }
    }
}
