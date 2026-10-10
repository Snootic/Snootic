pub mod contact;
pub mod identity;
pub mod stack;
pub mod track;
pub mod work;

use crate::libs::locale;
use crate::structs::sheet_at;
use leptos::prelude::*;

#[component]
fn SheetHead(
    index: usize,
    view_key: &'static str,
    #[prop(default = false)] numeric_meta: bool,
) -> impl IntoView {
    let sheet = sheet_at(index);
    view! {
        <div class="p-head">
            <p class="p-title t-head">{move || locale::t(sheet.title)}</p>
            <p class="p-view t-label">
                {move || locale::t(view_key)} " "
                <b>{move || locale::t(sheet.face.name())}</b>
                {format!(" \u{00b7} {}", sheet.no)}
            </p>
            <p class="p-meta" class:num=numeric_meta>{move || locale::t(sheet.meta)}</p>
        </div>
    }
}
