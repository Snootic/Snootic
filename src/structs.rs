use crate::libs::locale as loc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Face {
    Front,
    Right,
    Back,
    Left,
    Top,
    Bottom,
}

impl Face {
    pub const ALL: [Face; 6] = [
        Face::Front,
        Face::Right,
        Face::Back,
        Face::Left,
        Face::Top,
        Face::Bottom,
    ];

    pub const fn key(self) -> &'static str {
        match self {
            Face::Front => "front",
            Face::Right => "right",
            Face::Back => "back",
            Face::Left => "left",
            Face::Top => "top",
            Face::Bottom => "bottom",
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Face::Front => "cube.face.front",
            Face::Right => "cube.face.right",
            Face::Back => "cube.face.back",
            Face::Left => "cube.face.left",
            Face::Top => "cube.face.top",
            Face::Bottom => "cube.face.bottom",
        }
    }

    pub const fn no(self) -> &'static str {
        match self {
            Face::Front => "01",
            Face::Right => "02",
            Face::Back => "03",
            Face::Left => "04",
            Face::Top => "05",
            Face::Bottom => "06",
        }
    }

    pub const fn normal(self) -> [f32; 3] {
        match self {
            Face::Front => [0.0, 0.0, 1.0],
            Face::Right => [1.0, 0.0, 0.0],
            Face::Back => [0.0, 0.0, -1.0],
            Face::Left => [-1.0, 0.0, 0.0],
            Face::Top => [0.0, 1.0, 0.0],
            Face::Bottom => [0.0, -1.0, 0.0],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub const fn key(self) -> &'static str {
        match self {
            Direction::Left => "l",
            Direction::Right => "r",
            Direction::Up => "t",
            Direction::Down => "b",
        }
    }

    pub const fn opposite(self) -> Direction {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }

    pub const fn arrow(self) -> &'static str {
        match self {
            Direction::Left => "\u{2190}",
            Direction::Right => "\u{2192}",
            Direction::Up => "\u{2191}",
            Direction::Down => "\u{2193}",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Sheet {
    pub id: &'static str,
    pub no: &'static str,
    pub face: Face,
    pub title: &'static str,
    pub meta: &'static str,
    pub reveal: Direction,
}

pub const SHEETS: [Sheet; 5] = [
    Sheet {
        id: "p-id",
        no: "01",
        face: Face::Front,
        title: "sheets.identity.title",
        meta: "sheets.identity.meta",
        reveal: Direction::Left,
    },
    Sheet {
        id: "p-work",
        no: "02",
        face: Face::Right,
        title: "sheets.work.title",
        meta: "sheets.work.meta",
        reveal: Direction::Left,
    },
    Sheet {
        id: "p-stack",
        no: "03",
        face: Face::Back,
        title: "sheets.stack.title",
        meta: "sheets.stack.meta",
        reveal: Direction::Left,
    },
    Sheet {
        id: "p-track",
        no: "04",
        face: Face::Left,
        title: "sheets.track.title",
        meta: "sheets.track.meta",
        reveal: Direction::Left,
    },
    Sheet {
        id: "p-contact",
        no: "05",
        face: Face::Top,
        title: "sheets.contact.title",
        meta: "sheets.contact.meta",
        reveal: Direction::Up,
    },
];

pub fn sheet_of_face(face: Face) -> Option<&'static Sheet> {
    SHEETS.iter().find(|s| s.face == face)
}

pub fn sheet_at(index: usize) -> &'static Sheet {
    &SHEETS[index % SHEETS.len()]
}

pub fn sheet_index(id: &str) -> Option<usize> {
    SHEETS.iter().position(|s| s.id == id)
}

#[derive(Debug, Clone)]
pub struct Personal {
    pub name: &'static str,
    pub initials: &'static str,
    pub title: &'static str,
    pub tagline: &'static str,
    pub brief: &'static [&'static str],
    pub email: &'static str,
    pub phone: &'static str,
    pub phone_href: &'static str,
    pub location: &'static str,
    pub availability: &'static str,
    pub resume: &'static str,
    pub portrait: &'static str,
    pub portrait_alt: &'static str,
}

#[derive(Debug, Clone)]
pub struct Stat {
    pub value: String,
    pub label: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct Link {
    pub label: &'static str,
    pub href: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct Social {
    pub name: &'static str,
    pub handle: &'static str,
    pub href: Option<&'static str>,
}

#[derive(Debug, Clone, Copy)]
pub struct Evidence {
    pub label: &'static str,
    pub href: Option<&'static str>,
}

impl Evidence {
    pub const fn link(id: &'static str, label: &'static str) -> Evidence {
        Evidence {
            label,
            href: Some(id),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Chip {
    pub name: &'static str,
    pub evidence: Evidence,
}

#[derive(Debug, Clone, Copy)]
pub struct StackCell {
    pub category: &'static str,
    pub note: &'static str,
    pub chips: &'static [Chip],
}

#[derive(Debug, Clone, Copy)]
pub struct Term {
    pub title: &'static str,
    pub org: &'static str,
    pub location: Option<&'static str>,
    pub period: &'static str,
    pub points: &'static [&'static str],
    pub stack: &'static [&'static str],
}

#[derive(Debug, Clone, Copy)]
pub struct Plate {
    pub no: &'static str,
    pub label: &'static str,
    pub image: Option<&'static str>,
}

#[derive(Debug, Clone, Copy)]
pub struct Project {
    pub no: &'static str,
    pub id: &'static str,
    pub title: &'static str,
    pub brief: &'static str,
    pub detail: &'static str,
    pub tags: &'static [&'static str],
    pub featured: bool,
    pub kind: &'static str,
    pub source: Option<&'static str>,
    pub live: Option<&'static str>,
    pub plates: &'static [Plate],
}

impl Project {
    pub const fn status(&self) -> &'static str {
        if self.featured {
            "record.status.featured"
        } else {
            "record.status.archive"
        }
    }

    pub fn paragraphs(&self) -> Vec<String> {
        let text = loc::t(self.detail);
        text.split("\n\n").map(str::to_string).collect()
    }

    pub fn tags_line(&self) -> String {
        self.tags
            .iter()
            .map(|key| loc::t(key))
            .collect::<Vec<_>>()
            .join(" \u{00b7} ")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Course {
    pub name: &'static str,
    pub issuer: &'static str,
    pub year: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct Academy {
    pub institution: &'static str,
    pub programme: &'static str,
    pub location: &'static str,
    pub period: &'static str,
    pub points: &'static [&'static str],
}

#[derive(Debug, Clone)]
pub struct PortfolioContent {
    pub personal: Personal,
    pub social: &'static [Social],
    pub languages: &'static [&'static str],
    pub stack: &'static [StackCell],
    pub projects: &'static [Project],
    pub roles: &'static [Term],
    pub academy: Academy,
    pub courses: &'static [Course],
}

impl PortfolioContent {
    pub const LINE_MAX: usize = 90;

    pub fn stats(&self) -> [Stat; 3] {
        [
            Stat {
                value: "2023".to_string(),
                label: "content.stat.years",
            },
            Stat {
                value: self.projects.len().to_string(),
                label: "content.stat.projects",
            },
            Stat {
                value: self.roles.len().to_string(),
                label: "content.stat.roles",
            },
        ]
    }

    pub fn featured(&self) -> impl Iterator<Item = &Project> {
        self.projects.iter().filter(|p| p.featured)
    }

    pub fn archive(&self) -> impl Iterator<Item = &Project> {
        self.projects.iter().filter(|p| !p.featured)
    }

    pub fn project(&self, id: &str) -> Option<&Project> {
        self.projects.iter().find(|p| p.id == id)
    }

    pub fn readout(
        &self,
        active: Face,
        neighbours: [(Direction, Face); 4],
        state: &str,
        terse: bool,
    ) -> String {
        let mut line = match sheet_of_face(active) {
            Some(s) => format!("{} {}", s.no, loc::t(s.title).to_uppercase()),
            None => format!("{} {}", active.no(), loc::t(active.name()).to_uppercase()),
        };
        for (dir, face) in neighbours {
            line.push_str(" \u{00b7} ");
            line.push_str(dir.arrow());
            match sheet_of_face(face) {
                Some(s) if !terse => {
                    line.push_str(&format!(" {} {}", s.no, loc::t(s.title).to_uppercase()))
                }
                Some(s) => line.push_str(&format!(" {}", s.no)),
                None => line.push_str(&format!(" {} \u{2014}", face.no())),
            }
        }
        line.push_str(" \u{00b7} ");
        line.push_str(state);
        line
    }
}
