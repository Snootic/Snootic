use crate::structs::{
    Experience, Personal, PortfolioContent, Project, SkillCategory, Social, Stats,
};

pub fn get_content() -> PortfolioContent {
    PortfolioContent {
        personal: Personal {
            name: "Kaik Mendes",
            initials: "KM",
            title: "Software Developer",
            tagline: "A fellow OpenSUSE user and Persona enjoyer.",
            bio: "I'm a Software Developer graduated from FATEC with 3 years of experience, passionate about optimizing complex systems at every layer. My experience spans developing high-performance 3D editors with React Three Fiber—where I’ve engineered custom collision systems for precision and speed—to building tools in Rust that demand robust memory management.\n\nI also maintain a personal HomeLab, managing infrastructure and deploying scalable solutions with hands-on DevOps and networking. Technically curious, I thrive on transforming underperforming systems into efficient, reliable products.",
            email: "snootic@coisas-mais-estranhas.com.br",
            phone: "+55 (11) 96139-5863",
            location: "São Paulo, Brazil",
            availability: "Available for new projects",
            resume_url: "https://resources.coisas-mais-estranhas.com.br/public/share/i9Aua4CFMyN4sqNtm9vLRQ",
        },
        social: Social {
            github: "https://github.com/snootic",
            linkedin: "https://linkedin.com/in/kaikmen",
            twitter: "https://twitter.com/snootic_",
            website: "https://coisas-mais-estranhas.com.br",
            discord: "@snootic",
        },
        stats: vec![
            Stats {
                value: "3",
                label: "Years of experience",
            },
            Stats {
                value: "12",
                label: "Projects completed",
            },
        ],
        skills: vec![
            SkillCategory {
                category: "Backend",
                items: vec!["Node.js", "Rust", "Python", "C++", "Shell", "PostgreSQL", "Redis"],
            },
            SkillCategory {
                category: "Frontend",
                items: vec!["TypeScript", "Three.js", "React", "Tailwind CSS", "WASM", "Leptos"],
            },
            SkillCategory {
                category: "Infrastructure",
                items: vec!["Docker", "GitHub Actions", "CI/CD", "AWS", "GCP", "Vercel"],
            },
            SkillCategory {
                category: "Tooling",
                items: vec!["Git", "VS Code", "Linux", "Tauri", "Electron"],
            },
        ],
        projects: vec![
            Project {
                id: "1",
                title: "Minecraft Server Builder",
                description: "A web project that lets you configure, build and manage a minecraft server for any version using modrinth's api.",
                long_description: Some("Built with React and Vite, the platform supports gamerules, properties and start file management with a interactive web interface. You can create your own modpack, as well as download any modpack, mod or datapack available at Modrinth."),
                tags: vec!["Typescript", "React", "Tanstack", "Zustand", "Axios"],
                featured: true,
                github: Some("https://github.com/snootic/minecraft-server-builder"),
                demo: Some("https://minecraft.coisas-mais-estranhas.com.br"),
                gradient: "from-[#2C1C0E]/60 via-[#181310] to-[#0D0B09]",
            },
            Project {
                id: "2",
                title: "AI Translator",
                description: "A desktop application that translates files using AI models.",
                long_description: Some("Documents translator app. Translate Word, Excel, Powerpoint and PDF files using AI like DeepL and ChatGPT in instants, from/to any language, without losing any formatting. "),
                tags: vec!["Rust", "Tauri", "Python", "PyO3", "Tokio"],
                featured: true,
                github: Some("https://github.com/snootic/ai-translator"),
                demo: None,
                gradient: "from-[#1E2614]/60 via-[#14180E] to-[#0D0B09]",
            },
            Project {
                id: "3",
                title: "LAB",
                description: "A self-hosted homelab infrastructure built around Docker, Traefik, and Cloudflare, with automated DNS management and container-based service deployment.",
                long_description: Some("Manages multiple self-hosted services behind a Traefik reverse proxy with automatic TLS via Let's Encrypt. Features a Bash daemon that watches Docker events and automatically creates or updates Cloudflare DNS records based on container labels — supporting multiple domains, accounts, and record types (A, AAAA, CNAME, SRV). Uses a macvlan network to give containers real LAN addresses alongside a private bridge network for inter-container communication."),
                tags: vec!["Rust", "Shell", "Bash", "Docker", "Traefik", "Cloudflare API", "DNS", "Network"],
                featured: true,
                github: Some("https://github.com/snootic/lab"),
                demo: None,
                gradient: "from-[#0E2826]/60 via-[#0C1614] to-[#0D0B09]",
            },
            Project {
                id: "4",
                title: "Sphynx",
                description: "An access and flux control system built with ESP32, NFC and biometrics sensors, managed with websockets from a web interface.",
                long_description: None,
                tags: vec!["C++", "Embedded systems", "Adafruit", "Websockets", "PostgreSQL", "Java"],
                featured: false,
                github: Some("https://github.com/cliyo/sphynx"),
                demo: None,
                gradient: "from-[#111A32]/60 via-[#0E1020] to-[#0D0B09]",
            },
            Project {
                id: "5",
                title: "Socket Gallery",
                description: "Multiplayer web game created with React Three fiber and Vite. Explore an art gallery with your friends.",
                long_description: Some("Made as a project on my learning experience period on NewCad, this game lets you create/join rooms connecting to other people. The multiplayer logic was built in Typescript using Socket-IO and Simple-peer, with a room instancing and cache management made with a Redis database."),
                tags: vec!["React Three Fiber", "Three.JS", "React", "Socket-IO", "WebRTC", "Redis", "Game", "Multiplayer"],
                featured: false,
                github: Some("https://github.com/snootic/socket-gallery"),
                demo: Some("https://gallery.coisas-mais-estranhas.com.br"),
                gradient: "from-[#181030]/60 via-[#100C1C] to-[#0D0B09]",
            },
            Project {
                id: "6",
                title: "Playlist Converter",
                description: "Open-source app that connects your spotify and youtube account to convert playlists from one platform to another.",
                long_description: None,
                tags: vec!["TypeScript", "React Native", "Expo", "Spotify-SDK", "Google APIs", "Redis"],
                featured: false,
                github: Some("https://github.com/snootic/playlist-converter"),
                demo: None,
                gradient: "from-[#2A1020]/60 via-[#180C12] to-[#0D0B09]",
            },
        ],
        experience: vec![
            Experience {
                id: "1",
                role: "Mid-Level 3D Developer",
                company: "NewCAD",
                location: "Poá, SP - Brazil | Hybrid",
                period: "Apr 2026 - Present",
                description: vec![
                    "Refactored core system to Domain-Driven Design (DDD) and Clean Architecture, implementing a robust transaction system
to enhance scene history reliability.",
                    "Migrated state management from React Context to Zustand, and database models from Prisma 5 to 7, optimizing system
performance and bundling efficiency.",
                    "Engineered a pixel-precise 2D projection hole feature using Constructive Solid Geometry (CSG) and BVH, maintaining sub-
millisecond calculation speeds.",
                    "Optimized 3D resources using LRU caching algorithms, decreasing memory consumption by over 50% and sustaining 500+ FPS
in multi-million polygon scenes.",
                    "Developed parallel asset management and auto-updater services, eliminating data corruption risks and ensuring zero-downtime
client updates."
                ],
                technologies: vec!["TypeScript", "React", "Three.js", "Electron", "Zustand"],
            },
            Experience {
                id: "2",
                role: "Junior 3D Developer",
                company: "NewCAD",
                location: "Poá, SP - Brazil | Hybrid",
                period: "Sep 2025 - Apr 2026",
                description: vec![
                    "Accelerated 3D rendering speed from 15 FPS to 250 FPS (16x improvement) for high-density environments by implementing
batched and instanced meshes.",
                    "Refactored collision detection using Separating Axis Theorem, improving geometric accuracy and runtime stability.",
                    "Enhanced spatial querying efficiency by 22% by integrating Bounding Volume Hierarchy (BVH) and Separating Axis Theorem
(SAT) collision algorithms.",
                    "Programmed a 9-slice vertex-level scaling architecture, allowing precise geometric resizing without texture distortion.",
                    "Configured CI/CD pipelines via GitHub Actions for automated testing and cross-platform builds (Windows, Linux, macOS).",
                ],
                technologies: vec!["TypeScript", "React", "Three.js", "Electron", "Zustand"],
            },

            Experience {
                id: "3",
                role: "Web Development and DevOps Intern",
                company: "Faculdade Unimais",
                location: "São Paulo, SP - Brazil",
                period: "Jan/2025 - Sep/2025",
                description: vec![
                    "Architected a distributed ElasticSearch cluster using docker, reducing query latency by 99% (8s to 15ms) for a library of over 150,000 PDFs.",
                    "Developed a centralized asynchronous ticketing platform, increasing cross-departmental issue resolution efficiency by 15%.",
                    "Modernized legacy PHP infrastructure to React/React Native platforms, patching critical security vulnerabilities and improving cross-platform compatibility.",
                    "Optimized load balancing, firewall, and DNS configurations to increase system reliability and uptime."
                ],
                technologies: vec!["React", "Vue", "Node.js", "PHP", "MySQL", "Docker", "Elasticsearch", "Linux", "Windows Server"],
            },

            Experience {
                id: "4",
                role: "Software Development Intern",
                company: "The Drake Language Solutions",
                location: "São Paulo, SP - Brazil | Hybrid",
                period: "Jun/2024 - Jan/2025",
                description: vec![
                    "Automated internal workflows with Python and VBA scripts, cutting reporting time by 25%.",
                    "Built an LLM-powered translation tool that reduced document delivery cycles by 30%.",
                    "Optimized data pipelines using Pandas/OpenPyXL for faster processing and storage efficiency.",
                    "Developed multithreaded computer-vision monitoring system for RTSP cameras with motion/object detection and cloud sync."
                ],
                technologies: vec!["Python", "Rust", "VBA", "Pandas", "OpenCV", "RTSP"],
            },
        ],
    }
}
