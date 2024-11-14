use crate::theme::Theme;
use crate::theme::THEME;
use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    let dark_mode = *THEME.read();

    rsx! {
        section {
            class: format!(
                "min-h-screen flex flex-col items-center justify-center transition-colors duration-300 px-6 {}",
                if dark_mode == Theme::Dark { "bg-gray-900 text-white" } else { "bg-white text-black" }
            ),
            div {
                class: "text-center space-y-8",
                p {
                    class: "text-lg font-semibold uppercase tracking-widest text-transparent bg-clip-text bg-gradient-to-r from-pink-500 via-purple-500 to-indigo-600 animate-pulse",
                    "Discover Your Next Destination"
                }
                h1 {
                    class: "text-5xl md:text-7xl font-extrabold leading-tight animate-fade-in",
                    "AWS Do Be Trippin'"
                },
                p {
                    class: "text-xl md:text-2xl text-gray-600 dark:text-gray-300 max-w-3xl mx-auto animate-fade-in delay-150",
                    "Plan smarter, explore deeper. Let AI guide your journey!"
                },
                div {
                    class: "flex justify-center space-x-6 animate-slide-up delay-200",
                    button {
                        class: "bg-indigo-500 text-white py-2 px-6 rounded-full shadow-lg hover:bg-indigo-600 focus:outline-none transform hover:scale-105 transition-transform duration-150 font-semibold",
                        "Get Started"
                    }
                }
            }
        }
    }
}
