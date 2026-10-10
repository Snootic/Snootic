use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos::wasm_bindgen::JsValue;
use leptos::web_sys;

use crate::cube::{self, Mat3, IDENTITY};
use crate::js;
use crate::libs::browser::{document_opt, el, reduced_motion, window_opt};
use crate::libs::locale;
use crate::structs::{
    sheet_at, sheet_index, sheet_of_face, Direction, Face, PortfolioContent, SHEETS,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Motion {
    /// The page is drawn: the underdrawing is generated and swept by a pen.
    Plot,
    /// Everything is instant and quantised.
    Mech,
}

impl Motion {
    pub const fn key(self) -> &'static str {
        match self {
            Motion::Plot => "plot",
            Motion::Mech => "mech",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Motion::Plot => "Plot",
            Motion::Mech => "Mech",
        }
    }
}

pub const TURN_MS: f64 = 260.0;

#[derive(Clone, Copy)]
pub struct Nav {
    pub booted: RwSignal<bool>,
    content: StoredValue<PortfolioContent>,
    pub sheet: RwSignal<usize>,
    pub orient: RwSignal<Mat3>,
    /// The orientation the cube is turning toward.
    pub goal: RwSignal<Mat3>,
    pub preview: RwSignal<Option<Face>>,
    pub motion: RwSignal<Motion>,
    pub open: RwSignal<Option<&'static str>>,
    pub closing: RwSignal<bool>,
    pub rest: RwSignal<bool>,
    pub plot: RwSignal<u32>,
    /// Per panel: whether its body actually scrolls, so the cue is honest.
    pub scrolls: RwSignal<[bool; SHEETS.len()]>,
    /// Per panel: where it is revealed from — the direction the view moved. Kept
    /// here rather than on the DOM so a re-render cannot lose it.
    pub reveal: RwSignal<[Direction; SHEETS.len()]>,
}

impl Nav {
    pub fn new(content: PortfolioContent) -> Nav {
        Nav {
            content: StoredValue::new(content),
            booted: RwSignal::new(false),
            sheet: RwSignal::new(0),
            orient: RwSignal::new(IDENTITY),
            goal: RwSignal::new(IDENTITY),
            preview: RwSignal::new(None),
            motion: RwSignal::new(Motion::Plot),
            open: RwSignal::new(None),
            closing: RwSignal::new(false),
            rest: RwSignal::new(true),
            plot: RwSignal::new(0),
            scrolls: RwSignal::new([false; SHEETS.len()]),
            reveal: RwSignal::new(SHEETS.map(|s| s.reveal)),
        }
    }

    pub fn content(&self) -> PortfolioContent {
        self.content.get_value()
    }

    pub fn go(&self, index: usize, dir: Direction) {
        self.dismiss_record();
        let index = index % SHEETS.len();
        let sheet = sheet_at(index);
        if self.sheet.get_untracked() != index {
            self.reveal.update(|r| r[index] = dir);
            self.sheet.set(index);
            self.plot.update(|t| *t += 1);
        }
        self.replace_hash(Some(sheet.id));
        self.preview.set(None);
        self.aim(sheet.face);
        self.wake();
    }

    pub fn face_to(&self, face: Face, dir: Direction) {
        match sheet_of_face(face) {
            Some(sheet) => self.go(sheet_index(sheet.id).unwrap_or(0), dir),
            None => {
                self.preview.set(None);
                self.aim(face);
                self.wake();
            }
        }
    }

    pub fn aim(&self, face: Face) {
        let from = self.orient.get_untracked();
        self.goal.set(cube::snap_to(from, face));
    }

    pub fn step(&self, forward: bool) {
        let n = SHEETS.len();
        let i = self.sheet.get_untracked();
        let next = if forward {
            (i + 1) % n
        } else {
            (i + n - 1) % n
        };
        self.go(
            next,
            if forward {
                Direction::Left
            } else {
                Direction::Right
            },
        );
    }

    pub fn preview_dir(&self, dir: Direction) {
        let from = self.orient.get_untracked();
        let face = cube::Cube::new(from).neighbour(dir);
        self.preview.set(Some(face));
        self.goal.set(cube::snap_to(from, face));
        self.wake();
    }

    pub fn commit(&self) {
        if let Some(face) = self.preview.get_untracked() {
            self.face_to(face, Direction::Left);
        }
    }

    pub fn escape(&self) {
        if self.preview.get_untracked().is_some() {
            self.preview.set(None);
            self.aim(sheet_at(self.sheet.get_untracked()).face);
        } else if self.open.get_untracked().is_some() {
            self.close_record();
        }
    }

    pub fn toggle_motion(&self) {
        let next = match self.motion.get_untracked() {
            Motion::Plot => Motion::Mech,
            Motion::Mech => Motion::Plot,
        };
        self.motion.set(next);
        if let Some(node) = document_opt().and_then(|d| d.document_element()) {
            let _ = node.set_attribute("data-motion", next.key());
        }
        self.announce(&format!(
            "{} {}. {}",
            locale::t_untracked("nav.motion.register"),
            next.label(),
            match next {
                Motion::Plot => locale::t_untracked("nav.motion.plot"),
                Motion::Mech => locale::t_untracked("nav.motion.mech"),
            }
        ));
        self.plot.update(|t| *t += 1);
    }

    pub fn open_record(&self, id: &'static str) {
        if self.open.get_untracked() == Some(id) {
            return;
        }
        self.closing.set(false);
        self.open.set(Some(id));
        set_inert("frame", true);
        set_inert("dock", true);
        let title = self
            .content()
            .project(id)
            .map(|p| locale::t_untracked(p.title))
            .unwrap_or_else(|| locale::t_untracked("nav.record.fallback"));
        let no = sheet_at(self.sheet.get_untracked()).no;
        self.announce(&format!(
            "{title} {} {no}. {}",
            locale::t_untracked("nav.record.open"),
            locale::t_untracked("nav.record.open_hint")
        ));
        self.replace_hash(Some(id));
    }

    pub fn dismiss_record(&self) {
        let Some(was) = self.open.get_untracked() else {
            return;
        };
        self.closing.set(false);
        self.open.set(None);
        set_inert("frame", false);
        set_inert("dock", false);
        let opener = self
            .content()
            .project(was)
            .map(|p| format!("open-{}", p.no))
            .unwrap_or_default();
        if let Some(node) = el(&opener) {
            if let Some(node) = node.dyn_ref::<web_sys::HtmlElement>() {
                let _ = node.focus();
            }
        }
    }

    pub fn close_record(&self) {
        if self.open.get_untracked().is_none() || self.closing.get_untracked() {
            return;
        }
        self.closing.set(true);
        if reduced_motion() {
            self.finish_record_close();
        }
    }

    pub fn finish_record_close(&self) {
        if !self.closing.get_untracked() {
            return;
        }
        let Some(was) = self.open.get_untracked() else {
            return;
        };
        self.dismiss_record();
        self.replace_hash(Some(sheet_at(self.sheet.get_untracked()).id));
        if let Some(project) = self.content().project(was) {
            self.announce(&format!(
                "{} {}.",
                locale::t_untracked(project.title),
                locale::t_untracked("nav.record.closed")
            ));
        }
    }

    pub fn state_line(&self, terse: bool) -> String {
        if let Some(id) = self.open.get() {
            let title = self
                .content()
                .project(id)
                .map(|p| locale::t(p.title))
                .unwrap_or_default();
            let title = title.to_uppercase();
            return format!(
                "{title} {}",
                if terse {
                    locale::t("nav.state.record_open")
                } else {
                    locale::t("nav.state.record_open_hint")
                }
            );
        }
        if let Some(face) = self.preview.get() {
            let (no, name) = match sheet_of_face(face) {
                Some(s) => (s.no, locale::t(s.title).to_uppercase()),
                None => (face.no(), "\u{2014}".to_string()),
            };
            let preview = locale::t("nav.state.preview");
            return match (terse, sheet_of_face(face).is_some()) {
                (false, true) => format!(
                    "{preview} {no} {name} \u{00b7} {}",
                    locale::t("nav.state.enter_commits")
                ),
                (true, true) => format!("{preview} {no} {name} \u{00b7} ENTER"),
                (false, false) => format!(
                    "{preview} {no} \u{2014} {} \u{00b7} {}",
                    locale::t("nav.state.no_sheet"),
                    locale::t("nav.state.enter_holds")
                ),
                (true, false) => format!(
                    "{preview} {no} \u{2014} {}",
                    locale::t("nav.state.no_sheet")
                ),
            };
        }
        match (terse, self.motion.get()) {
            (false, Motion::Plot) => locale::t("nav.state.drag_orbit"),
            (false, Motion::Mech) => locale::t("nav.state.drag_orbit_mech"),
            (true, _) => locale::t("nav.state.drag_orbit_terse"),
        }
    }

    fn replace_hash(&self, id: Option<&str>) {
        let want = id.map(|id| format!("#{id}")).unwrap_or_default();
        let have = window_opt()
            .and_then(|w| w.location().hash().ok())
            .unwrap_or_default();
        if have == want {
            return;
        }
        let Some(hist) = window_opt().and_then(|w| w.history().ok()) else {
            return;
        };
        let _ = hist.replace_state_with_url(&JsValue::NULL, "", Some(&want));
    }

    pub fn boot_from_hash(&self) {
        let hash = window_opt()
            .and_then(|w| w.location().hash().ok())
            .unwrap_or_default();
        let id = hash.trim_start_matches('#');
        let index = sheet_index(id);
        match index {
            Some(i) => {
                self.sheet.set(i);
                self.settle_cube(i);
            }
            None if !id.is_empty() && self.content().project(id).is_some() => {
                /* a record opens over the work sheet */
                let i = sheet_index("p-work").unwrap_or(1);
                self.sheet.set(i);
                self.settle_cube(i);
                if let Some(project) = self.content().project(id) {
                    self.open.set(Some(project.id));
                    set_inert("frame", true);
                    set_inert("dock", true);
                }
            }
            None => {
                self.sheet.set(0);
                self.settle_cube(0);
            }
        }
        self.booted.set(true);
        self.plot.update(|t| *t += 1);
        self.wake();
    }

    fn settle_cube(&self, index: usize) {
        let m = cube::face_matrix(sheet_at(index).face, IDENTITY);
        self.orient.set(m);
        self.goal.set(m);
    }

    pub fn wake(&self) {
        self.rest.set(false);
    }

    pub fn announce(&self, text: &str) {
        if let Some(node) = el("live") {
            node.set_text_content(Some(text));
        }
    }
}

fn set_inert(id: &str, on: bool) {
    if let Some(node) = el(id) {
        if on {
            let _ = node.set_attribute("inert", "");
        } else {
            let _ = node.remove_attribute("inert");
        }
    }
}

pub fn plot_active(nav: Nav) {
    let id = sheet_at(nav.sheet.get_untracked()).id;
    let Some(panel) = el(id) else { return };
    for sheet in SHEETS.iter() {
        if sheet.id != id {
            if let Some(other) = el(sheet.id) {
                js::clear_plot(&other);
            }
        }
    }
    if nav.motion.get_untracked() == Motion::Mech || reduced_motion() {
        js::clear_plot(&panel);
    } else {
        js::plot_sheet(&panel);
    }
}

pub fn measure_scroll(nav: Nav) {
    let mut next = [false; SHEETS.len()];
    for (i, sheet) in SHEETS.iter().enumerate() {
        if let Some(body) = el(sheet.id).and_then(|p| p.query_selector(".p-body").ok().flatten()) {
            next[i] = body.scroll_height() - body.client_height() > 4;
        }
    }
    nav.scrolls.set(next);
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use crate::libs::content::get_content;

    #[test]
    fn closing_keeps_the_record_open_until_animation_completion() {
        let owner = Owner::new();
        owner.with(|| {
            let nav = Nav::new(get_content());
            nav.open.set(Some("proj-1"));
            nav.close_record();
            nav.close_record();
            assert!(nav.closing.get_untracked());
            assert_eq!(nav.open.get_untracked(), Some("proj-1"));
            nav.finish_record_close();
            assert!(!nav.closing.get_untracked());
            assert_eq!(nav.open.get_untracked(), None);

            nav.open.set(Some("proj-2"));
            nav.close_record();
            nav.go(0, Direction::Left);
            nav.open.set(Some("proj-3"));
            nav.finish_record_close();
            assert_eq!(nav.open.get_untracked(), Some("proj-3"));
        });
    }

    #[test]
    fn navigating_from_a_record_clears_modal_state_even_on_the_same_sheet() {
        let owner = Owner::new();
        owner.with(|| {
            let nav = Nav::new(get_content());
            let work = sheet_index("p-work").unwrap();
            nav.sheet.set(work);
            nav.open.set(Some("proj-1"));
            nav.go(work, Direction::Left);
            assert_eq!(nav.open.get_untracked(), None);
            assert_eq!(nav.sheet.get_untracked(), work);

            nav.open.set(Some("proj-1"));
            nav.go(0, Direction::Right);
            assert_eq!(nav.open.get_untracked(), None);
            assert_eq!(nav.sheet.get_untracked(), 0);
        });
    }
}
