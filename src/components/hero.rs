use super::ShowcaseCard;
use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    const BIONIC_GPT_LOGO: Asset = asset!("/assets/logos/bionic-gpt-logo.avif");
    const FREYA_LOGO: Asset = asset!("/assets/logos/freya-logo.avif");
    const FLONEUM_LOGO: Asset = asset!("/assets/logos/floneum-logo.avif");
    const HEMI_LOGO: Asset = asset!("/assets/logos/hemi-logo.avif");
    const BIYARD_LOGO: Asset = asset!("/assets/logos/biyard-logo.avif");
    const LINKECHO_LOGO: Asset = asset!("/assets/logos/linkecho-logo.avif");

    rsx! {
        div { class: "bg-zinc-950",
            main { class: "max-w-7xl mx-auto px-8 py-12",
                div { class: "mb-6",
                    h1 { class: "text-4xl font-bold text-white", "User Showcase" }
                }
                div { class: "mb-12",
                    p { class: "text-xl text-white",
                        "Check out how developers are using Dioxus for their products to serve thousands of users every day."
                    }
                }
                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8",
                    ShowcaseCard {
                        key: "Bionic GPT",
                        title: "Bionic GPT",
                        date: "Launched September 14, 2023",
                        description: "Bionic is an on-premise replacement for ChatGPT",
                        logo: BIONIC_GPT_LOGO,
                        github_link: "https://github.com/bionic-gpt/bionic-gpt",
                    }
                    ShowcaseCard {
                        key: "Floneum",
                        title: "Floneum",
                        date: "Launched May 4, 2023",
                        description: "Instant, controllable, local pre-trained AI models in Rust",
                        logo: FLONEUM_LOGO,
                        github_link: "https://github.com/floneum/floneum",
                    }
                    ShowcaseCard {
                        key: "Valin",
                        title: "Valin",
                        date: "Launched December 7, 2022",
                        description: "Cross-platform code editor made with Freya and Rust",
                        logo: FREYA_LOGO,
                        github_link: "https://github.com/marc2332/valin",
                    }
                    ShowcaseCard {
                        key: "Hemi",
                        title: "Hemi",
                        date: "Launched December 29, 2021",
                        description: "Typing tutor for training hands separately",
                        logo: HEMI_LOGO,
                        github_link: "https://github.com/kualta/hemi",
                    }
                    ShowcaseCard {
                        key: "LinkEcho",
                        title: "LinkEcho",
                        date: "Launched April 16, 2024",
                        description: "Batch replace/restore shortcut icons, personalized custom icon design",
                        logo: LINKECHO_LOGO,
                        github_link: "https://github.com/iKineticate/LinkEcho",
                    }
                    ShowcaseCard {
                        key: "Biyard Website",
                        title: "Biyard Website",
                        date: "Launched September 27, 2024",
                        description: "Use blockchain, AI, and security to solve real-world problems",
                        logo: BIYARD_LOGO,
                        github_link: "https://github.com/biyard/homepage",
                    }
                }
            }
        }
    }
}
