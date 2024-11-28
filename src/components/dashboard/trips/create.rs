use crate::components::dashboard::fields::input::InputField;
use crate::components::dashboard::fields::number::NumberField;
use crate::components::dashboard::fields::select::SelectField;
use crate::components::dashboard::trips::list::CachedTripsData;
use crate::components::dashboard::trips::list::CACHE_KEY;
use crate::components::spinner::Spinner;
use crate::components::spinner::SpinnerSize;
use crate::components::toast::manager::ToastManager;
use crate::components::toast::manager::ToastType;
use crate::server::trip::controller::fetch_google_places_autocomplete;
use crate::server::trip::controller::generate_detail_content;
use crate::server::trip::controller::generate_trip_outline;
use crate::server::trip::request::GenerateDetailContentRequest;
use crate::server::trip::request::GenerateTripRequest;
use crate::theme::Theme;
use crate::theme::THEME;
use chrono::Duration;
use chrono::Utc;
use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use serde::Deserialize;

#[derive(Deserialize)]
struct GooglePlacesResponse {
    predictions: Vec<Prediction>,
}

#[derive(Deserialize)]
struct Prediction {
    description: String,
    place_id: String,
}

#[component]
pub fn CreateTripPanel(user_token: Signal<String>) -> Element {
    let dark_mode = *THEME.read() == Theme::Dark;
    let title = use_signal(|| "".to_string());
    let model = use_signal(|| "anthropic.claude-3-haiku-20240307-v1:0".to_string());
    let subtopics = use_signal(|| 30);
    let details = use_signal(|| 5);
    let language = use_signal(|| "English".to_string());
    let max_length = use_signal(|| 10);
    let api_key = use_signal(|| "google_api_key".to_string());

    let title_valid = use_signal(|| true);
    let destination_valid = use_signal(|| true);
    let language_valid = use_signal(|| true);
    let mut loading = use_signal(|| false);
    let _form_error = use_signal(|| None::<String>);

    let validate_title = |title: &str| !title.is_empty();
    let validate_destination = |destination: &str| !destination.is_empty();
    let validate_language = |language: &str| !language.is_empty();

    let mut toasts_manager = use_context::<Signal<ToastManager>>();

    let mut recommended_destinations = use_signal(|| vec![]);
    let mut destination = use_signal(|| "".to_string());
    let mut selected_destination = use_signal(|| Some("Beirut, Lebanon".to_string()));

    let mut handle_destination_select = move |selected: String| {
        selected_destination.set(Some(selected.clone()));
        destination.set(selected);
        recommended_destinations.set(vec![]);
    };
    let mut fetch_recommendations = {
        move |input: String| {
            if input.is_empty() {
                recommended_destinations.set(vec![]);
                return;
            }

            spawn(async move {
                match fetch_google_places_autocomplete(input, api_key()).await {
                    Ok(response) => {
                        let suggestions: Vec<String> = response
                            .predictions
                            .iter()
                            .map(|p| p.description.clone())
                            .collect();
                        recommended_destinations.set(suggestions);
                    }
                    Err(_) => {
                        recommended_destinations.set(vec![]);
                    }
                }
            });
        }
    };

    let handle_destination_input = move |e: Event<FormData>| {
        let input = e.value();
        destination.set(input.clone());
        fetch_recommendations(input);
    };

    let handle_submit = move |e: Event<FormData>| {
        e.stop_propagation();
        loading.set(true);

        if !validate_title(&title()) || !validate_destination(&destination()) {
            toasts_manager.set(
                toasts_manager()
                    .add_toast(
                        "Error".into(),
                        "Title and subtitle are required!".into(),
                        ToastType::Error,
                        Some(Duration::seconds(5)),
                    )
                    .clone(),
            );
            return;
        }

        spawn({
            async move {
                if !user_token().is_empty() {
                    match generate_trip_outline(GenerateTripRequest {
                        title: title(),
                        token: user_token(),
                        subtitle: selected_destination().expect("destination"),
                        model: model(),
                        subtopics: subtopics(),
                        details: details(),
                        language: language(),
                        max_length: max_length(),
                    })
                    .await
                    {
                        Ok(response) => {
                            let mut cached_data = LocalStorage::get::<CachedTripsData>(CACHE_KEY)
                                .unwrap_or(CachedTripsData {
                                    data: Vec::new(),
                                    timestamp: Utc::now().timestamp(),
                                });

                            cached_data.data.push(response.data.trip);

                            let _ = LocalStorage::set(CACHE_KEY, &cached_data);
                            toasts_manager.set(
                                toasts_manager()
                                    .add_toast(
                                        "Info".into(),
                                        "Trip outline generated successfully!".into(),
                                        ToastType::Info,
                                        Some(Duration::seconds(5)),
                                    )
                                    .clone(),
                            );
                            toasts_manager.set(
                                toasts_manager()
                                    .add_toast(
                                        "Info".into(),
                                        "Generating Trip Daily Plans...".into(),
                                        ToastType::Info,
                                        Some(Duration::seconds(5)),
                                    )
                                    .clone(),
                            );
                            for detail in response.data.details {
                                match generate_detail_content(GenerateDetailContentRequest {
                                    detail_title: detail.title,
                                    trip_title: title(),
                                    language: language(),
                                    model: model(),
                                    detail_id: detail.id,
                                })
                                .await
                                {
                                    Ok(_) => {
                                        toasts_manager.set(
                                            toasts_manager()
                                                .add_toast(
                                                    "Info".into(),
                                                    "Trip generated successfully!".into(),
                                                    ToastType::Success,
                                                    Some(Duration::seconds(5)),
                                                )
                                                .clone(),
                                        );
                                        loading.set(false);
                                    }
                                    Err(e) => {
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
                            }
                        }
                        Err(e) => {
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
                }
            }
        });
    };

    rsx! {
        div {
            class: format!("flex p-4 flex-col lg:flex-row {}",
                if dark_mode { "bg-gray-800 text-white" } else { "bg-white text-gray-900" }
            ),
            div {
                h2 { class: "text-xl font-semibold mb-4", "Plan A Trip" }
                form {
                    class: "space-y-4 flex-1",
                    onsubmit: handle_submit,

                    InputField { label: "Title", value: title, is_valid: title_valid, validate: validate_title, required: true }

                    div { class: "relative",
                        div {
                            label {
                                class: format!("block text-sm font-medium {}", if dark_mode { "text-gray-300" } else { "text-gray-700" }),
                                "Destination"
                            }
                            input {
                                class: format!(
                                    "mt-1 block w-full p-2 border rounded-md shadow-sm {} {}",
                                    if dark_mode { "bg-gray-900" } else { "" },
                                    if destination_valid() { "border-gray-300" } else { "border-red-500" }
                                ),
                                value: "{destination}",
                                placeholder: "Enter a destination",
                                oninput: handle_destination_input,
                                required: true
                            }
                            if !destination_valid() {
                                p { class: "text-red-500 text-sm mt-1", "Invalid input" }
                            }
                        }
                        if !recommended_destinations().is_empty() {
                            ul { class: format!("absolute shadow-lg z-10 border rounded {}", if dark_mode { "bg-gray-900" } else { "bg-white" }),
                                for dest in recommended_destinations() {
                                    li {
                                        class: "p-2 hover:bg-gray-200 cursor-pointer",
                                        onclick: move |_| handle_destination_select(dest.clone()),
                                        "{dest}"
                                    }
                                }
                            }
                        }
                    }

                    SelectField { label: "Model", options: vec!["claude-3", "claude-3.5-sonet"], selected: model }
                    NumberField { label: "Budget ($)", value: subtopics, required: true }
                    InputField { label: "Language", value: language, is_valid: language_valid, validate: validate_language, required: true }
                    NumberField { label: "NB Days", value: max_length, required: true }

                    button {
                        class: format!("flex items-center space-x-2 bg-blue-500 text-white px-4 py-2 rounded {}", if dark_mode { "bg-blue-600" } else { "" }),
                        r#type: "submit",
                        disabled: loading(),
                        if loading() {
                            Spinner {
                                aria_label: "Loading spinner".to_string(),
                                size: SpinnerSize::Md,
                                dark_mode: true,
                            }
                            span { "Generating..." }
                        } else {
                            span { "Generate" }
                        }
                    }
                }

            }

            if let Some(destination) = selected_destination() {
                div {
                    class: "mt-4 lg:mt-0 lg:ml-8 flex-1",
                    h2 { class: "text-xl font-semibold mb-4", "Google Maps Preview" }
                    iframe {
                        src: format!("https://www.google.com/maps/embed/v1/place?key={}&q={}", api_key(), destination.replace(" ", "+")),
                        class: "w-full h-64 border-0 h-4/5",
                        allowfullscreen: "true",
                    }
                }
            }
        }
    }
}
