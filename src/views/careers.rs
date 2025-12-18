use crate::components::Footer;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn CareersPage() -> Element {
    rsx! {
        section { class: "py-16 bg-gray-50",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                div { class: "max-w-3xl",
                    h1 { class: "text-4xl md:text-5xl font-bold text-navy mb-6",
                        "Pushing the envelope"
                    }
                    p { class: "text-xl text-gray-700 leading-relaxed",
                        "Everyone at Maquoketa is 10x in one way or another. Whether you are an expert in computer vision, embedded systems, aerodynamics, manufacturing, or logistics, we like people who think from first principles and who are never satsfied with the status quo. All positions include equity compensation."
                    }
                }
            }
        }

        section { class: "py-16 bg-white",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",

                
                div { class: "mb-16",
                    h2 { class: "text-3xl font-bold text-navy mb-8 pb-3 border-b border-gray-300",
                        "Technology"
                    }
                    div { class: "space-y-3",
                        
                        Link {
                            to: Route::JobDetail { job_id: "research-engineer".to_string() },
                            class: "flex justify-between items-center py-3 hover:bg-gray-50 transition-colors group",
                            div { class: "flex items-center gap-3",
                                span { class: "text-lg text-gray-900 font-normal",
                                    "Research Engineer"
                                }
                                span { class: "text-gray-400 group-hover:text-gray-600 transition-colors",
                                    ">"
                                }
                            }
                            span { class: "text-base text-gray-600",
                                "Elk Grove Village, IL"
                            }
                        }

                        Link {
                            to: Route::JobDetail { job_id: "software-engineer-ui-ux".to_string() },
                            class: "flex justify-between items-center py-3 hover:bg-gray-50 transition-colors group",
                            div { class: "flex items-center gap-3",
                                span { class: "text-lg text-gray-900 font-normal",
                                    "UI/UX Engineer"
                                }
                                span { class: "text-gray-400 group-hover:text-gray-600 transition-colors",
                                    ">"
                                }
                            }
                            span { class: "text-base text-gray-600",
                                "Elk Grove Village, IL"
                            }
                        }

                        Link {
                            to: Route::JobDetail { job_id: "software-engineer-embedded".to_string() },
                            class: "flex justify-between items-center py-3 hover:bg-gray-50 transition-colors group",
                            div { class: "flex items-center gap-3",
                                span { class: "text-lg text-gray-900 font-normal",
                                    "Embedded Systems Engineer"
                                }
                                span { class: "text-gray-400 group-hover:text-gray-600 transition-colors",
                                    ">"
                                }
                            }
                            span { class: "text-base text-gray-600",
                                "Elk Grove Village, IL"
                            }
                        }

                        Link {
                            to: Route::JobDetail { job_id: "mechanical-engineer-motor".to_string() },
                            class: "flex justify-between items-center py-3 hover:bg-gray-50 transition-colors group",
                            div { class: "flex items-center gap-3",
                                span { class: "text-lg text-gray-900 font-normal",
                                    "Motor Technologist"
                                }
                                span { class: "text-gray-400 group-hover:text-gray-600 transition-colors",
                                    ">"
                                }
                            }
                            span { class: "text-base text-gray-600",
                                "Elk Grove Village, IL"
                            }
                        }

                        
                        Link {
                            to: Route::JobDetail { job_id: "process-engineer-manufacturing".to_string() },
                            class: "flex justify-between items-center py-3 hover:bg-gray-50 transition-colors group",
                            div { class: "flex items-center gap-3",
                                span { class: "text-lg text-gray-900 font-normal",
                                    "Process Engineer - Fabrication and Automated Manufacturing"
                                }
                                span { class: "text-gray-400 group-hover:text-gray-600 transition-colors",
                                    ">"
                                }
                            }
                            span { class: "text-base text-gray-600",
                                "Elk Grove Village, IL"
                            }
                        }
                    }
                }

                
                div { class: "mb-12",
                    h2 { class: "text-3xl font-bold text-navy mb-8 pb-3 border-b border-gray-300",
                        "Operations"
                    }
                    div { class: "space-y-3",
                        
                        Link {
                            to: Route::JobDetail { job_id: "client-relationship-manager".to_string() },
                            class: "flex justify-between items-center py-3 hover:bg-gray-50 transition-colors group",
                            div { class: "flex items-center gap-3",
                                span { class: "text-lg text-gray-900 font-normal",
                                    "Client Relationship Manager"
                                }
                                span { class: "text-gray-400 group-hover:text-gray-600 transition-colors",
                                    ">"
                                }
                            }
                            span { class: "text-base text-gray-600",
                                "Elk Grove Village, IL"
                            }
                        }

                        
                        Link {
                            to: Route::JobDetail { job_id: "compliance-officer".to_string() },
                            class: "flex justify-between items-center py-3 hover:bg-gray-50 transition-colors group",
                            div { class: "flex items-center gap-3",
                                span { class: "text-lg text-gray-900 font-normal",
                                    "Compliance Officer"
                                }
                                span { class: "text-gray-400 group-hover:text-gray-600 transition-colors",
                                    ">"
                                }
                            }
                            span { class: "text-base text-gray-600",
                                "Elk Grove Village, IL"
                            }
                        }

                        
                        Link {
                            to: Route::JobDetail { job_id: "financial-analyst".to_string() },
                            class: "flex justify-between items-center py-3 hover:bg-gray-50 transition-colors group",
                            div { class: "flex items-center gap-3",
                                span { class: "text-lg text-gray-900 font-normal",
                                    "Financial Analyst"
                                }
                                span { class: "text-gray-400 group-hover:text-gray-600 transition-colors",
                                    ">"
                                }
                            }
                            span { class: "text-base text-gray-600",
                                "Elk Grove Village, IL"
                            }
                        }
                    }
                }
            }
        }

        Footer {}
    }
}
