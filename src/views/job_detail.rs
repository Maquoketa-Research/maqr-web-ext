use crate::components::Footer;
use dioxus::prelude::*;

#[component]
pub fn JobDetail(job_id: String) -> Element {
    let job_info = get_job_info(&job_id);

    rsx! {
        section { class: "py-16 bg-gray-50",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                h1 { class: "text-4xl md:text-5xl font-bold text-gray-900 mb-4",
                    "{job_info.title}"
                }
                p { class: "text-lg text-gray-600",
                    "{job_info.location}"
                }
            }
        }

        section { class: "py-16 bg-white",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                // Responsibilities Section
                if !job_info.responsibilities.is_empty() {
                    div { class: "mb-12",
                        h2 { class: "text-2xl font-bold text-navy mb-6",
                            "Responsibilities"
                        }
                        ul { class: "list-disc list-inside space-y-2 text-gray-700",
                            for responsibility in &job_info.responsibilities {
                                li { "{responsibility}" }
                            }
                        }
                    }
                }

                // Skills Section
                if !job_info.skills.is_empty() {
                    div { class: "mb-12",
                        h2 { class: "text-2xl font-bold text-navy mb-6",
                            "Skills, Qualifications, and Experience"
                        }
                        ul { class: "list-disc list-inside space-y-2 text-gray-700",
                            for skill in &job_info.skills {
                                li { "{skill}" }
                            }
                        }
                    }
                }

                // Compensation Section
                if let Some(compensation) = &job_info.compensation {
                    div { class: "mb-12 pb-8 border-b border-gray-300",
                        p { class: "text-gray-700 leading-relaxed mb-4",
                            "{compensation}"
                        }
                        a {
                            href: "#",
                            class: "text-navy hover:underline font-medium",
                            "Explore Benefits >"
                        }
                    }
                }

                // Footer Text
                div { class: "mb-8",
                    p { class: "text-gray-700 italic mb-2",
                        "No telephone inquiries, please."
                    }
                    p { class: "text-gray-700",
                        "Maquoketa Research is an equal opportunity employer."
                    }
                }

                // Apply Button
                button {
                    class: "bg-navy hover:bg-navy/90 text-white font-semibold py-3 px-8 transition-colors duration-200",
                    "Apply Now"
                }
            }
        }

        Footer {}
    }
}

struct JobInfo {
    title: String,
    location: String,
    responsibilities: Vec<String>,
    skills: Vec<String>,
    compensation: Option<String>,
}

fn get_job_info(job_id: &str) -> JobInfo {
    match job_id {
        "research-engineer" => JobInfo {
            title: "Research Engineer".to_string(),
            location: "Elk Grove Village, IL".to_string(),
            responsibilities: vec![
                "Design and develop advanced UAV control systems and navigation algorithms".to_string(),
                "Research and implement computer vision systems for autonomous flight".to_string(),
                "Collaborate with mechanical and electrical teams on system integration".to_string(),
            ],
            skills: vec![
                "A degree in computer science, engineering, mathematics, physics, or a related discipline".to_string(),
                "Strong programming skills; experience in Python or Rust is a plus".to_string(),
                "Knowledge of modern machine learning frameworks; experience with PyTorch or ONNX is a plus".to_string(), 
                "Strong understanding of GPU/NPU/TPU architecture".to_string(),
                "Strong understanding of linear algebra, calculus, and machine learning algorithms".to_string(),
                "Experience with 'edge inference' in resource-constrained environments is a plus".to_string()
            ],
            compensation: Some("The base salary for this position ranges from $120,000 to $180,000. In addition to base salary, total compensation includes equity, discretionary bonus, and comprehensive benefits.".to_string()),
        },
        "software-engineer-ui-ux" => JobInfo {
            title: "UI/UX Engineer".to_string(),
            location: "Elk Grove Village, IL".to_string(),
            responsibilities: vec![
                "Design and build high-performance intuitive user interfaces for UAV ground control systems".to_string(),
                "Develop real-time data visualization tools for flight telemetry".to_string(),
                "Work closely with pilots and operators to understand user needs".to_string(),
            ],
            skills: vec![
                "A degree in computer science, design, or a related field".to_string(),
                "Strong programming skills; experience in Rust is a plus".to_string(),
                "Knowledge of modern web frameworks and technologies; Dioxus is a plus".to_string(),
            ],
            compensation: Some("The base salary for this position ranges from $110,000 to $160,000. In addition to base salary, total compensation includes equity, discretionary bonus, and comprehensive benefits.".to_string()),
        },
        "software-engineer-embedded" => JobInfo {
            title: "Embedded Systems Engineer".to_string(),
            location: "Elk Grove Village, IL".to_string(),
            responsibilities: vec![
                "Develop low-level firmware for flight control systems".to_string(),
                "Implement real-time operating systems for embedded processors".to_string(),
                "Design and implement communication protocols between UAV subsystems".to_string(),
            ],
            skills: vec![
                "A degree in computer science, electrical engineering, or related field".to_string(),
                "Strong proficiency in embedded systems programming; Rust is a plus".to_string(),
                "Understanding of hardware interfaces (I2C, SPI, UART, CAN)".to_string(),
                "Experience in PCB design and using PCB design tools (e.g. KiCad)".to_string(),
            ],
            compensation: Some("The base salary for this position ranges from $115,000 to $170,000. In addition to base salary, total compensation includes equity, discretionary bonus, and comprehensive benefits.".to_string()),
        },
        "mechanical-engineer-motor" => JobInfo {
            title: "Motor Technologist".to_string(),
            location: "Elk Grove Village, IL".to_string(),
            responsibilities: vec![
                "Design and optimize electric propulsion systems for UAV platforms".to_string(),
                "Develop motor control algorithms for maximum efficiency".to_string(),
                "Work with suppliers to source components".to_string(),
            ],
            skills: vec![
                "A degree in mechanical engineering, electrical engineering, or related field".to_string(),
                "Strong understanding of electric motor design and control".to_string(),
                "Experience with CAD software (SolidWorks, Fusion360, etc.); SiemensNX is a plus".to_string(),
                "Knowledge of power electronics and motor drive systems".to_string(),
            ],
            compensation: Some("The base salary for this position ranges from $105,000 to $155,000. In addition to base salary, total compensation includes equity, discretionary bonus, and comprehensive benefits.".to_string()),
        },
        "process-engineer-manufacturing" => JobInfo {
            title: "Process Engineer - Fabrication and Automated Manufacturing".to_string(),
            location: "Elk Grove Village, IL".to_string(),
            responsibilities: vec![
                "Design and optimize automated manufacturing processes for UAV components".to_string(),
                "Develop quality control systems and inspection procedures".to_string(),
                "Implement lean manufacturing principles to reduce waste and improve efficiency".to_string(),
                "Collaborate with design engineers to ensure manufacturability".to_string(),
            ],
            skills: vec![
                "A degree in mechanical engineering, industrial engineering, or related field".to_string(),
                "Experience with CNC machining, 3D printing, and composite fabrication".to_string(),
                "Experience with assembly line design is a plus".to_string(),
                "Experience with SiemensNX is a plus".to_string(),
                "Strong problem-solving and analytical skills".to_string(),
            ],
            compensation: Some("The base salary for this position ranges from $100,000 to $145,000. In addition to base salary, total compensation includes equity, discretionary bonus, and comprehensive benefits.".to_string()),
        },
        "client-relationship-manager" => JobInfo {
            title: "Client Relationship Manager".to_string(),
            location: "Elk Grove Village, IL".to_string(),
            responsibilities: vec![
                "Build and maintain relationships with key clients and partners".to_string(),
                "Understand client requirements and coordinate with technical teams".to_string(),
                "Manage contract negotiations and project timelines".to_string(),
                "Identify new business opportunities and expansion strategies".to_string(),
            ],
            skills: vec![
                "A degree in business, engineering, or related field".to_string(),
                "Experience in client-facing roles, preferably in aerospace or defense".to_string(),
                "Strong communication and interpersonal skills".to_string(),
                "Understanding of UAV technology and applications".to_string(),
                "Proven track record of managing complex client relationships".to_string(),
            ],
            compensation: Some("The base salary for this position ranges from $95,000 to $140,000. In addition to base salary, total compensation includes equity, discretionary bonus, and comprehensive benefits.".to_string()),
        },
        "compliance-officer" => JobInfo {
            title: "Compliance Officer".to_string(),
            location: "Elk Grove Village, IL".to_string(),
            responsibilities: vec![
                "Ensure compliance with NDAA regulations".to_string(),
                "Develop and implement compliance policies and procedures".to_string(),
                "Conduct internal audits and risk assessments".to_string(),
            ],
            skills: vec![
                "A degree in law, business, or related field".to_string(),
                "Deep understanding of defense industry regulations and compliance requirements".to_string(),
                "Strong attention to detail and analytical skills".to_string(),
                "Excellent written and verbal communication abilities".to_string(),
            ],
            compensation: Some("The base salary for this position ranges from $90,000 to $130,000. In addition to base salary, total compensation includes equity, discretionary bonus, and comprehensive benefits.".to_string()),
        },
        "financial-analyst" => JobInfo {
            title: "Financial Analyst".to_string(),
            location: "Elk Grove Village, IL".to_string(),
            responsibilities: vec![
                "Conduct financial analysis and modeling to support strategic decisions".to_string(),
                "Prepare budgets, forecasts, and financial reports".to_string(),
                "Analyze cost structures and identify opportunities for optimization".to_string(),
                "Support fundraising activities and investor relations".to_string(),
            ],
            skills: vec![
                "A degree in finance, economics, data science, computer science, or related field".to_string(),
                "Strong proficiency in financial modeling using software such as Excel".to_string(),
                "Understanding of venture capital and startup financing".to_string(),
                "Excellent analytical and problem-solving skills".to_string(),
                "Experience with Python/Pandas is considered a plus".to_string(),
            ],
            compensation: Some("The base salary for this position ranges from $85,000 to $125,000. In addition to base salary, total compensation includes equity, discretionary bonus, and comprehensive benefits.".to_string()),
        },
        _ => JobInfo {
            title: "Position Not Found".to_string(),
            location: "".to_string(),
            responsibilities: vec![],
            skills: vec![],
            compensation: None,
        },
    }
}
