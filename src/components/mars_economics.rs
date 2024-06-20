use dioxus::prelude::*;

// TODO Live supply
// TODO Live circulating supply

// TODO 10 MARS/min
// TODO Emphasis on simple predictability and fairness.
// TODO Linear supply inflation provides a baseline incentive for lending and spending
// TODO Protection from exponential inflation
// TODO Longterm sustainability in a way that deflationary currencies (such as BTC and ETH) do not.
// TODO This supply rate will be the same 100 years from now as it is today
// TODO Each generation will see approximately the same number of tokens mined (XYZ over the average 80 year human lifetime).
// TODO 21 million new supply every ~40 years.
// TODO Claims

pub fn MarsTokenomics() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-4 h-full font-hero max-w-3xl w-full mx-auto pb-20 leading-7",
            p {
                class: "text-4xl font-bold",
                "Mars tokenomics"
            }
            p {
                "(Coming soon...)"
            }
            p {
                "Here are the key points:"
                li {
                    class: "ml-2",
                    "Mars has a theoretical maximum supply limit of 26,294,400, but the actual supply will be less than the maximum."
                }
                li {
                    class: "ml-2",
                    "Mars supply grows at a linear rate of 10 MARS/min."
                }
                li {
                    class: "ml-2",
                    "It will take 5 years for the total Mars supply to reach 26 million tokens."
                }
                li {
                    class: "ml-2",
                    "Since the whole Mars program is immutable and final, Mars will evolve or perish on its own, becoming a precious natural mineral."
                }
                li {
                    class: "ml-2",
                    "The clock is ticking on the Mars Mining Mission and the die has been cast."
                }
            }
        }
    }
}
