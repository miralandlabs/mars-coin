use dioxus::prelude::*;

use crate::route::Route;

pub fn SendDone() -> Element {
    rsx! {
        div {
            class: "flex flex-col grow justify-between",
            div {
                class: "flex flex-col gap-3",
                h2 {
                    "Success!"
                }
                p {
                    class: "text-lg",
                    "You have sent Mars."
                }
                // p {
                //     class: "text-sm text-gray-300 dark:text-gray-700",
                //     "You can now spend and transfer your Mars from the dashboard."
                // }
            }
            div {
                class: "flex flex-col gap-3",
                div {
                    class: "h-full"
                }
                Link {
                    class: "w-full py-3 rounded font-semibold transition-colors text-center text-white bg-orange-500 hover:bg-orange-600 active:bg-orange-700",
                    to: Route::Home{},
                    "Done"
                }
            }
        }
    }
}
