//! `rust_i18n::locale()` is a plain global with no reactivity, so a string that has
//! to re-draw on a language change goes through [`t`], which reads the signal.

use std::cell::OnceCell;

use crate::libs::browser::{document_opt, window_opt};
use leptos::prelude::*;

/// Not the source of truth for which languages exist (that is
/// `rust_i18n::available_locales!`); this is the register's labels and `<html lang>`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Locale {
    En,
    PtBr,
}

impl Locale {
    pub const ALL: [Locale; 2] = [Locale::En, Locale::PtBr];

    /// The `rust-i18n` catalogue key.
    pub const fn code(self) -> &'static str {
        match self {
            Locale::En => "en",
            Locale::PtBr => "pt-BR",
        }
    }

    /// The BCP 47 tag for `<html lang>`.
    pub const fn tag(self) -> &'static str {
        self.code()
    }

    pub const fn label(self) -> &'static str {
        match self {
            Locale::En => "EN",
            Locale::PtBr => "PT",
        }
    }

    pub const fn alt_label(self) -> &'static str {
        match self {
            Locale::En => "PT",
            Locale::PtBr => "EN",
        }
    }

    pub const fn next(self) -> Locale {
        match self {
            Locale::En => Locale::PtBr,
            Locale::PtBr => Locale::En,
        }
    }

    /// Never translated: "Português" is how a Portuguese speaker is told which button
    /// to press.
    pub fn name(self) -> String {
        t(&format!("app.language.{}", self.code()))
    }

    /// True when `tag` names this locale, by full tag or by bare language.
    fn matches(self, tag: &str) -> bool {
        let t = normalise(tag);
        t.split('-').next() == normalise(self.code()).split('-').next()
    }
}

/// English, not Portuguese: the set is addressed from outside Brazil more often than
/// inside it.
pub const fn default_locale() -> Locale {
    Locale::En
}

/// Strip a platform's decorations: `pt_BR.UTF-8@latin` becomes `pt-br`.
fn normalise(tag: &str) -> String {
    tag.split(['.', '@'])
        .next()
        .unwrap_or_default()
        .trim()
        .replace('_', "-")
        .to_ascii_lowercase()
}

/// Detection order: `?lang=` on the URL, then the last choice, then the platform's.
pub fn detect() -> Locale {
    if let Some(from_url) = query_locale() {
        return from_url;
    }
    if let Some(stored) = storage_get("locale").and_then(|k| Locale::from_tag(&k)) {
        return stored;
    }
    if window_opt().is_none() {
        return default_locale();
    }
    sys_locale::get_locales()
        .find_map(|tag| Locale::from_tag(&tag))
        .unwrap_or_else(default_locale)
}

impl Locale {
    pub fn from_tag(tag: &str) -> Option<Locale> {
        Locale::ALL.into_iter().find(|l| l.matches(tag))
    }
}

/* --------------------------------------------------------------- the signal -- */

thread_local! {
    /// Only reached outside the shell's context; a fresh signal per call would never
    /// see a language change.
    static FALLBACK: OnceCell<RwSignal<Locale>> = const { OnceCell::new() };
}

pub fn locale() -> RwSignal<Locale> {
    use_context::<RwSignal<Locale>>()
        .unwrap_or_else(|| FALLBACK.with(|c| *c.get_or_init(|| RwSignal::new(detect()))))
}

/// Resolve a catalogue key. The signal read is the point: it subscribes the caller,
/// so the string re-draws when the register is pressed.
pub fn t(key: &str) -> String {
    rust_i18n::t!(key, locale = locale().get().code()).to_string()
}

pub fn t_untracked(key: &str) -> String {
    rust_i18n::t!(key, locale = locale().get_untracked().code()).to_string()
}

/// Record the choice in the library, on `<html lang>`, and in storage. `<html lang>`
/// is not decoration — a screen reader picks its voice from it.
pub fn set_locale(sig: RwSignal<Locale>, next: Locale) {
    if sig.get_untracked() == next {
        return;
    }
    sig.set(next);
    reflect(next);
    storage_set("locale", next.code());
}

pub fn reflect(next: Locale) {
    let Some(doc) = document_opt() else {
        return;
    };
    if let Some(root) = doc.document_element() {
        let _ = root.set_attribute("lang", next.tag());
        let _ = root.set_attribute("data-locale", next.code());
    }
}

/// Set the flag the stylesheet waits on; the server cannot know the language, so the
/// client reveals the sheet only once it does.
pub fn mark_ready() {
    let Some(doc) = document_opt() else {
        return;
    };
    if let Some(root) = doc.document_element() {
        let _ = root.set_attribute("data-ready", "");
    }
}

/* -------------------------------------------------------------- the browser -- */

fn query_locale() -> Option<Locale> {
    let search = window_opt()?.location().search().ok()?;
    let query = search.strip_prefix('?').unwrap_or(&search);
    query.split('&').find_map(|pair| {
        let (k, v) = pair.split_once('=')?;
        (k == "lang").then(|| Locale::from_tag(v)).flatten()
    })
}

fn storage_get(key: &str) -> Option<String> {
    window_opt()?
        .local_storage()
        .ok()
        .flatten()?
        .get_item(key)
        .ok()
        .flatten()
}

fn storage_set(key: &str, value: &str) {
    if let Some(Ok(Some(store))) = window_opt().map(|w| w.local_storage()) {
        let _ = store.set_item(key, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn language_tags_accept_regions_and_platform_suffixes() {
        for tag in ["pt", "pt-PT", "PT_br.UTF-8@latin", " pt-BR "] {
            assert_eq!(Locale::from_tag(tag), Some(Locale::PtBr));
        }
        for tag in ["en", "en-US", "en_GB.UTF-8"] {
            assert_eq!(Locale::from_tag(tag), Some(Locale::En));
        }
        for tag in ["", "fr", "english", "ptx"] {
            assert_eq!(Locale::from_tag(tag), None);
        }
    }

    #[test]
    fn translation_uses_the_owners_locale() {
        let owner = Owner::new();
        owner.with(|| {
            let signal = RwSignal::new(Locale::PtBr);
            provide_context(signal);
            let portuguese = t("sheets.identity.title");
            signal.set(Locale::En);
            let english = t("sheets.identity.title");
            assert_ne!(portuguese, english);
            signal.set(Locale::PtBr);
            assert_eq!(t("sheets.identity.title"), portuguese);
        });
    }
}
