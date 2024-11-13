pub(crate) mod grid;
pub(crate) mod item;

use crate::components::common::header::Header;
use crate::components::features::grid::Grid;
use crate::theme::Theme;
use crate::theme::THEME;
use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_brands_icons::{FaGoogle, FaRust};
use dioxus_free_icons::icons::fa_regular_icons::{FaChartBar, FaCompass, FaKeyboard, FaStar};
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
            icon: rsx! {Icon {
                width: 30,
                height: 30,
                icon: FaCompass,
            }},
            title: "Language Support",
            description: "Generate content in any languages, expanding your reach globally.",
        },
        Feature {
            icon: rsx! {Icon {
                width: 30,
                height: 30,
                icon: FaGoogle,
            }},
            title: "Powered by Google Gemini AI",
            description: "Utilize the advanced capabilities of Google Gemini models for high-quality content generation.",
        },
        Feature {
            icon: rsx! {Icon {
                width: 30,
                height: 30,
                icon: FaRust,
            }},
            title: "Built on Rust for Security",
            description: "Enjoy peace of mind with a Rust-powered frontend and backend ensuring a secure experience.",
        },
        Feature {
            icon: rsx! {Icon {
                width: 30,
                height: 30,
                icon: FaStar,
            }},
            title: "Real-Time Content Generation",
            description: "Get instant results with fast and responsive AI-powered content generation.",
        },
        Feature {
            icon: rsx! {Icon {
                width: 30,
                height: 30,
                icon: FaChartBar,
            }},
            title: "Advanced Analytics Dashboard",
            description: "Monitor and track the performance of generated content with an in-depth analytics dashboard.",
        },
        Feature {
            icon: rsx! {Icon {
                width: 30,
                height: 30,
                icon: FaKeyboard,
            }},
            title: "Developer-Friendly Platform",
            description: "Designed with developers in mind for easy customization and integration.",
        },
    ];

    rsx! {
        section {
            id: "features",
            class: format!("py-20 px-8 md:px-4 font-roboto flex min-h-screen justify-center {}",
                if dark_mode == Theme::Dark { "bg-gray-900 text-white" } else { "bg-white text-gray-900" }),

            div { class: "max-w-[1000px] mx-auto text-center",

                div { class: "relative mb-12",
                    img {
                        src: "./features.webp",
                        alt: "Tripper Icon",
                        class: "w-24 h-24 mx-auto animate-bounce"
                    }
                    Header {
                        title: "Why Tripper?",
                        subtitle: "Tripper is your secure, Rust-powered SaaS platform for effortless content creation with Google Gemini AI."
                    }
                }

                Grid { features: features }
            }
        }
    }
}
