use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "bg-zinc-900 border-t border-zinc-700",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12",
                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8",
                    // Column 1
                    div {
                        h3 { class: "text-lg font-semibold text-white mb-4", "Community" }
                        ul { class: "space-y-2",
                            li {
                                a {
                                    href: "https://github.com/dioxuslabs",
                                    class: "text-gray-400 hover:text-white transition-colors",
                                    "GitHub"
                                }
                            }
                            li {
                                a {
                                    href: "https://discord.gg/XgGxMSkvUM",
                                    class: "text-gray-400 hover:text-white transition-colors",
                                    "Discord"
                                }
                            }
                            li {
                                a {
                                    href: "https://twitter.com/dioxuslabs",
                                    class: "text-gray-400 hover:text-white transition-colors",
                                    "Twitter"
                                }
                            }
                            li {
                                a {
                                    href: "https://www.youtube.com/@DioxusLabs",
                                    class: "text-gray-400 hover:text-white transition-colors",
                                    "YouTube"
                                }
                            }
                        }
                    }
                    // Column 2
                    div {
                        h3 { class: "text-lg font-semibold text-white mb-4", "Resources" }
                        ul { class: "space-y-2",
                            li {
                                a {
                                    href: "https://docs.rs/dioxus",
                                    class: "text-gray-400 hover:text-white transition-colors",
                                    "docs.rs"
                                }
                            }
                            li {
                                a {
                                    href: "https://crates.io/crates/dioxus",
                                    class: "text-gray-400 hover:text-white transition-colors",
                                    "crates.io"
                                }
                            }
                            li {
                                a {
                                    href: "https://dioxus.dev/learn/0.6/guide",
                                    class: "text-gray-400 hover:text-white transition-colors",
                                    "Guide"
                                }
                            }
                            li {
                                a {
                                    href: "https://dioxus.dev/awesome",
                                    class: "text-gray-400 hover:text-white transition-colors",
                                    "Awesome"
                                }
                            }
                            li {
                                a {
                                    href: "https://dioxus.dev/playground",
                                    class: "text-gray-400 hover:text-white transition-colors",
                                    "Playground"
                                }
                            }
                        }
                    }
                    // Column 3
                    div {
                        h3 { class: "text-lg font-semibold text-white mb-4", "Projects" }
                        ul { class: "space-y-2",
                            li {
                                a {
                                    href: "https://github.com/DioxusLabs/dioxus",
                                    class: "text-gray-400 hover:text-white transition-colors",
                                    "Dioxus"
                                }
                            }
                            li {
                                a {
                                    href: "https://github.com/DioxusLabs/dioxus/tree/main/packages/cli",
                                    class: "text-gray-400 hover:text-white transition-colors",
                                    "CLI"
                                }
                            }
                            li {
                                a {
                                    href: "https://github.com/DioxusLabs/taffy",
                                    class: "text-gray-400 hover:text-white transition-colors",
                                    "Taffy"
                                }
                            }
                            li {
                                a {
                                    href: "https://github.com/DioxusLabs/blitz",
                                    class: "text-gray-400 hover:text-white transition-colors",
                                    "Blitz"
                                }
                            }
                            li {
                                a {
                                    href: "https://github.com/DioxusLabs/sdk",
                                    class: "text-gray-400 hover:text-white transition-colors",
                                    "SDK"
                                }
                            }
                        }
                    }
                    // Column 4
                    div {
                        h3 { class: "text-lg font-semibold text-white mb-4", "Legal" }
                        ul { class: "space-y-2",
                            li {
                                a {
                                    href: "#",
                                    class: "text-gray-400 hover:text-white transition-colors",
                                    "Terms of Service"
                                }
                            }
                            li {
                                a {
                                    href: "#",
                                    class: "text-gray-400 hover:text-white transition-colors",
                                    "Privacy Policy"
                                }
                            }
                        }
                    }
                }
            }
            // Copyright row
            div { class: "border-t border-zinc-700 py-6",
                div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8",
                    div { class: "flex flex-col sm:flex-row justify-center items-center",
                        p { class: "text-gray-400 text-sm", "© 2025 < Insert Company Name Here >" }
                    }
                }
            }
        }
    }
}
