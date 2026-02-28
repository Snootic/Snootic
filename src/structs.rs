use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub struct Personal {
    pub name: &'static str,
    pub initials: &'static str,
    pub title: &'static str,
    pub tagline: &'static str,
    pub bio: &'static str,
    pub email: &'static str,
    pub phone: &'static str,
    pub location: &'static str,
    pub availability: &'static str,
    pub resume_url: &'static str,
}

#[derive(Debug, Clone)]
pub struct Social {
    pub github: &'static str,
    pub linkedin: &'static str,
    pub twitter: &'static str,
    pub website: &'static str,
    pub discord: &'static str,
}

#[derive(Debug, Clone)]
pub struct Stats {
    pub value: &'static str,
    pub label: &'static str,
}

#[derive(Debug, Clone)]
pub struct SkillCategory {
    pub category: &'static str,
    pub items: Vec<&'static str>,
}

#[derive(Debug, Clone)]
pub struct Project {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub long_description: Option<&'static str>,
    pub tags: Vec<&'static str>,
    pub featured: bool,
    pub github: Option<&'static str>,
    pub demo: Option<&'static str>,
    pub gradient: &'static str,
}

#[derive(Debug, Clone)]
pub struct Experience {
    pub id: &'static str,
    pub role: &'static str,
    pub company: &'static str,
    pub location: &'static str,
    pub period: &'static str,
    pub description: Vec<&'static str>,
    pub technologies: Vec<&'static str>,
}

#[derive(Debug, Clone)]
pub struct PortfolioContent {
    pub personal: Personal,
    pub social: Social,
    pub stats: Vec<Stats>,
    pub skills: Vec<SkillCategory>,
    pub projects: Vec<Project>,
    pub experience: Vec<Experience>,
}