use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    let mut mobile_menu_open = use_signal(|| false);

    rsx! {
        nav { class: "bg-navy shadow-md sticky top-0 z-50",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                div { class: "flex justify-end items-center h-20",
                    div { class: "hidden md:flex space-x-8 items-center",
                        Link {
                            to: Route::Home {},
                            class: "text-white hover:text-gray-300 font-medium transition-colors",
                            "Home"
                        }
                        Link {
                            to: Route::ContactPage {},
                            class: "text-white hover:text-gray-300 font-medium transition-colors",
                            "Contact"
                        }
                    }

                    button {
                        class: "md:hidden text-white hover:text-gray-300",
                        onclick: move |_| mobile_menu_open.set(!mobile_menu_open()),
                        svg {
                            class: "w-6 h-6",
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M4 6h16M4 12h16M4 18h16"
                            }
                        }
                    }
                }

                if mobile_menu_open() {
                    div { class: "md:hidden pb-4",
                        div { class: "flex flex-col space-y-3",
                            Link {
                                to: Route::Home {},
                                class: "text-white hover:text-gray-300 font-medium py-2",
                                onclick: move |_| mobile_menu_open.set(false),
                                "Home"
                            }
                            Link {
                                to: Route::ContactPage {},
                                class: "text-white hover:text-gray-300 font-medium py-2",
                                onclick: move |_| mobile_menu_open.set(false),
                                "Contact"
                            }
                        }
                    }
                }
            }
        }

        Outlet::<Route> {}
    }
}
