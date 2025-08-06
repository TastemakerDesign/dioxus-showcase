use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    const DIOXUS_LOGO: Asset = asset!("/assets/dioxus-logo.png");

    let mut star_count = use_signal(|| "30.0k".to_string());
    let mut is_mobile_menu_open = use_signal(|| false);
    let mut search_input_ref = use_signal(|| None::<Event<MountedData>>);
    let mut just_focused_by_slash = use_signal(|| false);

    use_effect(move || {
        spawn(async move {
            if let Ok(response) =
                reqwest::get("https://api.github.com/repos/DioxusLabs/dioxus").await
            {
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
        });
    });

    use_effect(move || {
        spawn(async move {
            use gloo::events::EventListener;
            use wasm_bindgen::JsCast;

            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();

            let listener = EventListener::new(&document, "keydown", move |event| {
                let keyboard_event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap();

                if keyboard_event.key() == "/"
                    && !keyboard_event.ctrl_key()
                    && !keyboard_event.alt_key()
                    && !keyboard_event.meta_key()
                {
                    // Check if we're currently focused on an input field (but not our search box)
                    let should_focus_search = if let Some(target) = keyboard_event.target() {
                        if let Some(element) = target.dyn_ref::<web_sys::HtmlElement>() {
                            let tag_name = element.tag_name().to_lowercase();
                            tag_name != "input" && tag_name != "textarea"
                        } else {
                            true
                        }
                    } else {
                        true
                    };

                    if should_focus_search {
                        keyboard_event.prevent_default();
                        just_focused_by_slash.set(true);
                        if let Some(search_ref) = search_input_ref.read().as_ref() {
                            let _ = search_ref.set_focus(true);
                        }
                    }
                }
            });

            // Keep the listener alive
            std::mem::forget(listener);
        });
    });

    rsx! {
        header { class: "sticky top-0 z-50 bg-zinc-900/50 backdrop-blur-md backdrop-saturate-150 shadow-sm border-b border-zinc-700/50",
            nav { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                div { class: "flex justify-between items-center h-16",
                    // Dioxus logo and nav
                    div { class: "flex items-center space-x-4 md:space-x-8",
                        div { class: "flex items-center",
                            img {
                                src: DIOXUS_LOGO,
                                alt: "Dioxus Logo",
                                class: "w-6 h-6 md:w-8 md:h-8",
                            }
                            span {
                                class: "ml-1 text-lg md:text-xl text-white font-mono",
                                style: "font-family: Menlo, monospace",
                                "DIOXUS"
                            }
                        }
                        div { class: "hidden md:flex space-x-6",
                            a {
                                href: "#",
                                class: "text-gray-300 hover:text-white",
                                "Learn"
                            }
                            a {
                                href: "#",
                                class: "text-gray-300 hover:text-white",
                                "Blog"
                            }
                            a { href: "#", class: "text-blue-400 font-medium", "Showcase" }
                            a {
                                href: "#",
                                class: "text-gray-300 hover:text-white",
                                "Community"
                            }
                        }
                    }
                    // Mobile menu button and desktop items
                    div { class: "flex items-center space-x-2 md:space-x-4",
                        // Mobile menu button
                        button {
                            class: "lg:hidden p-2 text-gray-400 hover:text-white",
                            onclick: move |_| is_mobile_menu_open.set(!is_mobile_menu_open()),
                            svg {
                                class: "w-6 h-6",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                if is_mobile_menu_open() {
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M6 18L18 6M6 6l12 12",
                                    }
                                } else {
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M4 6h16M4 12h16M4 18h16",
                                    }
                                }
                            }
                        }
                        // Desktop-only search bar
                        div { class: "relative hidden lg:block",
                            svg {
                                class: "absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-zinc-400",
                                view_box: "0 0 20 20",
                                fill: "currentColor",
                                path { d: "M9 3.5a5.5 5.5 0 100 11 5.5 5.5 0 000-11zM2 9a7 7 0 1112.452 4.391l3.328 3.329a.75.75 0 11-1.06 1.06l-3.329-3.328A7 7 0 012 9z" }
                            }
                            input {
                                r#type: "text",
                                placeholder: "Search...",
                                class: "bg-zinc-800 border border-zinc-700 rounded-lg pl-10 pr-4 py-2 text-white placeholder-zinc-300 w-64 focus:outline-none focus:ring-2 focus:ring-blue-500",
                                onmounted: move |mounted| {
                                    search_input_ref.set(Some(mounted));
                                },
                                onkeypress: move |evt| {
                                    if just_focused_by_slash() && evt.code() == Code::Slash {
                                        evt.prevent_default();
                                        just_focused_by_slash.set(false);
                                    }
                                },
                                oninput: move |_| {
                                    just_focused_by_slash.set(false);
                                },
                            }
                            span { class: "absolute right-2 bg-zinc-900 w-6 h-6 p-3 pb-3.5 mt-[-1] rounded-md flex justify-center items-center border border-zinc-700 top-2 text-zinc-300 text-sm",
                                "/"
                            }
                        }
                        // Desktop-only social icons
                        div { class: "hidden lg:flex items-center space-x-3",
                            // Crates.io icon
                            a {
                                href: "https://crates.io/crates/dioxus",
                                class: "text-gray-400 hover:text-white",
                                title: "Crates.io",
                                svg {
                                    class: "w-6 h-6",
                                    view_box: "0 0 24 24",
                                    fill: "currentColor",
                                    path { d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 17.93c-3.94-.49-7-3.85-7-7.93 0-.62.08-1.21.21-1.79L9 15v1c0 1.1.9 2 2 2v1.93zm6.9-2.54c-.26-.81-1-1.39-1.9-1.39h-1v-3c0-.55-.45-1-1-1H8v-2h2c.55 0 1-.45 1-1V7h2c1.1 0 2-.9 2-2v-.41c2.93 1.19 5 4.06 5 7.41 0 2.08-.8 3.97-2.1 5.39z" }
                                }
                            }
                            // Discord icon
                            a {
                                href: "https://discord.gg/XgGxMSkvUM",
                                class: "text-gray-400 hover:text-white",
                                title: "Discord",
                                svg {
                                    class: "w-6 h-6",
                                    view_box: "0 0 24 24",
                                    fill: "currentColor",
                                    path {
                                        d: "M20.317 4.37a19.791 19.791 0 0 0-4.885-1.515.074.074 0 0 0-.079.037c-.21.375-.444.864-.608 1.25a18.27 18.27 0 0 0-5.487 0 12.64 12.64 0 0 0-.617-1.25.077.077 0 0 0-.079-.037A19.736 19.736 0 0 0 3.677 4.37a.07.07 0 0 0-.032.027C.533 9.046-.32 13.58.099 18.057a.082.082 0 0 0 .031.057 19.9 19.9 0 0 0 5.993 3.03.078.078 0 0 0 .084-.028c.462-.63.874-1.295 1.226-1.994a.076.076 0 0 0-.041-.106 13.107 13.107 0 0 1-1.872-.892.077.077 0 0 1-.008-.128 10.2 10.2 0 0 0 .372-.292.074.074 0 0 1 .077-.01c3.928 1.793 8.18 1.793 12.062 0a.074.074 0 0 1 .078.01c.12.098.246.198.373.292a.077.077 0 0 1-.006.127 12.299 12.299 0 0 1-1.873.892.077.077 0 0 0-.041.107c.36.698.772 1.362 1.225 1.993a.076.076 0 0 0 .084.028 19.839 19.839 0 0 0 6.002-3.03.077.077 0 0 0 .032-.054c.5-5.177-.838-9.674-3.549-13.66a.061.061 0 0 0-.031-.03zM8.02 15.33c-1.183 0-2.157-1.085-2.157-2.419 0-1.333.956-2.419 2.157-2.419 1.21 0 2.176 1.096 2.157 2.42 0 1.333-.956 2.418-2.157 2.418zm7.975 0c-1.183 0-2.157-1.085-2.157-2.419 0-1.333.955-2.419 2.157-2.419 1.21 0 2.176 1.096 2.157 2.42 0 1.333-.946 2.418-2.157 2.418z",
                                    }
                                }
                            }
                            // GitHub icon with star count
                            a {
                                href: "https://github.com/DioxusLabs/dioxus",
                                class: "flex items-center space-x-1 text-gray-400 hover:text-white",
                                title: "GitHub",
                                svg {
                                    class: "w-6 h-6",
                                    view_box: "0 0 24 24",
                                    fill: "currentColor",
                                    path { d: "M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z" }
                                }
                                span { class: "text-sm", "{star_count}" }
                            }
                        }
                        div { class: "hidden lg:flex flex-row space-x-3",
                            a {
                                href: "https://dioxus.dev/deploy",
                                class: "bg-gray-700 text-white px-4 py-2 rounded-md hover:bg-gray-600",
                                "Deploy"
                            }
                            a {
                                href: "https://dioxus.dev/learn/0.6/#",
                                class: "bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700",
                                "Learn"
                            }
                        }
                    }
                }
            }
            // Mobile menu
            if is_mobile_menu_open() {
                div { class: "lg:hidden bg-zinc-900/95 backdrop-blur-md border-t border-zinc-700/50",
                    div { class: "px-4 py-2 space-y-1",
                        a {
                            href: "#",
                            class: "block px-3 py-2 text-gray-300 hover:text-white hover:bg-zinc-800 rounded-md",
                            "Learn"
                        }
                        a {
                            href: "#",
                            class: "block px-3 py-2 text-gray-300 hover:text-white hover:bg-zinc-800 rounded-md",
                            "Blog"
                        }
                        a {
                            href: "#",
                            class: "block px-3 py-2 text-blue-400 font-medium hover:bg-zinc-800 rounded-md",
                            "Showcase"
                        }
                        a {
                            href: "#",
                            class: "block px-3 py-2 text-gray-300 hover:text-white hover:bg-zinc-800 rounded-md",
                            "Community"
                        }
                        div { class: "border-t border-zinc-700 pt-4 mt-2 pb-2",
                            div { class: "flex items-center space-x-4 px-3 py-2",
                                a {
                                    href: "https://crates.io/crates/dioxus",
                                    class: "text-gray-400 hover:text-white",
                                    title: "Crates.io",
                                    svg {
                                        class: "w-5 h-5",
                                        view_box: "0 0 24 24",
                                        fill: "currentColor",
                                        path { d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 17.93c-3.94-.49-7-3.85-7-7.93 0-.62.08-1.21.21-1.79L9 15v1c0 1.1.9 2 2 2v1.93zm6.9-2.54c-.26-.81-1-1.39-1.9-1.39h-1v-3c0-.55-.45-1-1-1H8v-2h2c.55 0 1-.45 1-1V7h2c1.1 0 2-.9 2-2v-.41c2.93 1.19 5 4.06 5 7.41 0 2.08-.8 3.97-2.1 5.39z" }
                                    }
                                }
                                a {
                                    href: "https://discord.gg/XgGxMSkvUM",
                                    class: "text-gray-400 hover:text-white",
                                    title: "Discord",
                                    svg {
                                        class: "w-5 h-5",
                                        view_box: "0 0 24 24",
                                        fill: "currentColor",
                                        path {
                                            d: "M20.317 4.37a19.791 19.791 0 0 0-4.885-1.515.074.074 0 0 0-.079.037c-.21.375-.444.864-.608 1.25a18.27 18.27 0 0 0-5.487 0 12.64 12.64 0 0 0-.617-1.25.077.077 0 0 0-.079-.037A19.736 19.736 0 0 0 3.677 4.37a.07.07 0 0 0-.032.027C.533 9.046-.32 13.58.099 18.057a.082.082 0 0 0 .031.057 19.9 19.9 0 0 0 5.993 3.03.078.078 0 0 0 .084-.028c.462-.63.874-1.295 1.226-1.994a.076.076 0 0 0-.041-.106 13.107 13.107 0 0 1-1.872-.892.077.077 0 0 1-.008-.128 10.2 10.2 0 0 0 .372-.292.074.074 0 0 1 .077-.01c3.928 1.793 8.18 1.793 12.062 0a.074.074 0 0 1 .078.01c.12.098.246.198.373.292a.077.077 0 0 1-.006.127 12.299 12.299 0 0 1-1.873.892.077.077 0 0 0-.041.107c.36.698.772 1.362 1.225 1.993a.076.076 0 0 0 .084.028 19.839 19.839 0 0 0 6.002-3.03.077.077 0 0 0 .032-.054c.5-5.177-.838-9.674-3.549-13.66a.061.061 0 0 0-.031-.03zM8.02 15.33c-1.183 0-2.157-1.085-2.157-2.419 0-1.333.956-2.419 2.157-2.419 1.21 0 2.176 1.096 2.157 2.42 0 1.333-.956 2.418-2.157 2.418zm7.975 0c-1.183 0-2.157-1.085-2.157-2.419 0-1.333.955-2.419 2.157-2.419 1.21 0 2.176 1.096 2.157 2.42 0 1.333-.946 2.418-2.157 2.418z",
                                        }
                                    }
                                }
                                a {
                                    href: "https://github.com/DioxusLabs/dioxus",
                                    class: "flex items-center space-x-1 text-gray-400 hover:text-white",
                                    title: "GitHub",
                                    svg {
                                        class: "w-5 h-5",
                                        view_box: "0 0 24 24",
                                        fill: "currentColor",
                                        path { d: "M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z" }
                                    }
                                    span { class: "text-sm", "{star_count}" }
                                }
                            }
                        }
                        div { class: "border-t border-zinc-700 pt-4 mt-2 space-y-2 mb-2",
                            a {
                                href: "https://dioxus.dev/deploy",
                                class: "block w-full text-center bg-gray-700 text-white px-4 py-2 rounded-md hover:bg-gray-600",
                                "Deploy"
                            }
                            a {
                                href: "https://dioxus.dev/learn/0.6/#",
                                class: "block w-full text-center bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700",
                                "Learn"
                            }
                        }
                    }
                }
            }
        }
    }
}
