use crate::{components::icons::{github_icon, globe_icon, linkedin_icon, twitter_icon}, structs::{Personal, Social}};
use leptos::prelude::*;
use chrono::Datelike;

#[component]
pub fn Footer(personal: Personal, social: Social) -> impl IntoView {
    let year = chrono::Utc::now().year();
    let social_icons = vec![
            (social.github, "GitHub", github_icon().into_any()),
            (social.linkedin, "LinkedIn", linkedin_icon().into_any()),
            (social.twitter, "Twitter / X", twitter_icon().into_any()),
            (social.website, "Website", globe_icon().into_any()),
        ];
    view! {
        <footer class="border-t border-edge py-8 px-6">
            <div class="max-w-6xl mx-auto flex flex-col sm:flex-row items-center justify-between gap-5">
                <div class="flex items-center gap-3">
                    <span class="w-7 h-7 rounded-lg bg-brand flex items-center justify-center text-[10px] font-bold text-page tracking-wide">
                        {personal.initials}
                    </span>
                    <span class="text-xs text-ink-3">
                        {format!("© {} {}. Built with Rust, Leptos, Tailwind and Three.JS.", year, personal.name)}
                    </span>
                </div>
                <div class="flex items-center gap-4">
                    {social_icons.into_iter().filter_map(|(key, label, icon_view)| {
                        let href = match key {
                            "github" => social.github,
                            "linkedin" => social.linkedin,
                            "twitter" => social.twitter,
                            "website" => social.website,
                            _ => "",
                        };
                        if href.is_empty() {
                            None
                        } else {
                            Some(view! {
                                <a
                                    href=href
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    aria-label=label
                                    class="text-ink-3 hover:text-brand transition-colors"
                                >
                                    {icon_view}
                                </a>
                            })
                        }
                    }).collect_view()}
                </div>
            </div>
        </footer>
    }
}
