use once_cell::sync::Lazy;

pub struct OpenGraph {
    pub title: String,
    pub description: String,
    pub image: String,
    pub type_: String,
    pub url: String,
}

pub struct TwitterCard {
    pub title: String,
    pub description: String,
    pub image: String,
    pub card: String,
    pub site: String,
}

pub struct Metadata {
    pub title: String,
    pub description: String,
    pub image: String,
    pub keywords: Vec<String>,
    pub authors: Vec<String>,
    pub open_graph: OpenGraph,
    pub twitter_card: TwitterCard,
}

pub static METADATA: Lazy<Metadata> = Lazy::new(|| Metadata {
    title: "Snootic | Portfolio".to_string(),
    description: "A showcase of my projects and experience.".to_string(),
    image: "/assets/og-image.png".to_string(),
    keywords: vec!["portfolio".to_string(), "projects".to_string(), "rust".to_string(), "typescript".to_string(), "react".to_string(), "web development".to_string(), "three.js".to_string()],
    authors: vec!["Snootic".to_string(), "Kaik Mendes".to_string()],
    open_graph: OpenGraph {
        title: "Snootic | Portfolio".to_string(),
        description: "A showcase of my projects and experience.".to_string(),
        image: "/assets/og-image.png".to_string(),
        type_: "website".to_string(),
        url: "https://snootic.com.br".to_string(),
    },
    twitter_card: TwitterCard {
        title: "Snootic | Portfolio".to_string(),
        description: "A showcase of my projects and experience.".to_string(),
        image: "/assets/twitter-image.png".to_string(),
        card: "summary_large_image".to_string(),
        site: "@snootic_".to_string(),
    },
});