use crate::components::{Footer, Hero};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
        Footer {}
    }
}
