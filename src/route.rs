use dioxus::prelude::*;

use crate::components::{
    Claim, ExportKey, Home, ImportKey, Landing, MinerToolbarLayout, Navbar, MarsTokenomics,
    PageNotFound, Send, Settings, SimpleNavbar, Tx, User, WhatIsMining,
};

#[rustfmt::skip]
#[derive(Routable, Clone)]
pub enum Route {
    #[route("/")]
    Landing {},

    #[layout(SimpleNavbar)]
        #[route("/what-is-mining")]
        WhatIsMining {},
        #[route("/mars-tokenomics")]
        MarsTokenomics {},
    #[end_layout]
    //     #[route("/download")]
    //     Download {},

    #[layout(Navbar)]
        #[layout(MinerToolbarLayout)]
            #[route("/home")]
            Home {},
            #[route("/claim")]
            Claim {},
            #[route("/settings")]
            Settings {},
            #[route("/settings/export-key")]
            ExportKey {},
            #[route("/settings/import-key")]
            ImportKey {},
            #[route("/send/:to")]
            Send {
                to: String
            },
            #[route("/tx/:sig")]
            Tx {
                sig: String,
            },
            #[route("/u/:id")]
            User {
                id: String,
            },
        #[end_layout]
    #[end_layout]

    #[route("/:.._route")]
    PageNotFound { _route: Vec<String> }
}
