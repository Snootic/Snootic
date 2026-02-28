use leptos::{html, prelude::*};
use leptos_use::{use_intersection_observer, UseIntersectionObserverReturn};

use crate::structs::Personal;
use crate::three::HeroScene;

#[component]
pub fn Hero(personal: Personal) -> impl IntoView {
    let container_ref = NodeRef::<html::Section>::new();
    let ft_canvas_ref = NodeRef::<html::Canvas>::new();
    let back_canvas_ref = NodeRef::<html::Canvas>::new();
    
    let (has_been_visible, set_has_been_visible) = signal(false);
    
    let UseIntersectionObserverReturn { .. } = use_intersection_observer(
        container_ref,
        move |entries, _| {
            if entries[0].is_intersecting() && !has_been_visible.get_untracked() {
                set_has_been_visible.set(true);
            }
        },
    );

    Effect::new(move |_| {
        if has_been_visible.get() {
            let ft = ft_canvas_ref.get();
            let back = back_canvas_ref.get();
        
            if let (Some(ft_canvas), Some(back_canvas)) = (ft, back) {
                let js_data = serde_wasm_bindgen::to_value(&personal).unwrap();
                HeroScene(ft_canvas, back_canvas, js_data);
            }
        }
    });
    
    view! {
        <section 
            node_ref=container_ref
            class="relative min-h-screen flex flex-col items-center justify-center overflow-hidden"
        >
            <div class="absolute inset-0 bg-page" />

            <div
                class="absolute inset-0 pointer-events-none"
                style="background: radial-gradient(ellipse 80% 60% at 50% 40%, rgba(204,122,90,0.07) 0%, transparent 70%)"
            />
            
            <canvas 
                node_ref=back_canvas_ref 
                class="absolute inset-0 w-full h-full z-0 pointer-events-none" 
                style="display: block; pointer-events: none;"
            />
            
            <canvas 
                node_ref=ft_canvas_ref 
                class="absolute inset-0 w-full h-full z-20 pointer-events-none" 
                style="display: block; pointer-events: none;"
            />

            <div
                class="absolute top-0 right-0 w-[600px] h-[600px] pointer-events-none"
                style="background: radial-gradient(circle at 80% 20%, rgba(204,122,90,0.06) 0%, transparent 60%)"
            />

            <div class="relative z-10 text-center px-6 max-w-4xl mx-auto">
                <h1
                    class="text-5xl sm:text-7xl md:text-8xl font-bold mb-5 leading-[1.05] tracking-tight text-gradient animate-slide-up"
                    style="animation-delay: 100ms"
                >
                    {personal.name}
                </h1>

                <p
                    class="font-mono text-lg md:text-xl text-brand mb-5 animate-slide-up"
                    style="animation-delay: 200ms"
                >
                    {personal.title}
                </p>

                <p
                    class="text-base md:text-lg text-ink-2 max-w-xl mx-auto mb-12 leading-relaxed animate-slide-up"
                    style="animation-delay: 300ms"
                >
                    {personal.tagline}
                </p>

                <div
                    class="flex flex-wrap items-center justify-center gap-4 animate-slide-up"
                    style="animation-delay: 400ms"
                >
                    <a
                        href="#projects"
                        class="group inline-flex items-center gap-2 px-6 py-3 rounded-xl bg-brand text-page font-semibold text-sm hover:bg-brand-light transition-all glow-brand-hover"
                    >
                        "View My Work"
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
                            class="group-hover:translate-x-0.5 transition-transform"
                        >
                            <line x1="5" y1="12" x2="19" y2="12"></line>
                            <polyline points="12 5 19 12 12 19"></polyline>
                        </svg>
                    </a>

                    <a
                        href=personal.resume_url
                        class="group inline-flex items-center gap-2 px-6 py-3 rounded-xl border border-edge text-ink-2 text-sm hover:border-brand/50 hover:text-ink transition-all"
                    >
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
                            class="animate-pulse"
                        >
                            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
                            <polyline points="7 10 12 15 17 10"></polyline>
                            <line x1="12" y1="15" x2="12" y2="3"></line>
                        </svg>
                        "Download CV"
                    </a>
                </div>
            </div>
            
            <div 
                class="mt-10 w-full h-38 md:h-[200px] pointer-events-none" 
                aria-hidden="true" 
            />

            <div class="group absolute bottom-8 left-1/2 -translate-x-1/2 flex flex-col items-center gap-2 text-ink-3">
                <span class="text-[10px] font-mono tracking-[0.2em] uppercase">
                    "scroll"
                </span>
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
                    class="animate-bounce"
                >
                    <line x1="12" y1="5" x2="12" y2="19"></line>
                    <polyline points="19 12 12 19 5 12"></polyline>
                </svg>
            </div>
        </section>
    }
}
