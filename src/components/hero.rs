use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
        section { class: "bg-white text-gray-900",
            div { class: "max-w-7xl mx-auto pl-4 pr-16 sm:pl-6 sm:pr-32 lg:pl-8 lg:pr-48 py-24 md:py-32",
                div { class: "max-w-3xl",
                    h1 { class: "text-4xl md:text-5xl lg:text-6xl font-bold mb-6 leading-tight text-navy",
                        "Maquoketa Research is a vertically integrated designer, developer, and manufacturer of unmanned aerial systems. We are based in the Chicago Area."
                    }
                }
            }
        }
    }
}
