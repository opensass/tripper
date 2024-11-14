pub(crate) mod author;
pub(crate) mod card;
pub(crate) mod rating;

use crate::components::testimonial::author::AuthorInfo;
use crate::components::testimonial::rating::StarRating;
use crate::theme::Theme;
use crate::theme::THEME;
use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_regular_icons::FaStar;
use dioxus_free_icons::Icon;

#[derive(Props, Clone, PartialEq)]
pub struct TestimonialData {
    quote: &'static str,
    author_name: &'static str,
    author_title: &'static str,
    author_image: &'static str,
    company_logo: &'static str,
    star_images: Vec<Element>,
}

#[allow(unused_mut)]
#[component]
pub fn Testimonial() -> Element {
    let testimonials = vec![
        TestimonialData {
            quote: "I asked Tripper to plan a weekend getaway, and I ended up with a luxury trip to Mars. Thanks, Bedrock AI!",
            author_name: "Elon Musk",
            author_title: "CEO, SpaceX",
            author_image: "./elon.webp",
            company_logo: "./spacex.webp",
            star_images: vec![rsx! {Icon {
                width: 30,
                height: 30,
                icon: FaStar,
            }}; 5],
        },
        TestimonialData {
            quote: "Tripper planned my honeymoon... and somehow it turned into a spontaneous trip to Japan. I love AI!",
            author_name: "Johnny Depp",
            author_title: "Actor, Adventurer",
            author_image: "https://resize.elle.fr/square/var/plain_site/storage/images/beaute/news-beaute/parfums/johnny-depp-nouveau-visage-d-un-parfum-dior-2956038/54653840-1-fre-FR/Johnny-Depp-nouveau-visage-d-un-parfum-Dior.jpg",
            company_logo: "https://w7.pngwing.com/pngs/826/892/png-transparent-pirates-of-the-caribbean-at-world-s-end-jack-sparrow-elizabeth-swann-will-turner-pirates-of-the-caribbean-piracy-pirates-of-the-caribbean-background-piracy-film-johnny-depp.png",
            star_images: vec![rsx! {Icon {
                width: 30,
                height: 30,
                icon: FaStar,
            }}; 5],
        },
        TestimonialData {
            quote: "I asked Tripper to organize a vacation... and it booked me a week in Atlantis. Am I trippin' or is this AI magic?",
            author_name: "Willy Wonka",
            author_title: "Chocolate Factory Owner",
            author_image: "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRAqDkElB9PBz3XL6ZDAlTWcBc_hYlRJgSiOw&s",
            company_logo: "https://img.favpng.com/22/7/6/the-willy-wonka-candy-company-wonka-bar-charlie-and-the-chocolate-factory-charlie-bucket-png-favpng-mG49wuv5Gb8RDupGzs586Retb.jpg",
            star_images: vec![rsx! {Icon {
                width: 30,
                height: 30,
                icon: FaStar,
            }}; 5],
        },
    ];

    let dark_mode = *THEME.read();
    let mut current_index = use_signal(|| 0);

    client! {
        let vec_len = testimonials.len();
        let mut eval = use_hook(|| {
            eval(
                r#"
                setInterval(() => {
                    dioxus.send("");
                }, 5000)
                "#,
            )
        });

        use_hook(|| {
            spawn(async move {
                loop {
                    let _ = eval.recv().await;
                    current_index.set((current_index() + 1) % vec_len);
                }
            })
        });
    }

    rsx! {
        section {
            id: "testimonial",
            class: format!("flex flex-col items-center justify-center min-h-screen p-8 {}",
            if dark_mode == Theme::Dark { "bg-gray-900 text-white" } else { "bg-white text-black" }),

            div { class: "flex flex-col items-center mb-8",
                h2 { class: "text-4xl font-bold text-center",
                    "What People Are Saying about Tripper"
                }

                p { class: format!("mt-2 text-lg {}", if dark_mode == Theme::Dark { "text-gray-300" } else { "text-gray-700" }),
                    "Tripper: Where AI meets your next adventure."
                }
            }

            div { class: "flex flex-wrap justify-center items-center gap-8 p-4",
                for (i, testimonial) in testimonials.iter().enumerate() {
                    div { class: format!("transition-transform duration-500 transform {}, hover:scale-105 hover:shadow-xl",
                        if current_index() == i { "opacity-100 scale-100" } else { "opacity-50 scale-75 blur-sm" }),
                        div { class: format!("{} p-8 rounded-xl shadow-2xl text-center max-w-sm border",
                            if dark_mode == Theme::Dark { "border-gray-700 bg-gray-800" } else { "bg-white border-gray-300" }),
                            StarRating { star_images: testimonial.star_images.clone() }
                            blockquote {
                                class: format!("text-lg font-semibold italic {}",
                                    if dark_mode == Theme::Dark { "text-gray-400" } else { "text-gray-600" }
                                ),
                                "{testimonial.quote}"
                            }
                            AuthorInfo {
                                author_image: testimonial.author_image,
                                author_name: testimonial.author_name,
                                author_title: testimonial.author_title,
                                company_logo: testimonial.company_logo,
                            }
                        }
                    }
                }
            }

            div { class: "flex justify-center mt-6 space-x-2",
                for (i, _) in testimonials.iter().enumerate() {
                    div { class: format!("w-3 h-3 rounded-full {} transition-all duration-300",
                        if current_index() == i { "bg-gradient-to-r from-blue-400 to-indigo-500" } else { "bg-gray-400" })
                    }
                }
            }
        }
    }
}
