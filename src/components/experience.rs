use leptos::prelude::*;
use crate::{components::{animate_on_scroll::AnimateOnScroll, section_header::SectionHeader}, structs::Experience};

#[component]
pub fn Experience(experience: Vec<Experience>) -> impl IntoView {
    view! {
        <section id="experience" class="py-28 px-6 bg-page-alt">
            <div class="max-w-4xl mx-auto">
                <SectionHeader number=3 title="Experience" />
    
                <div class="relative">
                    <div class="absolute left-[7px] top-2 bottom-2 w-px bg-edge" />
        
                    <div class="space-y-10 pl-10">
                        {experience.into_iter().enumerate().map(|(idx, entry)| {
                            view! {
                                <AnimateOnScroll delay={idx as u16 * 80}>
                                    <div class="relative">
                                        <div class="absolute -left-[2.6rem] top-[22px] w-3.5 h-3.5 rounded-full bg-brand ring-4 ring-page-alt" />
                    
                                        <div class="p-6 rounded-2xl bg-surface border border-edge hover:border-brand/25 transition-all">
                                            <div class="flex flex-wrap items-start justify-between gap-4 mb-4">
                                                <div>
                                                    <h3 class="text-base font-semibold text-ink mb-0.5">
                                                        {entry.role}
                                                    </h3>
                                                    <span class="text-brand font-medium text-sm">
                                                        {entry.company}
                                                    </span>
                                                </div>
                    
                                                <div class="flex flex-col gap-1 text-xs text-ink-3 font-mono">
                                                    <span class="flex items-center gap-1.5">
                                                        <svg
                                                            xmlns="http://www.w3.org/2000/svg"
                                                            width={16}
                                                            height={16}
                                                            viewBox="0 0 24 24"
                                                            fill="none"
                                                            stroke="currentColor"
                                                            stroke-width="2"
                                                            stroke-linecap="round"
                                                            stroke-linejoin="round"
                                                            class="transition-colors text-brand/70"
                                                        >
                                                            <rect width="18" height="18" x="3" y="4" rx="2" ry="2" />
                                                            <line x1="16" x2="16" y1="2" y2="6" />
                                                            <line x1="8" x2="8" y1="2" y2="6" />
                                                            <line x1="3" x2="21" y1="10" y2="10" />
                                                        </svg>
                                                        {entry.period}
                                                    </span>
                                                    <span class="flex items-center gap-1.5">
                                                        <svg
                                                            xmlns="http://www.w3.org/2000/svg"
                                                            width={16}
                                                            height={16}
                                                            viewBox="0 0 24 24"
                                                            fill="none"
                                                            stroke="currentColor"
                                                            stroke-width="2"
                                                            stroke-linecap="round"
                                                            stroke-linejoin="round"
                                                            class="transition-colors text-brand/70"
                                                        >
                                                            <path d="M20 10c0 6-8 12-8 12s-8-6-8-12a8 8 0 0 1 16 0Z" />
                                                            <circle cx="12" cy="10" r="3" />
                                                        </svg>
                                                        {entry.location}
                                                    </span>
                                                </div>
                                            </div>
                    
                                            <ul class="space-y-2 mb-5">
                                                {entry.description.iter().map(|item| {
                                                    view! {
                                                        <li class="flex gap-3 text-sm text-ink-2 leading-relaxed">
                                                        <span class="text-brand mt-0.5 shrink-0">"—"</span>
                                                            {*item}
                                                        </li>
                                                    }
                                                }).collect_view()}
                                            </ul>
                    
                                            <div class="flex flex-wrap gap-1.5">
                                                {entry.technologies.iter().map(|tech| {
                                                    view! {
                                                        <span
                                                            class="px-2.5 py-0.5 rounded-md text-[11px] font-mono text-ink-3 bg-page border border-edge"
                                                        >
                                                            {*tech}
                                                        </span>
                                                    }
                                                }).collect_view()}
                                            </div>
                                        </div>
                                    </div>
                                </AnimateOnScroll>
                            }
                        }).collect_view()}
                    </div>
                </div>
            </div>
        </section>
    }
}
