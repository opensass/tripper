use crate::components::dashboard::analytics::AnalyticsPage;
use crate::components::spinner::Spinner;
use crate::components::spinner::SpinnerSize;
use crate::router::Route;
use crate::server::trip::controller::get_trips_for_user;
use crate::server::trip::model::Trip;
use crate::server::trip::request::GetTripsForUserRequest;
use crate::theme::Theme;
use crate::theme::THEME;
use chrono::Utc;
use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CachedTripsData {
    pub data: Vec<Trip>,
    pub timestamp: i64,
}

pub const CACHE_KEY: &str = "trips_cache";
pub const CACHE_TIMEOUT: i64 = 2 * 60 * 60;

#[component]
pub fn TripsPanel(user_token: Signal<String>) -> Element {
    let dark_mode = *THEME.read() == Theme::Dark;
    let mut trips = use_signal(Vec::new);
    let mut displayed_trips = use_signal(Vec::new);
    let mut loading = use_signal(|| true);
    let mut search_query = use_signal(String::new);

    let _ = use_resource(move || async move {
        let now = Utc::now().timestamp();

        if let Ok(cached_data) = LocalStorage::get::<CachedTripsData>(CACHE_KEY) {
            if now - cached_data.timestamp < CACHE_TIMEOUT {
                loading.set(false);
                trips.set(cached_data.data.clone());
                displayed_trips.set(cached_data.data);
                return;
            }
        }

        match get_trips_for_user(GetTripsForUserRequest {
            token: user_token(),
        })
        .await
        {
            Ok(response) => {
                let cached_data = CachedTripsData {
                    data: response.data.clone(),
                    timestamp: now,
                };
                let _ = LocalStorage::set(CACHE_KEY, &cached_data);

                loading.set(false);
                trips.set(response.data.clone());
                displayed_trips.set(response.data);
            }
            Err(_) => {
                loading.set(false);
            }
        }
    });

    let mut filter_trips = move || {
        let query = search_query().to_lowercase();

        let filtered_trips = trips()
            .iter()
            .filter(|trip| {
                let matches_query = if query.is_empty() {
                    true
                } else {
                    let title_matches = trip.title.to_lowercase().contains(&query);
                    let subtitle_matches = trip
                        .subtitle
                        .as_deref()
                        .map(|s| s.to_lowercase().contains(&query))
                        .unwrap_or(false);
                    title_matches || subtitle_matches
                };

                matches_query
            })
            .cloned()
            .collect::<Vec<_>>();

        displayed_trips.set(filtered_trips);
    };

    rsx! {
        div {
            AnalyticsPage {}
            div {
                div {
                    class: "w-full md:w-1/3 pb-4 mb-4 md:mb-0 flex flex-col gap-8",

                    div {
                        h3 { class: "text-2xl font-bold mb-4", "Search" }
                        input {
                            class: format!(
                                "mt-1 block w-full p-2 border rounded-md shadow-sm {}",
                                if dark_mode { "bg-gray-900" } else { "" },
                            ),
                            placeholder: "Search by title...",
                            value: "{search_query()}",
                            oninput: move |e| {
                                search_query.set(e.value());
                                filter_trips();
                            },
                        }
                    }
                }
                h2 { class: "text-xl font-semibold mb-4", "All Trips" }
                if displayed_trips.len() > 0 {
                    div {
                        class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6",
                        for trip in displayed_trips() {
                            Link {
                                to: Route::ReadTrip { id: trip.id.to_string() },
                                class: format!(
                                    "p-4 shadow rounded-lg {}",
                                    if dark_mode { "bg-gray-700" } else { "bg-gray-100" }
                                ),
                                img {
                                    src: trip.cover.as_deref().unwrap_or("/path/to/default-cover.jpg"),
                                    alt: "Trip cover",
                                    class: "w-full h-48 object-cover rounded-md mb-4"
                                }
                                p {
                                    class: "text-sm text-gray-500 mb-2",
                                    "{trip.created_at.format(\"%B %d, %Y\")} · {trip.title.len() / 7000} min read"
                                }
                                p {
                                    class: format!(
                                        "text-sm {}",
                                        if trip.completed { "text-green-600" } else { "text-red-600" }
                                    ),
                                    if trip.completed { "Completed" } else { "In Progress" }
                                }
                                p {
                                    class: "mt-2 text-sm text-gray-700",
                                    "{trip.title.chars().take(30).collect::<String>()}..."
                                }
                            }
                        }
                    }
                } else {
                    p {
                        class: "flex items-center space-x-2 px-4 py-2 rounded",
                        if loading() {
                            Spinner {
                                aria_label: "Loading spinner".to_string(),
                                size: SpinnerSize::Md,
                                dark_mode: true,
                            }
                            span { "Loading trips..." }
                        } else {
                            span { "No trips match your search filter." }
                        }
                    }
                }
            }
        }
    }
}
