pub(crate) mod grid;
pub(crate) mod item;

use crate::components::common::header::Header;
use crate::components::features::grid::Grid;
use crate::theme::Theme;
use crate::theme::THEME;
use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_regular_icons::{
    FaCalendarCheck, FaChartBar, FaClock, FaCompass, FaHeart, FaLightbulb,
};
use dioxus_free_icons::Icon;

#[derive(Props, Clone, PartialEq)]
struct Feature {
    icon: Element,
    title: &'static str,
    description: &'static str,
}

#[component]
pub fn Features() -> Element {
    let dark_mode = *THEME.read();

    let features = vec![
        Feature {
            icon: rsx! { Icon { width: 30, height: 30, icon: FaCompass, class: "text-indigo-500 group-hover:animate-bounce" } },
            title: "Global Destinations",
            description: "Explore limitless destinations with smart recommendations from Amazon Bedrock AI.",
        },
        Feature {
            icon: rsx! { Icon { width: 30, height: 30, icon: FaCalendarCheck, class: "text-green-500 group-hover:animate-spin" } },
            title: "Flexible Itineraries",
            description: "Plan days with ease; adjust, customize, and optimize itineraries effortlessly.",
        },
        Feature {
            icon: rsx! { Icon { width: 30, height: 30, icon: FaHeart, class: "text-red-500 group-hover:animate-pulse" } },
            title: "Tailored Experiences",
            description: "Get suggestions that fit your style and preferences, made possible by Bedrock AI.",
        },
        Feature {
            icon: rsx! { Icon { width: 30, height: 30, icon: FaChartBar, class: "text-blue-500 group-hover:animate-ping" } },
            title: "Analytics Insights",
            description: "Gain insights with Tripper's tracking and planning analytics dashboard.",
        },
        Feature {
            icon: rsx! { Icon { width: 30, height: 30, icon: FaClock, class: "text-yellow-500 group-hover:animate-spin" } },
            title: "Real-Time Updates",
            description: "Stay updated with live itinerary changes and real-time notifications.",
        },
        Feature {
            icon: rsx! { Icon { width: 30, height: 30, icon: FaLightbulb, class: "text-purple-500 group-hover:animate-bounce" } },
            title: "Easy Integration",
            description: "Developers enjoy a Rust-powered backend designed for flexibility and customization.",
        },
    ];

    rsx! {
        section {
            id: "features",
            class: format!("py-20 px-8 md:px-4 font-roboto flex min-h-screen justify-center transition-colors duration-300 {}",
                if dark_mode == Theme::Dark { "bg-gray-900 text-white" } else { "bg-gray-100 text-gray-900" }),

            div { class: "max-w-5xl mx-auto text-center space-y-12",

                div { class: "relative mb-12 space-y-6",
                    img {
                        src: "./features-icon.webp",
                        alt: "Tripper Icon",
                        class: "w-24 h-24 mx-auto animate-spin-slow hover:animate-spin"
                    }
                    Header {
                        title: "Why Tripper?",
                        subtitle: "Built on Rust and powered by Amazon Bedrock, Tripper brings intelligent, seamless travel planning to you."
                    }
                }

                Grid { features: features }
            }
        }
    }
}
