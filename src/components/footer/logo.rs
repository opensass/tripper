use dioxus::prelude::*;

#[component]
pub fn Logo() -> Element {
    rsx! {
        div {
            class: "mb-6 lg:mb-0",
            div {
                class: "flex items-center space-x-2 mb-4",
                img { src: "/logo.webp", alt: "Logo", class: "h-24" },
            }
            p { class: "text-sm text-gray-400", "Your ultimate travel planning assistant, powered by AWS Bedrock AI and Rust." }
        }
    }
}
