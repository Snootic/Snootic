use leptos::{prelude::*};
use crate::{components::{section_header::SectionHeader, animate_on_scroll::AnimateOnScroll}, structs::{Personal, Stats}};

#[component]
pub fn About(personal: Personal, stats: Vec<Stats>) -> impl IntoView {
    let paragraphs = personal.bio.split("\n\n").map(|s| s.to_string()).collect::<Vec<String>>();
    
    view! {
        <section id="about" class="py-28 px-6">
            <div class="max-w-6xl mx-auto">
                <SectionHeader number=01 title="About Me" />
                
                <div class="grid md:grid-cols-2 gap-14 items-start">
                    <AnimateOnScroll>
                        <div class="space-y-5 text-ink-2 leading-[1.8] text-[15px]">
                            {paragraphs.into_iter().map(|p| {
                                view! {
                                    <p>
                                        {p}
                                    </p>
                                }
                            }).collect_view()}
                        </div>
                        
                        <div class="grid grid-cols-2 gap-3 mt-10">
                            {stats.iter().map(|s| {
                                view! {
                                    <div class="p-5 rounded-xl bg-surface border border-edge hover:border-brand/30 transition-colors">
                                        <div class="text-2xl font-bold text-brand mb-1">{s.value}</div>
                                        <div class="text-xs text-ink-3">{s.label}</div>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    </AnimateOnScroll>
                    
                    <AnimateOnScroll delay={100}>
                        <div class="relative max-w-sm mx-auto">
                            <div class="aspect-square rounded-2xl bg-surface border border-edge overflow-hidden">
                                <div class="w-full h-full bg-gradient-to-br from-brand/20 via-surface-2 to-page flex items-center justify-center">
                                    <img src="/assets/img_me_1.jpg"/> //TODO upload an image on the server
                                </div>
                            </div>
                            <div class="absolute -bottom-5 -left-5 px-4 py-3 rounded-xl bg-surface border border-edge shadow-xl text-xs font-mono text-ink-2">
                                <span class="text-ink-3">
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        width="15"
                                        height="15"
                                        viewBox="0 0 24 24"
                                        fill="none"
                                        stroke="currentColor"
                                        stroke-width="2"
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        class="transition-transform group-hover:scale-105"
                                    >
                                        <rect x="3" y="5" width="18" height="14" rx="2"></rect>
                                        <polyline points="3 7 12 13 21 7"></polyline>
                                    </svg>
                                    {personal.email}
                                </span>
                                </div>
                
                                <div class="absolute -top-4 -right-4 px-3.5 py-2 rounded-xl bg-brand text-page text-xs font-semibold shadow-lg glow-brand">
                                    {personal.availability}
                                </div>
                            <div class="absolute inset-0 rounded-2xl border border-brand/10 scale-[1.03] pointer-events-none" />
                        </div>
                    </AnimateOnScroll>
                </div>
            </div>
        </section>
    }
}