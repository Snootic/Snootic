use leptos::either::Either;
use leptos::prelude::*;

use crate::components::{animate_on_scroll::AnimateOnScroll, section_header::SectionHeader};
use crate::structs::SkillCategory;

#[component]
pub fn Skills(skills: Vec<SkillCategory>) -> impl IntoView {
    let skills_len = skills.len();
    
    view! {
        <section id="skills" class="py-28 px-6 bg-page-alt">
          <div class="max-w-6xl mx-auto">
            <SectionHeader number=2 title="Skills" />

            <div class="space-y-10">
              {skills.into_iter().enumerate().map(|(idx, cat)| {
                view! {
                  <AnimateOnScroll delay={idx as u16 * 60}>
                    <div class="flex flex-col sm:flex-row gap-5 sm:items-start">
                      <div class="w-28 shrink-0 pt-0.5">
                        <span class="font-mono text-xs text-ink-3 uppercase tracking-widest">
                          {cat.category}
                        </span>
                      </div>

                      <div class="flex flex-wrap gap-2">
                        {cat.items.iter().map(|skill| {
                          view! {
                            <span
                              class="px-3.5 py-1.5 rounded-lg bg-surface border border-edge text-sm text-ink-2 hover:border-brand/40 hover:text-ink hover:bg-surface-2 transition-all cursor-default"
                            >
                                {*skill}
                            </span>
                          }
                        }).collect_view()}
                      </div>
                    </div>

                    {if idx < skills_len - 1 {
                        Either::Left(view! { <div class="h-px bg-edge mt-10" /> })
                    } else {
                        Either::Right(())
                    }}
                  </AnimateOnScroll>
                }
              }).collect_view()}
            </div>

            <AnimateOnScroll delay={200}>
              <p class="mt-14 text-xs text-ink-3 font-mono border-l-2 border-brand/30 pl-4 leading-relaxed">
                "Always learning — currently exploring Rust."
              </p>
            </AnimateOnScroll>
          </div>
        </section>
    }
}