use leptos::{web_sys::window, web_sys::HtmlElement, prelude::*, wasm_bindgen::JsCast, ev, leptos_dom::helpers::window_event_listener};

use crate::structs::{Personal};

struct NavLinks<'a> {
    label: &'a str,
    href: &'a str,
}

const NAV_LINKS: &[NavLinks] = &[
    NavLinks { label: "About", href: "#about" },
    NavLinks { label: "Skills", href: "#skills" },
    NavLinks { label: "Experience", href: "#experience" },
    NavLinks { label: "Projects", href: "#projects" },
    NavLinks { label: "Contact", href: "#contact" },
];

#[component]
pub fn Navbar(
    personal: Personal,
) -> impl IntoView {
    let (_scrolled, set_scrolled) = signal(false);
    let (mobile_open, set_mobile_open) = signal(false);
    let (active, set_active) = signal("".to_string());

    let handle = window_event_listener(ev::scroll, move |_| {
        let scroll_y = window().unwrap().scroll_y().unwrap_or(0.0);
        
        set_scrolled.set(scroll_y > 24.0);

        let document = window().unwrap().document().unwrap();
        
        for link in NAV_LINKS.iter().rev() {
            if let Some(el) = document.get_element_by_id(link.label) {
                if let Ok(html_el) = el.dyn_into::<HtmlElement>() {
                    let offset = html_el.offset_top() as f64;
                    
                    if scroll_y >= offset - 120.0 {
                        set_active.set(link.label.to_string());
                        break;
                    }
                }
            }
        }
    });

    on_cleanup(move || handle.remove());

    view! {
        <header class="fixed top-0 left-0 right-0 z-50 transition-all duration-300">
            <nav class="max-w-6xl mx-auto px-6 h-16 flex items-center justify-between">
                <a href="/" class="flex items-center gap-3 group">
                    <span class="w-8 h-8 rounded-lg bg-brand flex items-center justify-center text-xs font-bold text-page tracking-wide group-hover:bg-brand-light transition-colors">
                        {personal.initials}
                    </span>
                    <span class="font-semibold text-ink hidden sm:block">{personal.name}</span>
                </a>

                <ul class="hidden md:flex items-center gap-7">
                    {NAV_LINKS.iter().map(|link| {
                        let class = move || if active.get() == link.label {
                            "text-sm transition-colors text-brand"
                        } else {
                            "text-sm transition-colors text-ink-2 hover:text-ink"
                        };
                        view! {
                            <li>
                                <a href={link.href} class={class}>
                                    {link.label}
                                </a>
                            </li>
                        }
                    }).collect::<Vec<_>>()}
                </ul>
                <a 
                    href="#contact" 
                    class="hidden md:inline-flex items-center gap-2 px-4 py-2 rounded-lg bg-brand text-page text-sm font-semibold hover:bg-brand-light transition-all glow-brand-hover"
                >
                    Contact Me
                </a>

                <button 
                    class="md:hidden text-ink-2 hover:text-ink transition-colors p-1" 
                    aria-label="Toggle menu"
                    on:click=move |_| set_mobile_open.update(|open| *open = !*open)
                >
                    {
                        move || if mobile_open.get() {
                            view! {
                                <svg class="w-6 h-6 cursor-pointer" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                                </svg>
                            }
                        } else {
                            view! {
                                <svg class="w-6 h-6 cursor-pointer" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>
                                </svg>
                            }
                        }
                    }
                    
                </button>
            </nav>

            <div
                class=move || {
                    let visibility = if mobile_open.get() {
                        "max-h-96 opacity-100"
                    } else {
                        "max-h-0 opacity-0"
                    };
                    
                    format!(
                        "md:hidden overflow-hidden transition-all duration-300 {} bg-page/95 backdrop-blur-lg border-b border-edge",
                        visibility
                    )
                }
            >
                <div class="px-6 py-4 space-y-1">
                    {NAV_LINKS.iter().map(|link| {
                        let class = move || if active.get() == link.label {
                            "block py-2.5 text-sm text-brand"
                        } else {
                            "block py-2.5 text-sm text-ink-2 hover:text-ink transition-colors"
                        };
                        view!{
                            <a
                                href={link.href}
                                class={class}
                            >
                            {link.label}
                            </a>
                        }
                    }).collect::<Vec<_>>()}
                    <div class="pt-2">
                        <a
                            href="#contact"
                            class="block w-full text-center py-2.5 rounded-lg bg-brand text-page text-sm font-semibold hover:bg-brand-light transition-colors"
                        >
                            Contact Me
                        </a>
                    </div>
                </div>
            </div>
        </header>
    }
}