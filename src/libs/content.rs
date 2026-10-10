//! The portfolio's content. Everything here traces to a real source —
//! `../my-experience.md` for the track sheet, the repositories for the records —
//! and where nothing is on file the sheet says so rather than inventing it.
//!
//! Numbers are the owner's measurements; a figure that could drift out of date is
//! derived instead — see [`PortfolioContent::stats`].

use crate::structs::{
    Academy, Chip, Course, Evidence, Link, Personal, Plate, PortfolioContent, Project, Social,
    StackCell, Term,
};

pub fn get_content() -> PortfolioContent {
    PortfolioContent {
        personal: PERSONAL,
        social: SOCIAL,
        languages: LANGUAGES,
        stack: STACK,
        projects: PROJECTS,
        roles: ROLES,
        academy: ACADEMY,
        courses: COURSES,
    }
}

const PERSONAL: Personal = Personal {
    name: "Kaik Mendes",
    initials: "KM",
    title: "content.personal.title",
    tagline: "content.personal.tagline",
    brief: &["content.personal.brief.1", "content.personal.brief.2"],
    email: "snootic@coisas-mais-estranhas.com.br",
    phone: "+55 (11) 96139-5863",
    phone_href: "tel:+5511961395863",
    location: "content.personal.location",
    availability: "content.personal.availability",
    resume: "https://resources.coisas-mais-estranhas.com.br/public/share/i9Aua4CFMyN4sqNtm9vLRQ",
    portrait: "/assets/img_me_1.jpg",
    portrait_alt: "content.personal.portrait_alt",
};

/// The networks the owner can be reached on, in the order the contact sheet draws
/// them. Proper nouns, so none of this is translated.
const SOCIAL: &[Social] = &[
    Social {
        name: "Git",
        handle: "git.coisas-mais-estranhas.com.br/snootic",
        href: Some("https://git.coisas-mais-estranhas.com.br/snootic"),
    },
    Social {
        name: "GitHub",
        handle: "github.com/snootic",
        href: Some("https://github.com/snootic"),
    },
    Social {
        name: "LinkedIn",
        handle: "linkedin.com/in/kaikmen",
        href: Some("https://linkedin.com/in/kaikmen"),
    },
    Social {
        name: "Twitter",
        handle: "twitter.com/snootic_",
        href: Some("https://twitter.com/snootic_"),
    },
    Social {
        name: "Discord",
        handle: "@snootic",
        href: None,
    },
];

/// How the languages are rated, one key each.
const LANGUAGES: &[&str] = &["content.language.1", "content.language.2"];

const ACADEMY: Academy = Academy {
    institution: "Fatec Ferraz de Vasconcelos",
    programme: "content.academy.programme",
    location: "content.academy.location",
    period: "content.academy.period",
    points: &[
        "content.academy.point.1",
        "content.academy.point.2",
        "content.academy.point.3",
    ],
};

/// The chosen courses, newest first as the track sheet lists them. Name and issuer
/// are keys for uniformity; the names are proper nouns and merely round-trip.
const COURSES: &[Course] = &[
    Course {
        name: "content.course.01.name",
        issuer: "content.course.01.issuer",
        year: "2025",
    },
    Course {
        name: "content.course.02.name",
        issuer: "content.course.02.issuer",
        year: "2025",
    },
    Course {
        name: "content.course.03.name",
        issuer: "content.course.03.issuer",
        year: "2024",
    },
    Course {
        name: "content.course.04.name",
        issuer: "content.course.04.issuer",
        year: "2024",
    },
    Course {
        name: "content.course.05.name",
        issuer: "content.course.05.issuer",
        year: "2023",
    },
];

/// The identity sheet's calls to action.
pub const CTAS: [Link; 2] = [
    Link {
        label: "content.cta.work",
        href: "#p-work",
    },
    Link {
        label: "content.cta.track",
        href: "#p-track",
    },
];

/* The stack index. Every entry names what evidences it, or says in drafting
convention that nothing public is on file. Entries with no evidence at all
were dropped, not listed with a shrug: AWS, GCP and Vercel were course
certificates, not shipped work; Tailwind and WASM became evidenced once the
records below could point at Minecraft Server Builder and Mutaria. */
const STACK: &[StackCell] = &[
    StackCell {
        category: "content.stack.languages.category",
        note: "content.stack.languages.note",
        chips: &[
            Chip {
                name: "Rust",
                evidence: Evidence::link("#proj-2", "content.evidence.mutaria_ai_translator"),
            },
            Chip {
                name: "TypeScript",
                evidence: Evidence::link(
                    "#proj-1",
                    "content.evidence.minecraft_server_builder_socket_gallery",
                ),
            },
            Chip {
                name: "Python",
                evidence: Evidence::link("#p-track", "content.evidence.drake_ai_translator"),
            },
            Chip {
                name: "C++",
                evidence: Evidence::link("#proj-6", "content.evidence.sphynx"),
            },
            Chip {
                name: "Shell",
                evidence: Evidence::link("#proj-3", "content.evidence.lab"),
            },
            Chip {
                name: "SQL",
                evidence: Evidence::link("#proj-2", "content.evidence.mutaria_sphynx"),
            },
        ],
    },
    StackCell {
        category: "content.stack.backend.category",
        note: "content.stack.backend.note",
        chips: &[
            Chip {
                name: "Actix Web",
                evidence: Evidence::link("#proj-2", "content.evidence.mutaria"),
            },
            Chip {
                name: "Axum",
                evidence: Evidence::link("#proj-1", "content.evidence.minecraft_server_builder"),
            },
            Chip {
                name: "Node / Express",
                evidence: Evidence::link(
                    "#proj-7",
                    "content.evidence.playlist_converter_socket_gallery",
                ),
            },
            Chip {
                name: "Socket.IO",
                evidence: Evidence::link("#proj-5", "content.evidence.socket_gallery"),
            },
            Chip {
                name: "Tokio",
                evidence: Evidence::link("#proj-2", "content.evidence.mutaria_ai_translator"),
            },
            Chip {
                name: "PyO3",
                evidence: Evidence::link("#proj-4", "content.evidence.ai_translator"),
            },
        ],
    },
    StackCell {
        category: "content.stack.frontend.category",
        note: "content.stack.frontend.note",
        chips: &[
            Chip {
                name: "React",
                evidence: Evidence::link(
                    "#proj-1",
                    "content.evidence.minecraft_server_builder_socket_gallery",
                ),
            },
            Chip {
                name: "React Three Fiber",
                evidence: Evidence::link("#proj-5", "content.evidence.socket_gallery_newcad"),
            },
            Chip {
                name: "Three.js",
                evidence: Evidence::link("#proj-5", "content.evidence.socket_gallery_this_sheet"),
            },
            Chip {
                name: "Leptos",
                evidence: Evidence::link("#proj-2", "content.evidence.mutaria_this_sheet"),
            },
            Chip {
                name: "React Native",
                evidence: Evidence::link("#proj-7", "content.evidence.playlist_converter"),
            },
            Chip {
                name: "WASM",
                evidence: Evidence::link("#proj-2", "content.evidence.mutaria_this_sheet"),
            },
            Chip {
                name: "Tailwind CSS",
                evidence: Evidence::link("#proj-1", "content.evidence.minecraft_server_builder"),
            },
            Chip {
                name: "Vite",
                evidence: Evidence::link(
                    "#proj-1",
                    "content.evidence.minecraft_server_builder_socket_gallery",
                ),
            },
            Chip {
                name: "Zustand",
                evidence: Evidence::link(
                    "#proj-1",
                    "content.evidence.minecraft_server_builder_newcad",
                ),
            },
            Chip {
                name: "TanStack Query",
                evidence: Evidence::link("#proj-1", "content.evidence.minecraft_server_builder"),
            },
        ],
    },
    StackCell {
        category: "content.stack.data.category",
        note: "content.stack.data.note",
        chips: &[
            Chip {
                name: "SQLite",
                evidence: Evidence::link("#proj-2", "content.evidence.mutaria"),
            },
            Chip {
                name: "PostgreSQL",
                evidence: Evidence::link("#proj-6", "content.evidence.sphynx"),
            },
            Chip {
                name: "Redis",
                evidence: Evidence::link(
                    "#proj-5",
                    "content.evidence.socket_gallery_playlist_converter",
                ),
            },
            Chip {
                name: "Elasticsearch",
                evidence: Evidence::link("#p-track", "content.evidence.unimais_search"),
            },
            Chip {
                name: "MySQL",
                evidence: Evidence::link("#p-track", "content.evidence.unimais"),
            },
        ],
    },
    StackCell {
        category: "content.stack.infrastructure.category",
        note: "content.stack.infrastructure.note",
        chips: &[
            Chip {
                name: "Docker Swarm / Compose",
                evidence: Evidence::link(
                    "#proj-3",
                    "content.evidence.lab_mutaria_mc_server_builder",
                ),
            },
            Chip {
                name: "Traefik",
                evidence: Evidence::link("#proj-3", "content.evidence.lab_mutaria"),
            },
            Chip {
                name: "Cloudflare",
                evidence: Evidence::link("#proj-3", "content.evidence.lab_dns"),
            },
            Chip {
                name: "Let's Encrypt",
                evidence: Evidence::link("#proj-3", "content.evidence.lab_tls"),
            },
            Chip {
                name: "WireGuard",
                evidence: Evidence::link("#proj-3", "content.evidence.lab_remote_access"),
            },
            Chip {
                name: "GitHub Actions",
                evidence: Evidence::link("#p-track", "content.evidence.newcad_ci"),
            },
            Chip {
                name: "OpenTelemetry",
                evidence: Evidence::link("#p-track", "content.evidence.newcad_telemetry"),
            },
            Chip {
                name: "Linux",
                evidence: Evidence::link("#proj-3", "content.evidence.lab_unimais"),
            },
        ],
    },
    StackCell {
        category: "content.stack.desktop.category",
        note: "content.stack.desktop.note",
        chips: &[
            Chip {
                name: "Tauri",
                evidence: Evidence::link("#proj-2", "content.evidence.mutaria_ai_translator"),
            },
            Chip {
                name: "Electron",
                evidence: Evidence::link("#p-track", "content.evidence.newcad"),
            },
            Chip {
                name: "Real-ESRGAN / NCNN / Vulkan",
                evidence: Evidence::link("#proj-2", "content.evidence.mutaria_no_cuda"),
            },
            Chip {
                name: "PDFium",
                evidence: Evidence::link("#proj-2", "content.evidence.mutaria_pdf"),
            },
            Chip {
                name: "BVH / SAT",
                evidence: Evidence::link("#p-track", "content.evidence.newcad_picking"),
            },
            Chip {
                name: "CSG",
                evidence: Evidence::link("#p-track", "content.evidence.newcad_openings"),
            },
            Chip {
                name: "ESP32 / PlatformIO",
                evidence: Evidence::link("#proj-6", "content.evidence.sphynx"),
            },
            Chip {
                name: "OpenCV",
                evidence: Evidence::link("#p-track", "content.evidence.drake_rtsp"),
            },
        ],
    },
];

const PROJECTS: &[Project] = &[
    Project {
        no: "01",
        id: "proj-1",
        title: "content.project.01.title",
        brief: "content.project.01.brief",
        detail: "content.project.01.detail",
        tags: &[
            "content.project.01.tag.1",
            "content.project.01.tag.2",
            "content.project.01.tag.3",
            "content.project.01.tag.4",
            "content.project.01.tag.5",
            "content.project.01.tag.6",
            "content.project.01.tag.7",
            "content.project.01.tag.8",
            "content.project.01.tag.9",
        ],
        featured: true,
        kind: "content.project.01.kind",
        source: Some("https://git.coisas-mais-estranhas.com.br/snootic/minecraft-server-builder"),
        live: Some("https://minecraft.coisas-mais-estranhas.com.br"),
        plates: &[
            Plate {
                no: "01",
                label: "content.project.01.plate.1",
                image: Some("/assets/work/mc-mod-browser.webp"),
            },
            Plate {
                no: "02",
                label: "content.project.01.plate.2",
                image: Some("/assets/work/mc-modpack-chosen.webp"),
            },
            Plate {
                no: "03",
                label: "content.project.01.plate.3",
                image: Some("/assets/work/mc-selection-summary.webp"),
            },
            Plate {
                no: "04",
                label: "content.project.01.plate.4",
                image: Some("/assets/work/mc-build-server.webp"),
            },
            Plate {
                no: "05",
                label: "content.project.01.plate.5",
                image: Some("/assets/work/mc-startup-script.webp"),
            },
        ],
    },
    Project {
        no: "02",
        id: "proj-2",
        title: "content.project.02.title",
        brief: "content.project.02.brief",
        detail: "content.project.02.detail",
        tags: &[
            "content.project.02.tag.1",
            "content.project.02.tag.2",
            "content.project.02.tag.3",
            "content.project.02.tag.4",
            "content.project.02.tag.5",
            "content.project.02.tag.6",
            "content.project.02.tag.7",
            "content.project.02.tag.8",
            "content.project.02.tag.9",
            "content.project.02.tag.10",
            "content.project.02.tag.11",
        ],
        featured: true,
        kind: "content.project.02.kind",
        source: Some("https://github.com/snootic/mutaria"),
        live: Some("https://mutaria.coisas-mais-estranhas.com.br"),
        plates: &[
            Plate {
                no: "01",
                label: "content.project.02.plate.1",
                image: Some("/assets/work/mutaria-upscale-done.webp"),
            },
            Plate {
                no: "02",
                label: "content.project.02.plate.2",
                image: Some("/assets/work/mutaria-upscale-running.webp"),
            },
            Plate {
                no: "03",
                label: "content.project.02.plate.3",
                image: Some("/assets/work/mutaria-convert-document.webp"),
            },
            Plate {
                no: "04",
                label: "content.project.02.plate.4",
                image: Some("/assets/work/mutaria-convert-animated.webp"),
            },
            Plate {
                no: "05",
                label: "content.project.02.plate.5",
                image: Some("/assets/work/mutaria-resize.webp"),
            },
        ],
    },
    Project {
        no: "03",
        id: "proj-3",
        title: "content.project.03.title",
        brief: "content.project.03.brief",
        detail: "content.project.03.detail",
        tags: &[
            "content.project.03.tag.1",
            "content.project.03.tag.2",
            "content.project.03.tag.3",
            "content.project.03.tag.4",
            "content.project.03.tag.5",
            "content.project.03.tag.6",
            "content.project.03.tag.7",
            "content.project.03.tag.8",
        ],
        featured: true,
        kind: "content.project.03.kind",
        source: Some("https://git.coisas-mais-estranhas.com.br/snootic/lab"),
        live: None,
        plates: &[
            Plate {
                no: "01",
                label: "content.project.03.plate.1",
                image: None,
            },
            Plate {
                no: "02",
                label: "content.project.03.plate.2",
                image: None,
            },
            Plate {
                no: "03",
                label: "content.project.03.plate.3",
                image: None,
            },
            Plate {
                no: "04",
                label: "content.project.03.plate.4",
                image: None,
            },
        ],
    },
    Project {
        no: "04",
        id: "proj-4",
        title: "content.project.04.title",
        brief: "content.project.04.brief",
        detail: "content.project.04.detail",
        tags: &[
            "content.project.04.tag.1",
            "content.project.04.tag.2",
            "content.project.04.tag.3",
            "content.project.04.tag.4",
            "content.project.04.tag.5",
            "content.project.04.tag.6",
            "content.project.04.tag.7",
            "content.project.04.tag.8",
            "content.project.04.tag.9",
        ],
        featured: false,
        kind: "content.project.04.kind",
        source: Some("https://git.coisas-mais-estranhas.com.br/snootic/ai-translator"),
        live: None,
        plates: &[
            Plate {
                no: "01",
                label: "content.project.04.plate.1",
                image: None,
            },
            Plate {
                no: "02",
                label: "content.project.04.plate.2",
                image: None,
            },
            Plate {
                no: "03",
                label: "content.project.04.plate.3",
                image: None,
            },
            Plate {
                no: "04",
                label: "content.project.04.plate.4",
                image: None,
            },
        ],
    },
    Project {
        no: "05",
        id: "proj-5",
        title: "content.project.05.title",
        brief: "content.project.05.brief",
        detail: "content.project.05.detail",
        tags: &[
            "content.project.05.tag.1",
            "content.project.05.tag.2",
            "content.project.05.tag.3",
            "content.project.05.tag.4",
            "content.project.05.tag.5",
            "content.project.05.tag.6",
            "content.project.05.tag.7",
            "content.project.05.tag.8",
        ],
        featured: false,
        kind: "content.project.05.kind",
        source: Some("https://git.coisas-mais-estranhas.com.br/snootic/socket-gallery"),
        live: Some("https://gallery.coisas-mais-estranhas.com.br"),
        plates: &[
            Plate {
                no: "01",
                label: "content.project.05.plate.1",
                image: None,
            },
            Plate {
                no: "02",
                label: "content.project.05.plate.2",
                image: None,
            },
            Plate {
                no: "03",
                label: "content.project.05.plate.3",
                image: None,
            },
        ],
    },
    Project {
        no: "06",
        id: "proj-6",
        title: "content.project.06.title",
        brief: "content.project.06.brief",
        detail: "content.project.06.detail",
        tags: &[
            "content.project.06.tag.1",
            "content.project.06.tag.2",
            "content.project.06.tag.3",
            "content.project.06.tag.4",
            "content.project.06.tag.5",
            "content.project.06.tag.6",
            "content.project.06.tag.7",
            "content.project.06.tag.8",
        ],
        featured: false,
        kind: "content.project.06.kind",
        source: Some("https://github.com/cliyo/sphynx"),
        live: None,
        plates: &[
            Plate {
                no: "01",
                label: "content.project.06.plate.1",
                image: None,
            },
            Plate {
                no: "02",
                label: "content.project.06.plate.2",
                image: None,
            },
            Plate {
                no: "03",
                label: "content.project.06.plate.3",
                image: None,
            },
            Plate {
                no: "04",
                label: "content.project.06.plate.4",
                image: None,
            },
        ],
    },
    Project {
        no: "07",
        id: "proj-7",
        title: "content.project.07.title",
        brief: "content.project.07.brief",
        detail: "content.project.07.detail",
        tags: &[
            "content.project.07.tag.1",
            "content.project.07.tag.2",
            "content.project.07.tag.3",
            "content.project.07.tag.4",
            "content.project.07.tag.5",
            "content.project.07.tag.6",
            "content.project.07.tag.7",
            "content.project.07.tag.8",
        ],
        featured: false,
        kind: "content.project.07.kind",
        source: Some("https://git.coisas-mais-estranhas.com.br/snootic/playlist-converter"),
        live: Some("https://converter.coisas-mais-estranhas.com.br"),
        plates: &[
            Plate {
                no: "01",
                label: "content.project.07.plate.1",
                image: None,
            },
            Plate {
                no: "02",
                label: "content.project.07.plate.2",
                image: None,
            },
        ],
    },
];

const ROLES: &[Term] = &[
    /* XYZ shape: what was achieved, the measurement that proves it, then the work
    that produced it. Where the owner measured no figure, the measure is the
    observable behaviour rather than an invented number. */
    Term {
        title: "content.role.01.title",
        org: "NewCAD",
        location: Some("content.role.01.location"),
        period: "content.role.01.period",
        points: &[
            "content.role.01.point.1",
            "content.role.01.point.2",
            "content.role.01.point.4",
            "content.role.01.point.3",
            "content.role.01.point.5",
            "content.role.01.point.6",
            "content.role.01.point.7",
            "content.role.01.point.8",
            "content.role.01.point.9",
            "content.role.01.point.10",
        ],
        stack: &[
            "TypeScript",
            "React Three Fiber",
            "Electron",
            "BVH / SAT",
            "Prisma",
            "Ed25519",
            "OpenTelemetry",
            "Grafana",
        ],
    },
    Term {
        title: "content.role.02.title",
        org: "NewCAD",
        location: Some("content.role.02.location"),
        period: "content.role.02.period",
        points: &[
            "content.role.02.point.1",
            "content.role.02.point.2",
            "content.role.02.point.3",
            "content.role.02.point.4",
            "content.role.02.point.5",
        ],
        stack: &[
            "TypeScript",
            "React",
            "Three.js",
            "Zustand",
            "math.js",
            "GitHub Actions",
        ],
    },
    Term {
        title: "content.role.03.title",
        org: "Faculdade Unimais",
        location: Some("content.role.03.location"),
        period: "content.role.03.period",
        points: &[
            "content.role.03.point.1",
            "content.role.03.point.2",
            "content.role.03.point.3",
            "content.role.03.point.4",
        ],
        stack: &[
            "Elasticsearch",
            "Docker Swarm",
            "PHP",
            "React",
            "React Native",
            "WHM / cPanel",
            "Active Directory",
        ],
    },
    Term {
        title: "content.role.04.title",
        org: "The Drake Language Solutions",
        location: Some("content.role.04.location"),
        period: "content.role.04.period",
        points: &[
            "content.role.04.point.1",
            "content.role.04.point.2",
            "content.role.04.point.3",
            "content.role.04.point.4",
        ],
        stack: &[
            "Python",
            "Rust",
            "Tauri",
            "OpenCV",
            "YOLO",
            "Node.js",
            "Google Drive",
        ],
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::SHEETS;

    /* Sheet order, drawing numbers and fragment ids have to agree, and editing the
    content is what quietly breaks them. A broken evidence link claims proof that
    is not there, so the agreement is asserted rather than assumed. */

    #[test]
    fn records_are_numbered_by_their_position() {
        for (i, project) in PROJECTS.iter().enumerate() {
            assert_eq!(
                project.no,
                format!("{:02}", i + 1),
                "{} carries the wrong drawing number",
                project.title
            );
        }
    }

    #[test]
    fn record_ids_are_unique() {
        let mut seen = Vec::new();
        for project in PROJECTS {
            assert!(
                !seen.contains(&project.id),
                "duplicate record id {}",
                project.id
            );
            seen.push(project.id);
        }
    }

    #[test]
    fn featured_records_come_first() {
        let first_archive = PROJECTS
            .iter()
            .position(|p| !p.featured)
            .unwrap_or(PROJECTS.len());
        assert!(PROJECTS[..first_archive].iter().all(|p| p.featured));
        assert!(PROJECTS[first_archive..].iter().all(|p| !p.featured));
    }

    #[test]
    fn every_stack_entry_points_at_something_real() {
        let mut anchors: Vec<String> = PROJECTS.iter().map(|p| format!("#{}", p.id)).collect();
        anchors.extend(SHEETS.iter().map(|s| format!("#{}", s.id)));
        for cell in STACK {
            for chip in cell.chips {
                if let Some(href) = chip.evidence.href {
                    assert!(
                        anchors.iter().any(|a| a == href),
                        "{} points at {href}, which is not a record or a sheet",
                        chip.name
                    );
                }
            }
        }
    }

    #[test]
    fn the_counters_agree_with_the_content() {
        let stats = get_content().stats();
        assert_eq!(stats[1].value, PROJECTS.len().to_string());
        assert_eq!(stats[2].value, ROLES.len().to_string());
    }
}
