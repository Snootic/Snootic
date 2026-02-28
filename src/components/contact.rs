use leptos::{prelude::*, ev::{SubmitEvent, Event}, task::spawn_local};
use crate::components::{animate_on_scroll::AnimateOnScroll, icons::{mail_icon, map_pin, whatsapp_icon}, section_header::SectionHeader};
use crate::structs::Personal;

#[derive(Clone, Default)]
struct Form {
    name: String,
    email: String,
    subject: String,
    message: String,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Status {
    Idle,
    Sending,
    Sent,
}

impl Default for Status {
    fn default() -> Self {
        Status::Idle
    }
}

#[component]
pub fn Contact(personal: Personal) -> impl IntoView {
    let (form, set_form) = signal(Form::default());
    let (status, set_status) = signal(Status::Idle);

    let input_class = "w-full px-4 py-3 rounded-xl bg-surface border border-edge text-ink text-sm \
        placeholder:text-ink-4 focus:outline-none focus:border-brand/50 focus:bg-surface-2 \
        transition-all";

    let update = move |field: &'static str| {
        move |ev: Event| {
            let val = event_target_value(&ev);
            set_form.update(|f| match field {
                "name" => f.name = val,
                "email" => f.email = val,
                "subject" => f.subject = val,
                "message" => f.message = val,
                _ => {},
            });
        }
    };

    let handle_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        set_status.set(Status::Sending);
        spawn_local(async move {
            set_status.set(Status::Sent);
        });
    };

    let info_items = vec![
        (mail_icon().into_any(), "Email", personal.email, Some(format!("mailto:{}", personal.email))),
        (whatsapp_icon().into_any(), "Phone", personal.phone, Some("https://wa.me/5511961395863".to_string())),
        (map_pin().into_any(), "Location", personal.location, None),
    ];

    view! {
        <section id="contact" class="py-28 px-6">
            <div class="max-w-6xl mx-auto">
                <SectionHeader number=5 title="Get In Touch" />
                <div class="grid md:grid-cols-2 gap-14">
                    <AnimateOnScroll>
                        <p class="text-ink-2 leading-relaxed mb-10 text-[15px]">
                            "Whether you have a project in mind, a question, or just want to say hello — I'd love to hear from you. I typically reply within 24 hours."
                        </p>
                        <div class="space-y-5">
                            {info_items.into_iter().map(|(icon, label, value, href)| {
                                view! {
                                    <div class="flex items-center gap-4 group">
                                        <div class="w-11 h-11 rounded-xl bg-surface border border-edge flex items-center justify-center text-brand shrink-0 group-hover:border-brand/40 transition-colors">
                                            {icon}
                                        </div>
                                        <div>
                                            <div class="text-[10px] text-ink-3 font-mono uppercase tracking-widest mb-0.5">{label}</div>
                                            {if let Some(h) = href {
                                                view! { <a href=h class="text-sm text-ink-2 hover:text-ink transition-colors">{value}</a> }.into_any()
                                            } else {
                                                view! { <span class="text-sm text-ink-2">{value}</span> }.into_any()
                                            }}
                                        </div>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    </AnimateOnScroll>
                    <AnimateOnScroll delay=100>
                        {move || match status.get() {
                            Status::Sent => view! {
                                <div class="h-full flex items-center justify-center">
                                    <div class="text-center">
                                        <svg 
                                            xmlns="http://www.w3.org/2000/svg"
                                            width="48"
                                            height="48"
                                            viewBox="0 0 48 48"
                                            fill="none"
                                            stroke="currentColor"
                                            stroke-width="2"
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            class="lucide lucide-circle-check-icon lucide-circle-check text-brand mx-auto mb-4"
                                        >
                                            <circle cx="12" cy="12" r="10"/>
                                            <path d="m9 12 2 2 4-4"/>
                                        </svg>
                                        <h3 class="text-xl font-semibold text-ink mb-2">"Message sent!"</h3>
                                        <p class="text-ink-2 text-sm">"Thanks for reaching out. I'll get back to you soon."</p>
                                    </div>
                                </div>
                            }.into_any(),
                            _ => view! {
                                <form on:submit=handle_submit class="space-y-4">
                                    <div class="grid grid-cols-2 gap-4">
                                        <div>
                                            <label class="block text-[10px] text-ink-3 font-mono uppercase tracking-widest mb-1.5">"Name"</label>
                                            <input
                                                type="text"
                                                required
                                                placeholder="Your name"
                                                prop:value=move || form.get().name
                                                on:input=update("name")
                                                class=input_class
                                            />
                                        </div>
                                        <div>
                                            <label class="block text-[10px] text-ink-3 font-mono uppercase tracking-widest mb-1.5">"Email"</label>
                                            <input
                                                type="email"
                                                required
                                                placeholder="you@example.com"
                                                prop:value=move || form.get().email
                                                on:input=update("email")
                                                class=input_class
                                            />
                                        </div>
                                    </div>
                                    <div>
                                        <label class="block text-[10px] text-ink-3 font-mono uppercase tracking-widest mb-1.5">"Subject"</label>
                                        <input
                                            type="text"
                                            placeholder="What's this about?"
                                            prop:value=move || form.get().subject
                                            on:input=update("subject")
                                            class=input_class
                                        />
                                    </div>
                                    <div>
                                        <label class="block text-[10px] text-ink-3 font-mono uppercase tracking-widest mb-1.5">"Message"</label>
                                        <textarea
                                            required
                                            rows=6
                                            placeholder="Your message…"
                                            prop:value=move || form.get().message
                                            on:input=update("message")
                                            class=format!("{} resize-none", input_class)
                                        />
                                    </div>
                                    <button
                                        type="submit"
                                        disabled=move || status.get() == Status::Sending
                                        class="w-full flex items-center justify-center gap-2 px-6 py-3.5 rounded-xl bg-brand text-page font-semibold text-sm hover:bg-brand-light glow-brand-hover transition-all disabled:opacity-60 disabled:cursor-not-allowed"
                                    >
                                        {move || if status.get() == Status::Sending {
                                            "Sending…"
                                        } else {
                                            "Send Message"
                                        }}
                                    </button>
                                </form>
                            }.into_any()
                        }}
                    </AnimateOnScroll>
                </div>
            </div>
        </section>
    }
}
