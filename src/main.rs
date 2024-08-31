#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing;
use crate::components::app::App;


pub mod components;


const _TAILWIND_URL: &str = manganis::mg!(file("assets/tailwind.css"));

fn main() {
    // Init logger
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(App);
}

