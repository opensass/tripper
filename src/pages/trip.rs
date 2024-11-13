#![allow(unused)]

use crate::components::dashboard::trips::create::CreateTripPanel;
use crate::components::dashboard::trips::list::TripsPanel;
use crate::components::dashboard::trips::read::ReadTripPanel;
use crate::components::dashboard::chat::ChatPanelPage;
use crate::components::dashboard::navbar::Navbar;
use crate::components::dashboard::profile::EditProfilePanel;
use crate::components::dashboard::sidebar::Sidebar;
use crate::components::dashboard::sidebar::Tab;
use crate::server::auth::controller::about_me;
use crate::theme::Theme;
use crate::theme::THEME;
use bson::oid::ObjectId;
use dioxus::prelude::*;
use gloo_storage::SessionStorage;
use gloo_storage::Storage;

#[component]
pub fn ReadTrip(id: String) -> Element {
    let active_tab = use_signal(|| Tab::ReadTrip);
    let dark_mode = *THEME.read() == Theme::Dark;
    let mut user_token = use_signal(|| "".to_string());
    let navigator = use_navigator();
    let mut current_tab = rsx! { TripsPanel { user_token } };
    if id.is_empty() {
        current_tab = match active_tab() {
            Tab::Trips => rsx! { TripsPanel { user_token } },
            Tab::CreateTrip => rsx! { CreateTripPanel { user_token } },
            Tab::ReadTrip => rsx! { ReadTripPanel { trip_id: id } },
            Tab::EditProfile => rsx! { EditProfilePanel {} },
            Tab::Chat => rsx! { ChatPanelPage { user_token, trip_id: id} },
        };
    } else {
        current_tab = rsx! { ReadTripPanel { trip_id: id } };
    }

    use_effect(move || {
        spawn(async move {
            let token: String = SessionStorage::get("jwt").unwrap_or_default();
            if token.is_empty() {
                navigator.push("/login");
            } else {
                match about_me(token.clone()).await {
                    Ok(data) => {
                        let _user = data.data.user;
                        user_token.set(token.clone());
                    }
                    Err(_) => {
                        navigator.push("/login");
                    }
                }
            }
        });
    });

    rsx! {
        div { class: format!("min-h-screen flex {}", if dark_mode { "bg-gray-900 text-white" } else { "bg-white text-gray-900" }),
            Sidebar { navigate: true, active_tab: active_tab.clone() }

            div { class: "flex-1 p-4 md:p-8",
                Navbar { dark_mode }

                div { class: format!("p-4 shadow rounded-lg {}", if dark_mode { "bg-gray-800" } else { "bg-white" }),
                    {current_tab}
                }
            }
        }
    }
}

#[component]
pub fn EditTrip(id: String) -> Element {
    let active_tab = use_signal(|| Tab::ReadTrip);
    let dark_mode = *THEME.read() == Theme::Dark;
    let mut user_token = use_signal(|| "".to_string());
    let navigator = use_navigator();
    let mut current_tab = rsx! { TripsPanel { user_token } };
    if id.is_empty() {
        current_tab = match active_tab() {
            Tab::Trips => rsx! { TripsPanel { user_token } },
            Tab::ReadTrip => rsx! { ReadTripPanel { trip_id: id } },
            Tab::EditProfile => rsx! { EditProfilePanel {} },
            Tab::Chat => rsx! { ChatPanelPage { user_token, trip_id: id} },
            Tab::CreateTrip => todo!(),
        };
    }

    use_effect(move || {
        spawn(async move {
            let token: String = SessionStorage::get("jwt").unwrap_or_default();
            if token.is_empty() {
                navigator.push("/login");
            } else {
                match about_me(token.clone()).await {
                    Ok(data) => {
                        let _user = data.data.user;
                        user_token.set(token.clone());
                    }
                    Err(_) => {
                        navigator.push("/login");
                    }
                }
            }
        });
    });

    rsx! {
        div { class: format!("min-h-screen flex {}", if dark_mode { "bg-gray-900 text-white" } else { "bg-white text-gray-900" }),
            Sidebar { navigate: true, active_tab: active_tab.clone() }

            div { class: "flex-1 p-4 md:p-8",
                Navbar { dark_mode }

                div { class: format!("p-4 shadow rounded-lg {}", if dark_mode { "bg-gray-800" } else { "bg-white" }),
                    {current_tab}
                }
            }
        }
    }
}
