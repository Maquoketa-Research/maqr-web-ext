use crate::Route;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct ContactFormData {
    name: String,
    company: String,
    email: String,
    phone: String,
    reason: String,
    message: String,
}

#[component]
pub fn Contact() -> Element {
    let mut status = use_signal(|| String::new());
    let mut is_submitting = use_signal(|| false);

    let handle_submit = move |evt: Event<FormData>| async move {
        is_submitting.set(true);
        status.set(String::new());

        let values = evt.values();

        let get_value = |key: &str| -> String {
            values
                .iter()
                .find(|(k, _)| k == key)
                .map(|(_, v)| match v {
                    dioxus::prelude::FormValue::Text(s) => s.clone(),
                    _ => String::new(),
                })
                .unwrap_or_default()
        };

        let form_data = ContactFormData {
            name: get_value("name"),
            company: get_value("company"),
            email: get_value("email"),
            phone: get_value("phone"),
            reason: get_value("reason"),
            message: get_value("message"),
        };

        match send_contact_email(form_data).await {
            Ok(_) => {
                status.set("Thank you! Your message has been sent successfully.".to_string());
            }
            Err(e) => {
                status.set(format!("Error sending message: {}. Please try again.", e));
            }
        }

        is_submitting.set(false);
    };

    rsx! {
        section { class: "py-16 bg-white",
            div { class: "max-w-4xl mx-auto px-4 sm:px-6 lg:px-8",
                // Introductory text
                div { class: "mb-8 space-y-4",
                    p { class: "text-base text-gray-700",
                        "For career-related inquiries, please visit our "
                        Link {
                            to: Route::CareersPage {},
                            class: "text-navy underline hover:no-underline",
                            "Careers"
                        }
                        " page."
                    }
                    p { class: "text-base text-gray-700",
                        "For all other inquiries, please submit the form below."
                    }
                }

                // Status message
                if !status().is_empty() {
                    div {
                        class: if status().contains("Error") {
                            "mb-6 p-4 bg-red-50 border border-red-200 text-red-800 rounded"
                        } else {
                            "mb-6 p-4 bg-green-50 border border-green-200 text-green-800 rounded"
                        },
                        "{status}"
                    }
                }

                // Contact Form
                form {
                    class: "space-y-6",
                    onsubmit: handle_submit,

                    // Name field
                    div {
                        label {
                            r#for: "name",
                            class: "block text-sm font-medium text-gray-900 mb-2",
                            "Name "
                            span { class: "text-navy", "*" }
                        }
                        input {
                            r#type: "text",
                            id: "name",
                            name: "name",
                            required: true,
                            class: "w-full px-4 py-2.5 border border-gray-300 focus:ring-2 focus:ring-navy focus:border-navy outline-none transition"
                        }
                    }

                    // Company field
                    div {
                        label {
                            r#for: "company",
                            class: "block text-sm font-medium text-gray-900 mb-2",
                            "Company"
                        }
                        input {
                            r#type: "text",
                            id: "company",
                            name: "company",
                            class: "w-full px-4 py-2.5 border border-gray-300 focus:ring-2 focus:ring-navy focus:border-navy outline-none transition"
                        }
                    }

                    // Email field
                    div {
                        label {
                            r#for: "email",
                            class: "block text-sm font-medium text-gray-900 mb-2",
                            "Email "
                            span { class: "text-navy", "*" }
                        }
                        input {
                            r#type: "email",
                            id: "email",
                            name: "email",
                            required: true,
                            class: "w-full px-4 py-2.5 border border-gray-300 focus:ring-2 focus:ring-navy focus:border-navy outline-none transition"
                        }
                    }

                    // Phone field
                    div {
                        label {
                            r#for: "phone",
                            class: "block text-sm font-medium text-gray-900 mb-2",
                            "Phone"
                        }
                        input {
                            r#type: "tel",
                            id: "phone",
                            name: "phone",
                            class: "w-full px-4 py-2.5 border border-gray-300 focus:ring-2 focus:ring-navy focus:border-navy outline-none transition"
                        }
                    }

                    // Reason for Contact dropdown
                    div {
                        label {
                            r#for: "reason",
                            class: "block text-sm font-medium text-gray-900 mb-2",
                            "Reason for Contact "
                            span { class: "text-navy", "*" }
                        }
                        select {
                            id: "reason",
                            name: "reason",
                            required: true,
                            class: "w-full px-4 py-2.5 border border-gray-300 focus:ring-2 focus:ring-navy focus:border-navy outline-none transition bg-white",
                            option { value: "", "Select a reason..." }
                            option { value: "general", "General Inquiry" }
                            option { value: "partnership", "Partnership Opportunity" }
                            option { value: "technical", "Technical Support" }
                            option { value: "sales", "Sales Inquiry" }
                            option { value: "other", "Other" }
                        }
                    }

                    // Message field
                    div {
                        label {
                            r#for: "message",
                            class: "block text-sm font-medium text-gray-900 mb-2",
                            "Message "
                            span { class: "text-navy", "*" }
                        }
                        textarea {
                            id: "message",
                            name: "message",
                            rows: "6",
                            required: true,
                            class: "w-full px-4 py-2.5 border border-gray-300 focus:ring-2 focus:ring-navy focus:border-navy outline-none transition resize-none"
                        }
                    }

                    // Submit button
                    button {
                        r#type: "submit",
                        disabled: is_submitting(),
                        class: if is_submitting() {
                            "bg-gray-400 text-white font-semibold py-3 px-8 cursor-not-allowed"
                        } else {
                            "bg-navy hover:bg-navy/90 text-white font-semibold py-3 px-8 transition-colors duration-200"
                        },
                        style: "font-family: Palatino, 'Palatino Linotype', 'Book Antiqua', 'Century Schoolbook', Georgia, serif;",
                        if is_submitting() {
                            "Sending..."
                        } else {
                            "Submit"
                        }
                    }
                }
            }
        }
    }
}

#[post("/api/contact")]
async fn send_contact_email(form_data: ContactFormData) -> Result<String> {
    #[cfg(feature = "server")]
    {
        use lettre::message::header::ContentType;
        use lettre::transport::smtp::authentication::Credentials;
        use lettre::{Message, SmtpTransport, Transport};

        let reason_display = match form_data.reason.as_str() {
            "general" => "General Inquiry",
            "partnership" => "Partnership Opportunity",
            "technical" => "Technical Support",
            "sales" => "Sales Inquiry",
            "other" => "Other",
            _ => &form_data.reason,
        };

        let email_body = format!(
            "New contact form submission from Maquoketa Research website:\n\n\
             Name: {}\n\
             Company: {}\n\
             Email: {}\n\
             Phone: {}\n\
             Reason: {}\n\n\
             Message:\n{}",
            form_data.name,
            if form_data.company.is_empty() { "Not provided" } else { &form_data.company },
            form_data.email,
            if form_data.phone.is_empty() { "Not provided" } else { &form_data.phone },
            reason_display,
            form_data.message
        );

        // Get SMTP credentials from environment variables
        let smtp_username = std::env::var("SMTP_USERNAME")
            .unwrap_or_else(|_| "your-email@example.com".to_string());
        let smtp_password = std::env::var("SMTP_PASSWORD")
            .unwrap_or_else(|_| "your-password".to_string());
        let smtp_server = std::env::var("SMTP_SERVER")
            .unwrap_or_else(|_| "smtp.gmail.com".to_string());

        let email = Message::builder()
            .from(smtp_username.parse().map_err(|e| {
                dioxus::prelude::ServerFnError::new(format!("Invalid from address: {}", e))
            })?)
            .to("evan@maquoketa.net".parse().map_err(|e| {
                dioxus::prelude::ServerFnError::new(format!("Invalid to address: {}", e))
            })?)
            .subject(format!("Contact Form: {} - {}", form_data.name, reason_display))
            .header(ContentType::TEXT_PLAIN)
            .body(email_body)
            .map_err(|e| dioxus::prelude::ServerFnError::new(format!("Failed to build email: {}", e)))?;

        let creds = Credentials::new(smtp_username, smtp_password);

        let mailer = SmtpTransport::relay(&smtp_server)
            .map_err(|e| dioxus::prelude::ServerFnError::new(format!("Failed to connect to SMTP server: {}", e)))?
            .credentials(creds)
            .build();

        mailer.send(&email)
            .map_err(|e| dioxus::prelude::ServerFnError::new(format!("Failed to send email: {}", e)))?;

        Ok("Email sent successfully".to_string())
    }

    #[cfg(not(feature = "server"))]
    {
        // This should never be called on the client side
        Ok("Email functionality only available on server".to_string())
    }
}
