use dioxus::prelude::*;

#[component]
pub fn ShowcaseCard(
    date: String,
    title: String,
    description: String,
    logo: Asset,
    github_link: String,
) -> Element {
    let mut star_count = use_signal(|| "...".to_string());
    
    use_effect({
        let github_link = github_link.clone();
        move || {
            let github_link = github_link.clone();
            spawn(async move {
                // Extract repo path from GitHub URL
                if let Some(repo_path) = github_link.strip_prefix("https://github.com/") {
                    let repo_path = repo_path.trim_end_matches('/');
                    let api_url = format!("https://api.github.com/repos/{}", repo_path);
                    
                    if let Ok(response) = reqwest::get(&api_url).await {
                        if let Ok(json) = response.json::<serde_json::Value>().await {
                            if let Some(stars) = json["stargazers_count"].as_u64() {
                                let formatted_stars = if stars >= 1000 {
                                    format!("{:.1}k", stars as f64 / 1000.0)
                                } else {
                                    stars.to_string()
                                };
                                star_count.set(formatted_stars);
                            }
                        }
                    }
                }
            });
        }
    });
    rsx! {
        div { class: "bg-zinc-800 rounded-lg shadow-sm hover:shadow-md transition-shadow duration-200 p-6 border border-zinc-700 flex flex-col h-full",
            div { class: "mb-4",
                img {
                    src: logo,
                    alt: "{title} logo",
                    class: "w-24 h-24 object-contain",
                }
            }
            div { class: "text-sm text-gray-400 mb-2", "{date}" }
            h3 { class: "text-xl font-semibold text-white mb-3", "{title}" }
            p { class: "text-gray-300 leading-relaxed mb-4 flex-grow", "{description}" }
            div { class: "flex justify-start",
                a {
                href: "{github_link}",
                class: "inline-flex items-stretch border border-zinc-600 rounded-md overflow-hidden hover:border-zinc-500 transition-colors",
                target: "_blank",
                rel: "noopener noreferrer",
                div {
                    class: "flex items-center px-3 py-2 bg-zinc-700 hover:bg-zinc-600 transition-colors",
                    svg {
                        class: "w-4 h-4 mr-2 text-gray-300",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M11.049 2.927c.3-.921 1.603-.921 1.902 0l1.519 4.674a1 1 0 00.95.69h4.915c.969 0 1.371 1.24.588 1.81l-3.976 2.888a1 1 0 00-.363 1.118l1.518 4.674c.3.922-.755 1.688-1.538 1.118l-3.976-2.888a1 1 0 00-1.176 0l-3.976 2.888c-.783.57-1.838-.197-1.538-1.118l1.518-4.674a1 1 0 00-.363-1.118l-3.976-2.888c-.784-.57-.38-1.81.588-1.81h4.914a1 1 0 00.951-.69l1.519-4.674z"
                        }
                    }
                    span { class: "text-sm text-white font-medium", "Star" }
                }
                div {
                    class: "flex items-center px-3 py-2 bg-zinc-800 text-white",
                    span { class: "text-sm font-medium", "{star_count}" }
                }
            }
            }
        }
    }
}
