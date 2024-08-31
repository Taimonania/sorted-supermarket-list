use dioxus::prelude::*;
use super::home::Home;

pub fn App() -> Element {
    rsx! {
        Home {}
    }
}

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    Home {},
}