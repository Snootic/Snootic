use leptos::{html, prelude::*, children::Children};
use leptos_use::use_intersection_observer;

#[component]
pub fn AnimateOnScroll(children: Children, #[prop(optional)] class_name: &'static str, #[prop(default = 0)] delay: u16) -> impl IntoView {
    let el = NodeRef::<html::Div>::new();
    let (is_visible, set_visible) = signal(false);
    
    use_intersection_observer(
        el,
        move |entries, _| {
            set_visible.set(entries[0].is_intersecting());
        },
    );
    
    let base_class = "transition-all duration-700 ease-out".to_owned();
    
    view! {
        <div 
            node_ref=el 
            class=move || {
                let visibility_state = if is_visible.get() { 
                    "opacity-100 translate-y-0" 
                } else { 
                    "opacity-0 translate-y-6" 
                };
                format!("{} {} {}", base_class, visibility_state, class_name)
            }
            style=move || {
                let d = if is_visible.get() { delay } else { 0 };
                format!("transition-delay: {}ms;", d)
            }
        >
            {children()}
        </div>
    }
}