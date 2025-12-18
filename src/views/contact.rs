use crate::components::{Contact, Footer};
use dioxus::prelude::*;

/// The Contact page component
#[component]
pub fn ContactPage() -> Element {
    rsx! {
        Contact {}
        Footer {}
    }
}
