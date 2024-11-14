// use crate::components::common::server::JWT_TOKEN;
use crate::components::spinner::Spinner;
use crate::components::spinner::SpinnerSize;
use crate::components::toast::manager::ToastManager;
use crate::components::toast::manager::ToastType;
use crate::router::Route;
use crate::server::auth::controller::{about_me, register_user};
use crate::server::auth::response::RegisterUserSchema;
use crate::theme::Theme;
use crate::theme::THEME;
use chrono::Duration;
use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_regular_icons::{FaEye, FaEyeSlash};
use dioxus_free_icons::Icon;
use gloo_storage::SessionStorage;
use gloo_storage::Storage;
use regex::Regex;

#[component]
pub fn Register() -> Element {
    let navigator = use_navigator();
    let dark_mode = *THEME.read();

    let mut name = use_signal(|| "".to_string());
    let mut email = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());

    let mut error_message = use_signal(|| None::<String>);
    let mut email_valid = use_signal(|| true);
    let mut name_valid = use_signal(|| true);
    let mut password_valid = use_signal(|| true);
    let mut show_password = use_signal(|| false);

    let mut loading = use_signal(|| false);

    let mut toasts_manager = use_context::<Signal<ToastManager>>();

    let validate_email = |email: &str| {
        let pattern = Regex::new(r"^[^ ]+@[^ ]+\.[a-z]{2,3}$").unwrap();
        pattern.is_match(email)
    };

    let validate_password = |password: &str| !password.is_empty();
    let validate_name = |name: &str| !name.is_empty();

    use_effect(move || {
        spawn(async move {
            let token: String = SessionStorage::get("jwt").unwrap_or_default();
            if !token.is_empty() {
                match about_me(token.clone()).await {
                    Ok(data) => {
                        let _user = data.data.user;
                        navigator.push("/dashboard");
                    }
                    Err(e) => {
                        error_message.set(Some(e.to_string()));
                    }
                }
            }
        });
    });

    let handle_register = move |_| {
        let name = name().clone();
        let email = email().clone();
        let password = password().clone();
        loading.set(true);

        if !validate_email(&email) || password.is_empty() {
            error_message.set(Some(
                "Please provide a valid email and password.".to_string(),
            ));
            toasts_manager.set(
                toasts_manager()
                    .add_toast(
                        "Error".into(),
                        "Please provide a valid email and password!".into(),
                        ToastType::Error,
                        Some(Duration::seconds(5)),
                    )
                    .clone(),
            );
            loading.set(false);
            return;
        }

        spawn(async move {
            match register_user(RegisterUserSchema {
                name,
                email,
                password,
            })
            .await
            {
                Ok(_) => {
                    toasts_manager.set(
                        toasts_manager()
                            .add_toast(
                                "Success".into(),
                                "Now, you can log in!".into(),
                                ToastType::Success,
                                Some(Duration::seconds(5)),
                            )
                            .clone(),
                    );
                    navigator.push("/login");
                    loading.set(false);
                }
                Err(e) => {
                    // error_message.set(Some(e.to_string()));
                    let msg = e.to_string();
                    let error_message = msg
                        .splitn(2, "error running server function:")
                        .nth(1)
                        .unwrap_or("")
                        .trim();
                    toasts_manager.set(
                        toasts_manager()
                            .add_toast(
                                "Error".into(),
                                error_message.into(),
                                ToastType::Error,
                                Some(Duration::seconds(5)),
                            )
                            .clone(),
                    );
                    loading.set(false);
                }
            }
        });
    };

    rsx! {
        div {
            class: format!("min-h-screen flex items-center justify-center {}",
                if dark_mode == Theme::Dark { "bg-blue-500 text-white" } else { "bg-blue-900 text-gray-900" }
            ),
            style: "background-image: linear-gradient(90deg, rgba(0,0,0,0.05) 1px, transparent 1px), linear-gradient(rgba(0,0,0,0.05) 1px, transparent 1px); background-size: 40px 40px;",
            form {
                style: if dark_mode == Theme::Dark { "background-color: #1f2937; color: white;" } else { "background-color: white; color: black;" },
                class: "w-full max-w-md flex flex-col items-center p-6 bg-white shadow-lg rounded-lg transform transition-all duration-300 hover:shadow-2xl",
                onsubmit: handle_register,
                Link {
                    to: Route::Home {},
                    class: "text-gray-400 text-sm mb-4",
                    "← Back to Home"
                }
                h1 { class: "text-3xl font-semibold mb-6 mt-4", "Sign Up" },
                div { class: "flex flex-col md:flex-row gap-4 w-full mb-6",
                    div { class: "flex flex-col items-start w-full",
                        span { class: "text-xs text-gray-500 mb-1", "Coming Soon" },
                        button {
                            class: "w-full py-2 border rounded-md border-gray-300 bg-gray-100 text-gray-400 cursor-not-allowed whitespace-nowrap",
                            disabled: "true",
                            "Register with Google"
                        }
                    }
                    div { class: "flex flex-col items-start w-full",
                        span { class: "text-xs text-gray-500 mb-1", "Coming Soon" },
                        button {
                            class: "w-full py-2 border rounded-md border-gray-300 bg-gray-100 text-gray-400 cursor-not-allowed whitespace-nowrap",
                            disabled: "true",
                            "Register with Github"
                        }
                    }
                }

                div { class: "text-center text-gray-500 mb-6", "or" }

                div { class: "relative mb-4 w-full",
                    input {
                        class: format!(
                            "w-full p-3 border rounded-md shadow-sm transition-all {} {}",
                            if dark_mode == Theme::Dark { "bg-gray-700 text-white" } else { "bg-white text-gray-900" },
                            if name_valid() { "border-gray-300" } else { "border-red-500" }
                        ),
                        r#type: "text",
                        placeholder: "Full Name",
                        value: "{name}",
                        required: true,
                        oninput: move |e| {
                            let value = e.value().clone();
                            name.set(value.clone());
                            name_valid.set(validate_name(&value));
                        }
                    },
                    if !name_valid() {
                        p { class: "text-red-500 text-sm mt-1", "Name can't be blank" }
                    }
                }

                div { class: "relative mb-4 w-full",
                    input {
                        class: format!(
                            "w-full p-3 border rounded-md shadow-sm transition-all {} {}",
                            if dark_mode == Theme::Dark { "bg-gray-700 text-white" } else { "bg-white text-gray-900" },
                            if email_valid() { "border-gray-300" } else { "border-red-500" }
                        ),
                        r#type: "text",
                        placeholder: "Email",
                        value: "{email}",
                        required: true,
                        oninput: move |e| {
                            let value = e.value().clone();
                            email.set(value.clone());
                            email_valid.set(validate_email(&value));
                        }
                    },
                    if !email_valid() {
                        p { class: "text-red-500 text-sm mt-1", "Enter a valid email address" }
                    }
                }

                div { class: "relative mb-4 w-full",
                    input {
                        class: format!(
                            "w-full p-3 border rounded-md shadow-sm transition-all {} {}",
                            if dark_mode == Theme::Dark { "bg-gray-700 text-white" } else { "bg-white text-gray-900" },
                            if password_valid() { "border-gray-300" } else { "border-red-500" }
                        ),
                        r#type: if show_password() { "text" } else { "password" },
                        placeholder: "Password",
                        value: "{password}",
                        required: true,
                        oninput: move |e| {
                            let value = e.value().clone();
                            password.set(value.clone());
                            password_valid.set(validate_password(&value));
                        }
                    },
                    button {
                        onclick: move |_| show_password.set(!show_password()),
                        class: "absolute inset-y-0 right-0 pr-3 text-gray-500 hover:text-gray-700",
                        if show_password() {
                            Icon { icon: FaEye, width: 20, height: 20 }
                        } else {
                            Icon { icon: FaEyeSlash, width: 20, height: 20 }
                        }
                    },
                    if !password_valid() {
                        p { class: "text-red-500 text-sm mt-1", "Password can't be blank" }
                    }
                }

                button {
                    class: "flex items-center justify-center space-x-2 w-full py-2 mt-4 bg-blue-600 hover:bg-blue-700 text-white rounded-md transition-all",
                    r#type: "submit",
                    disabled: loading(),
                    if loading() {
                        Spinner {
                            aria_label: "Loading spinner".to_string(),
                            size: SpinnerSize::Md,
                            dark_mode: dark_mode == Theme::Dark,
                        }
                        span { "Signing Up..." }
                    } else {
                        span { "Sign Up" }
                    }
                }

                div {
                    class: "text-gray-500 mt-6",
                    "Already have an account? ",
                    a {
                        href: "#",
                        class: "text-blue-500 font-semibold hover:underline",
                        "Sign in"
                    }
                }
            }
        }
    }
}
