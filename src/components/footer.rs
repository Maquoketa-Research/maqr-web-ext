use dioxus::prelude::*;

const LOGO_LIGHT: Asset = asset!("/assets/logo-01-light.png");

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "bg-navy text-white",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-20",
                div { class: "flex flex-col items-center text-center space-y-8",
                    img {
                        src: LOGO_LIGHT,
                        alt: "Maquoketa Research",
                        class: "h-32 w-auto"
                    }

                    div { class: "text-gray-300 space-y-3",
                        p { class: "text-lg",
                            "780 Bonnie Ln"
                            br {}
                            "Elk Grove Village, IL 60007"
                        }
                        p { class: "text-lg",
                            a {
                                href: "tel:+15631234567",
                                class: "hover:text-white transition-colors",
                                ""
                            }
                        }
                    }

                    p { class: "text-gray-400 text-base pt-6",
                        "© 2025 Maquoketa Research. All rights reserved."
                    }
                }
            }
        }
    }
}
