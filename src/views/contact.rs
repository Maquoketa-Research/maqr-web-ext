use crate::components::{Contact, Footer};
use dioxus::prelude::*;

#[component]
pub fn ContactPage() -> Element {
    rsx! {
        Contact {}
        Footer {}
    }
}
