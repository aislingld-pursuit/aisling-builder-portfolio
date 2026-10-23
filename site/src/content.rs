//! Everything the site says, as data. Layout lives in main.rs; facts live here, so a
//! new build is one entry in `BUILDS` rather than edits in five places.
//!
//! Sources: the L2 portfolio READMEs, the résumé, and the capstone handoff. Numbers are
//! the measured ones from those documents, with the date they were measured where the
//! source gives one. Nothing here is an estimate dressed as a measurement.

#[derive(PartialEq)]
pub struct Fig {
    pub v: &'static str,
    pub k: &'static str,
}

#[derive(PartialEq)]
pub struct Section {
    pub title: &'static str,
    pub lead: &'static str,
    pub points: &'static [&'static str],
}

#[derive(PartialEq)]
pub struct Link {
    pub label: &'static str,
    pub href: &'static str,
}

#[derive(PartialEq)]
pub struct Person {
    pub name: &'static str,
    pub handle: &'static str,
    pub role: &'static str,
}

#[derive(PartialEq)]
pub struct Media {
    /// Paths relative to the site root.
    pub video: &'static str,
    pub poster: &'static str,
    pub alt: &'static str,
    pub w: u32,
    pub h: u32,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Group {
    /// Rust, and a guarantee the program makes.
    Integrity,
    /// Team and client work.
    Team,
}

#[derive(PartialEq)]
pub struct Build {
    pub slug: &'static str,
    pub name: &'static str,
    pub kicker: &'static str,
    pub year: &'static str,
    pub group: Group,
    /// One line, used on cards and in the page description.
    pub line: &'static str,
    /// The single hardest number, set as a readout on cards.
    pub stat: &'static str,
    pub role: &'static str,
    pub team: &'static [Person],
    pub status: &'static str,
    pub figs: &'static [Fig],
    pub stack: &'static [&'static str],
    pub links: &'static [Link],
    pub media: Option<Media>,
    pub sections: &'static [Section],
    /// A shell command a reader can run to check a claim themselves.
    pub verify: Option<(&'static str, &'static str)>,
    pub note: Option<&'static str>,
    pub featured: bool,
}

pub const BUILDS: &[Build] = &[
    Build {
        slug: "housecheck",
        name: "HouseCheck",
        kicker: "Pursuit L2 · Cycle 4 capstone",
        year: "2026",
        group: Group::Integrity,
        line: "Carfax for NYC apartments: a Building Health Card scored from public city data, \
               with an export a stranger can verify offline.",
        stat: "149 tests · Ed25519 · live",
        role: "Lead builder · 192 of 201 commits",
        team: &[Person { name: "Antonin Lesov", handle: "antoninlesov", role: "frontend" }],
        status: "Live",
        figs: &[
            Fig { v: "149", k: "tests, clippy clean" },
            Fig { v: "250", k: "buildings served live" },
            Fig { v: "26,343", k: "violations stored" },
            Fig { v: "2.51 MB", k: "whole data artifact" },
            Fig { v: "137–157 ms", k: "search, warm" },
            Fig { v: "9.89×", k: "description compression" },
        ],
        stack: &["Rust", "axum", "SQLite", "ed25519-dalek", "sha2", "TypeScript", "Docker", "Fly.io", "Vercel"],
        links: &[
            Link { label: "Live demo", href: "https://housecheck-wine.vercel.app" },
            Link { label: "Source", href: "https://github.com/nessaisling-lab/housecheck" },
            Link {
                label: "Export write-up",
                href: "https://github.com/nessaisling-lab/pursuit-l2-portfolio/tree/main/cycle-4-housecheck#the-export-is-the-hard-part",
            },
        ],
        media: Some(Media {
            video: "assets/work/housecheck/workflow.mp4",
            poster: "assets/work/housecheck/workflow-poster.jpg",
            alt: "HouseCheck: searching an address through to a Building Health Card",
            w: 440,
            h: 714,
        }),
        sections: &[
            Section {
                title: "What it is",
                lead: "Before you sign a New York lease, the information that would tell you whether \
                       the building is maintained already exists, and it is public. It is also spread \
                       across four agency websites in formats designed for clerks. HouseCheck puts it \
                       on one card, scored 0–100 across condition, legal, neighborhood and \
                       accessibility, with every number linked to the dataset it came from.",
                points: &[],
            },
            Section {
                title: "How it works",
                lead: "Six public datasets, one artifact, no database server.",
                points: &[
                    "HPD violations, 311, PLUTO, DOB, DOHMH and Census ACS, ingested with \
                     completeness checks and block-compressed per building, measured at 9.89×.",
                    "A read-only SQLite artifact baked into the container: no database server, no \
                     connection pool, no cold-start query.",
                    "Repair speed is the one measure of behaviour rather than state, with three \
                     states instead of two: a median, “nothing closed since 2023”, or genuinely no \
                     data.",
                ],
            },
            Section {
                title: "The hard part",
                lead: "Signing the export turned out not to be enough.",
                points: &[
                    "Each export carries an append-only hash chain, entry_hash[i] = sha256(entry_hash[i-1] \
                     ++ payload_hash[i]), with the head signed by Ed25519. Source dataset IDs and \
                     retrieval times sit inside the signed region.",
                    "Verification has three outcomes, not two: signed and intact, intact but unsigned, \
                     tampered. Collapsing the middle one would let an unsigned document pass as an \
                     authenticated one.",
                    "I wrote an independent verifier in Python, with a different crypto library and \
                     the document as its only input, and found a forgery path: a row rewritten to \
                     “NO VIOLATIONS OF ANY KIND AT THIS ADDRESS”, the chain recomputed and re-signed \
                     with the forger's own key, verified as intact.",
                    "The fix was publishing the signing key at a stable endpoint, separate from the \
                     documents it signs, so a reader has something independent to compare against.",
                ],
            },
            Section {
                title: "What it doesn't do",
                lead: "Stated here rather than discovered later.",
                points: &[
                    "250 buildings, about 0.1% of the city.",
                    "Class I violations are excluded; 753 records were skipped at ingest, and the \
                     card says so.",
                    "It is a signal, not a legal ruling, and it does not give legal advice.",
                ],
            },
        ],
        verify: Some((
            "Every live figure is served by the app itself. The signing key is published apart from the documents it signs:",
            "curl -s https://housecheck-nessa.fly.dev/meta",
        )),
        note: None,
        featured: true,
    },
    Build {
        slug: "gala-guest-list",
        name: "Gala Guest List Tool",
        kicker: "Pursuit L3 · client capstone",
        year: "2026",
        group: Group::Team,
        line: "One guest list for a NYC nonprofit's annual gala: a staff app, a private sponsor \
               portal, and an iPad check-in that keeps working offline.",
        stat: "150 of 189 PRs · 787 access-rule checks",
        role: "Backend lead & repo admin",
        team: &[
            Person { name: "Natalie Walker", handle: "wsvdesign", role: "product lead, UI/UX" },
            Person { name: "Cornell Robertson", handle: "CodeToTheCore", role: "backend" },
            Person { name: "Antonin Lesov", handle: "antoninlesov", role: "frontend, UI/UX" },
        ],
        status: "Prototype in client review · Demo Day Nov 10, 2026",
        figs: &[
            Fig { v: "150 / 189", k: "merged PRs authored" },
            Fig { v: "~2,375", k: "automated checks per run" },
            Fig { v: "787", k: "access-rule checks" },
            Fig { v: "~24k", k: "lines, TS · SQL · JS · Python" },
            Fig { v: "98", k: "dated decisions logged" },
            Fig { v: "3 · 2", k: "apps · environments" },
        ],
        stack: &[
            "Supabase (Postgres, Auth, Edge Functions)",
            "row-level security",
            "React",
            "TypeScript",
            "Vite",
            "Tailwind",
            "Vitest",
            "Playwright",
            "Cloudflare Pages + Access",
            "Claude Code",
        ],
        links: &[],
        media: None,
        sections: &[
            Section {
                title: "What it is",
                lead: "A nonprofit ran its annual gala across four disconnected sources: ticket \
                       sales, sponsorship pledges paid by check or bank transfer, comp tickets \
                       tracked by individual staff, and a shared spreadsheet. Sponsors emailed guest \
                       names that staff retyped, badges were mail-merged by hand, and the door ran \
                       on a printed list.",
                points: &[],
            },
            Section {
                title: "How it works",
                lead: "Three apps over one Postgres database, built by a team of four.",
                points: &[
                    "Staff app: gala setup, sponsorship levels, a ticketing-platform import, orders \
                     with a payment ledger, tables, badges, a staff-approved email queue, CSV reports \
                     and a change log.",
                    "Sponsor portal: a private link per order, in four languages, where a sponsor \
                     names and edits their guests until the deadline. No account needed.",
                    "iPad check-in: A-to-Z search, QR e-ticket scanning, walk-ins, and check-in that \
                     keeps working when the Wi-Fi drops and syncs when it returns.",
                ],
            },
            Section {
                title: "The hard part",
                lead: "My part was the backend, and most of it is about who can do what.",
                points: &[
                    "Row-level security in Postgres for three staff roles, a sign-in hook that admits \
                     only the staff list, and 6-digit email codes instead of passwords.",
                    "Sponsor links and QR e-tickets on random tokens of at least 128 bits, stored only \
                     as SHA-256 hashes, scoped to one order or one seat, and expiring at the deadline \
                     or after gala night.",
                    "An append-only change log of every edit (who, when, before and after) that only \
                     gala admins can read, and offline sync where the first arrival wins when two \
                     iPads check in the same guest.",
                    "When the team ran out of GitHub Actions minutes, a local CI fast lane: a full \
                     seven-stage run marks the commit on GitHub, so every PR still gets the complete \
                     check.",
                ],
            },
            Section {
                title: "Bugs the testing caught",
                lead: "Click-through tests of every button on a local copy found real defects.",
                points: &[
                    "A QR check-in that admitted the guest without saying who had come in.",
                    "Relationship managers receiving the whole change log, which only gala admins \
                     should read.",
                    "A payment form that let the same check, or an extra zero, through (from a \
                     teammate's review).",
                ],
            },
            Section {
                title: "What it doesn't do",
                lead: "A working prototype, not production.",
                points: &[
                    "Every record in testing and in the walkthrough is invented. Production hosting \
                     and real data are the client's decision after Demo Day.",
                    "The repository is private and the apps sit behind a sign-in gate, so there is no \
                     public link. Numbers are as of October 2026.",
                ],
            },
        ],
        verify: None,
        note: Some(
            "Screens are frames from the team's walkthrough, on invented data, with the client's \
             name and logo pixelated. The full narrated walkthrough is available on request.",
        ),
        featured: true,
    },
    Build {
        slug: "resona",
        name: "Resona / Wisper",
        kicker: "Pursuit L2",
        year: "2026",
        group: Group::Integrity,
        line: "Privacy-first voice-to-text: 100% on-device via whisper.cpp, so the audio never \
               leaves the machine.",
        stat: "100% on-device · 4 GPU backends",
        role: "Builder · co-developed with Jimmy Ong",
        team: &[Person { name: "Jimmy Ong", handle: "jimmyronin", role: "co-developer" }],
        status: "Installable · v0.2.0-beta.29",
        figs: &[
            Fig { v: "100%", k: "on-device transcription" },
            Fig { v: "4", k: "GPU backends, one core" },
            Fig { v: "3", k: "desktop platforms" },
            Fig { v: "beta.29", k: "latest installer" },
        ],
        stack: &["Rust", "Tauri 2", "whisper.cpp", "whisper-rs", "cpal", "React", "TypeScript", "CUDA · Vulkan · Metal"],
        links: &[
            Link {
                label: "Download v0.2.0-beta.29",
                href: "https://github.com/aislingld-pursuit/L2-Clone-Prodject/releases/tag/v0.2.0-beta.29",
            },
            Link { label: "Source", href: "https://github.com/aislingld-pursuit/L2-Clone-Prodject" },
        ],
        media: Some(Media {
            video: "assets/work/resona/workflow.mp4",
            poster: "assets/work/resona/workflow-poster.jpg",
            alt: "Resona: checking the machine and choosing a speech model size",
            w: 500,
            h: 940,
        }),
        sections: &[
            Section {
                title: "What it is",
                lead: "Desktop dictation, live and from files, that runs entirely on the machine. \
                       The privacy claim is checkable by anyone in ten seconds: disconnect the \
                       network and it keeps transcribing.",
                points: &[],
            },
            Section {
                title: "How it works",
                lead: "Dictation is the one input that is inherently sensitive.",
                points: &[
                    "Microphone capture through cpal, energy-based voice-activity detection, then \
                     incremental Whisper, so text appears as you speak.",
                    "One Rust core and one codebase for CUDA, Vulkan, Metal and CPU; Tauri 2 builds \
                     every target from the same source.",
                    "Tier gating lives in the Rust core with Ed25519-signed licences, so an \
                     entitlement check is not a frontend suggestion.",
                ],
            },
            Section {
                title: "The hard part",
                lead: "Making the privacy claim testable rather than promised.",
                points: &[
                    "Mainstream dictation streams audio to someone else's server, with a privacy \
                     policy as the only thing between the recording and a training set.",
                    "On first run it profiles the machine and recommends a model size, instead of \
                     making the user guess between tiny and large.",
                    "A security pass found and fixed an SSRF path and an unsafe export (SEC-001, \
                     SEC-002).",
                ],
            },
            Section {
                title: "What it doesn't do",
                lead: "One defect kept visible on purpose.",
                points: &[
                    "The hardware probe reports 32041 GB RAM on a 32 GB machine: megabytes labelled as \
                     gigabytes. It stays documented rather than cropped out, because it is the class \
                     of bug that survives by looking like a big impressive number.",
                ],
            },
        ],
        verify: None,
        note: None,
        featured: true,
    },
    Build {
        slug: "nisaba",
        name: "Nisaba Capital Charting",
        kicker: "Founder · solo build",
        year: "2026",
        group: Group::Integrity,
        line: "A native-Rust financial-intelligence terminal that pairs market data with an \
               astrology engine validated to sub-arcsecond precision.",
        stat: "~25k lines of Rust · 132 tests",
        role: "Founder & sole builder",
        team: &[],
        status: "Windows installer v12.2.0 · pre-revenue",
        figs: &[
            Fig { v: "~25k", k: "lines of Rust, solo, 30 days" },
            Fig { v: "132", k: "tests · 0 warnings" },
            Fig { v: "46", k: "SQLx migrations" },
            Fig { v: "+37.06%", k: "paper-trade ROI, verified" },
        ],
        stack: &["Rust", "Iced", "axum", "SQLx", "Swiss Ephemeris", "WGSL shaders"],
        links: &[
            Link { label: "Download v12.2.0 (Windows)", href: "https://github.com/nessaisling-lab/nisaba-capital-charting/releases/tag/v12.2.0" },
            Link { label: "Source", href: "https://github.com/nessaisling-lab/nisaba-capital-charting" },
            Link { label: "Nisaba brand site", href: "https://nessaisling-lab.github.io/nisaba-site/" },
        ],
        media: None,
        sections: &[
            Section {
                title: "What it is",
                lead: "A desktop terminal that pairs conventional market data with a Hellenistic \
                       astrology engine. The thesis isn't that astrology predicts markets. It's that \
                       strong investors don't dismiss a data source because it's unconventional; \
                       they measure it.",
                points: &[],
            },
            Section {
                title: "How it works",
                lead: "Conventional plumbing, held to the same bar as the unconventional part.",
                points: &[
                    "12 data APIs and 10 native Rust scrapers feeding 46 SQLx migrations.",
                    "An axum REST sidecar beside an Iced desktop UI with WGSL shaders.",
                    "An ephemeris engine validated to sub-arcsecond precision against Swiss Ephemeris \
                     and JPL data.",
                ],
            },
            Section {
                title: "What it doesn't do",
                lead: "Paper trading only.",
                points: &[
                    "The +37.06% is verified paper-trade ROI, not real money and not a forecast.",
                    "Pre-revenue. Presented at Pursuit Demo Day on May 6, 2026, to Pursuit staff, \
                     fellow Builders and a Fortune 500 industry reviewer.",
                ],
            },
        ],
        verify: None,
        note: None,
        featured: false,
    },
    Build {
        slug: "siteassure",
        name: "SiteAssure",
        kicker: "Pursuit L2 · Cycle 2",
        year: "2026",
        group: Group::Integrity,
        line: "A tamper-evident, offline-first OSHA compliance logger, driven by voice.",
        stat: "18 commands · zero stubs · 3-OS CI",
        role: "Builder",
        team: &[Person { name: "Jimmy Ong", handle: "jimmyronin", role: "co-author of the clone it grew from" }],
        status: "Feature-complete · build from source",
        figs: &[
            Fig { v: "18", k: "commands, zero stubs" },
            Fig { v: "100%", k: "audit integrity" },
            Fig { v: "3", k: "OS CI matrix" },
        ],
        stack: &["Rust", "cross-platform desktop", "GitHub Actions"],
        links: &[Link { label: "Source (siteassure/)", href: "https://github.com/nessaisling-lab/L2-C2-Solution" }],
        media: Some(Media {
            video: "assets/work/siteassure/workflow.mp4",
            poster: "assets/work/siteassure/workflow-poster.jpg",
            alt: "SiteAssure: daily log, records, trade-risk dashboard and audit trail",
            w: 436,
            h: 918,
        }),
        sections: &[
            Section {
                title: "What it is",
                lead: "A construction site's safety record is a legal document before it is anything \
                       else. SiteAssure keeps it append-only: every create, amend and delete is \
                       chained and hash-verified, so nothing is silently altered or removed.",
                points: &[],
            },
            Section {
                title: "How it works",
                lead: "Each entry's hash includes the one before it.",
                points: &[
                    "Editing any historical record breaks every hash after it, so tampering is \
                     detectable by anyone holding the file, not only whoever runs the server.",
                    "Network egress is cut to an explicit allowlist rather than a kill-switch, so a \
                     new call site cannot quietly reach the internet by default.",
                    "The same records, read a second way, drive a trade-risk dashboard with no extra \
                     data entry.",
                ],
            },
            Section {
                title: "What it doesn't do",
                lead: "Feature-complete and uninstallable.",
                points: &[
                    "There is no signed installer yet, so running it means building from source. \
                     That is tracked as the next task rather than hidden.",
                    "This is the hash chain HouseCheck's export uses two cycles later. It was proven \
                     here first, on OSHA records.",
                ],
            },
        ],
        verify: None,
        note: None,
        featured: false,
    },
    Build {
        slug: "ziqpu",
        name: "Ziqpu",
        kicker: "Pursuit L2 · Cycle 3",
        year: "2026",
        group: Group::Integrity,
        line: "An astrology decision tool with two visible agents: one measures, one interprets, \
               and the separation is the integrity guarantee.",
        stat: "2 agents · offline ephemeris · no keys",
        role: "Team lead · 3 people",
        team: &[],
        status: "Runs locally",
        figs: &[
            Fig { v: "2", k: "agents, measurer and interpreter" },
            Fig { v: "0", k: "API keys or network needed" },
            Fig { v: "Q6_K", k: "Qwen3 14B, picked by benchmark on this laptop" },
        ],
        stack: &["Rust", "Dioxus", "axum", "Postgres", "MCP", "ANISE / JPL ephemeris", "local models"],
        links: &[Link { label: "Source", href: "https://github.com/nessaisling-lab/Ziqpu-L2-Cycle-3" }],
        media: Some(Media {
            video: "assets/work/ziqpu/workflow.mp4",
            poster: "assets/work/ziqpu/workflow-poster.jpg",
            alt: "Ziqpu: benchmarking the machine and recommending a local model",
            w: 1100,
            h: 798,
        }),
        sections: &[
            Section {
                title: "What it is",
                lead: "It scores your natal chart against the chart of anything with a date of \
                       origin (v1: stocks, by IPO). Measurement and meaning are different programs, \
                       so a language model cannot touch the arithmetic.",
                points: &[],
            },
            Section {
                title: "How it works",
                lead: "The sequence is fixed in code rather than chosen by the model.",
                points: &[
                    "Hamun-ana computes positions and aspects and returns structured JSON only. It \
                     never interprets.",
                    "Ungasaga reads those measures back in three beats (measured, meaning, reminder) \
                     and declines to give advice.",
                    "A read-only axum sidecar and an MCP server sit beside the Dioxus desktop UI.",
                ],
            },
            Section {
                title: "The hard part",
                lead: "An unknown birth time is honest.",
                points: &[
                    "With no birth hour, the app withholds the Ascendant and Midheaven rather than \
                     guessing them.",
                    "It benchmarks the machine and recommends the largest quantisation that actually \
                     fits, and downloads nothing until you say so.",
                ],
            },
            Section {
                title: "What it doesn't do",
                lead: "For reflection, not prediction.",
                points: &[
                    "Not financial, medical, legal or psychological guidance. The interpreter is built \
                     to decline advice rather than disclaim it in a footer.",
                ],
            },
        ],
        verify: None,
        note: None,
        featured: false,
    },
    Build {
        slug: "excalidraw",
        name: "Excalidraw: First-Save Features",
        kicker: "Pursuit L3 · client pitch",
        year: "2026",
        group: Group::Team,
        line: "Two features shipped into Excalidraw's open-source codebase, aimed at one number: new \
               users who save their first drawing.",
        stat: "22 of 27 commits · WCAG AA",
        role: "Builder · 22 of 27 commits on main",
        team: &[
            Person { name: "Jillian Krebsbach", handle: "JillK83", role: "" },
            Person { name: "Lawrence Carrillo", handle: "lawrencecarrillo", role: "" },
            Person { name: "Cornell Robertson", handle: "CodeToTheCore", role: "" },
        ],
        status: "Shipped · live A/B demo",
        figs: &[
            Fig { v: "2", k: "approved features shipped" },
            Fig { v: "6", k: "tagged releases" },
            Fig { v: "3,135", k: "users per arm, sized" },
        ],
        stack: &["TypeScript", "React", "Vite", "Vitest", "GitHub Actions", "CodeQL"],
        links: &[
            Link { label: "Live demo", href: "https://nessaisling-lab.github.io/Excalidraw-L3-Cycle-2-Features-Pitch/?arm=nudge" },
            Link { label: "Source", href: "https://github.com/nessaisling-lab/Excalidraw-L3-Cycle-2-Features-Pitch" },
        ],
        media: None,
        sections: &[
            Section {
                title: "What it is",
                lead: "A client pitch built as working code inside Excalidraw itself: a one-time \
                       first-save prompt, and a persistent “Save to file” button with a format \
                       picker.",
                points: &[],
            },
            Section {
                title: "How it works",
                lead: "Both arms of the experiment run in one live demo.",
                points: &[
                    "The demo switches between control and treatment arms from the URL, so a reviewer \
                     can see both without a build.",
                    "CI with CodeQL, a GitHub Pages demo, and every colour pair measured to WCAG AA.",
                ],
            },
            Section {
                title: "What it doesn't do",
                lead: "No result is claimed.",
                points: &[
                    "The test was sized at 3,135 users per arm, but it never ran on real traffic, so \
                     there is no KPI result to report.",
                ],
            },
        ],
        verify: None,
        note: None,
        featured: false,
    },
];

/// Still images for a case study, shown as a captioned gallery.
pub struct Still {
    pub slug: &'static str,
    pub src: &'static str,
    pub alt: &'static str,
    pub caption: &'static str,
}

/// The capstone stills are frames from the team's walkthrough recording, on invented
/// data, with the client's name and logo pixelated (every OCR hit plus the logo
/// regions, then re-checked by OCR). The e-ticket email frame was left out: its body
/// names the client in text too small to redact reliably.
pub const GALLERY: &[Still] = &[
    Still {
        slug: "gala-guest-list",
        src: "assets/work/gala-guest-list/06-record-a-payment.webp",
        alt: "Staff app: recording a $12,500 check from an invented sponsor, Acme Community \
              Partners, with the check number and deposit date. Client name and logo pixelated.",
        caption: "A sponsor's check is recorded: amount, check number, deposit date. No card or bank details, ever.",
    },
    Still {
        slug: "gala-guest-list",
        src: "assets/work/gala-guest-list/09-sponsor-portal.webp",
        alt: "Sponsor portal: a sponsor's private page with open guest seats to fill and a \
              balance of $12,500 of $25,000. Client logo pixelated.",
        caption: "The sponsor's private link: name your guests, see what's paid. No account needed.",
    },
    Still {
        slug: "gala-guest-list",
        src: "assets/work/gala-guest-list/19-ipad-qr-check-in.webp",
        alt: "iPad check-in: a scanned QR ticket confirms Sam Ortega is checked in, with an Undo \
              button, above an A-to-Z guest list.",
        caption: "At the door: the scan says who came in, and Undo is one tap away.",
    },
    Still {
        slug: "gala-guest-list",
        src: "assets/work/gala-guest-list/22-live-arrivals-dashboard.webp",
        alt: "Staff dashboard: commitments, money received, seats named and arrivals, with \
              follow-ups listed by sponsor.",
        caption: "After the doors close: money, seats and arrivals on one screen.",
    },
];

/// Earlier work, listed on the Work page without a case study of its own.
pub struct Earlier {
    pub name: &'static str,
    pub line: &'static str,
    pub href: &'static str,
}

pub const EARLIER: &[Earlier] = &[
    Earlier {
        name: "Berkshire Hathaway, Reimagined",
        line: "UX/UI redesign on seven paired principles; WCAG 2.2 AA; an 8-section design rationale.",
        href: "https://nessaisling-lab.github.io/Final-Berkshire-Hathaway-redesign/",
    },
    Earlier {
        name: "Nuclear Force: Escape Router",
        line: "Built on a fork of Nicolercc's nuclear-shelter-app: NYC blast zones, a shelter finder over \
               17 real sites, evacuation routing, packaged for Windows with Electron.",
        href: "https://github.com/aislingld-pursuit/Nightfall-Win-x64",
    },
    Earlier {
        name: "NYC Showroom: Inventory Tracker",
        line: "Multi-user desktop app over 2,730+ items in Postgres, with roles and a who-did-what audit log.",
        href: "https://github.com/aislingld-pursuit/Inventorix-Unified-Live-Dashboard-Part-4-of-Solutions-",
    },
    Earlier {
        name: "PS3 Debian + custom Wi-Fi driver",
        line: "C and Linux device drivers on a PlayStation 3; where I learned to bring up unfamiliar systems.",
        href: "",
    },
];

pub struct Rule {
    pub head: &'static str,
    pub body: &'static str,
}

pub const RULES: &[Rule] = &[
    Rule {
        head: "Measure, don't estimate.",
        body: "Figures in my repos are measured and say so. Anything derived is labelled derived; \
               anything unverified says unverified.",
    },
    Rule {
        head: "Verify against production, not the repo.",
        body: "Most real defects I have found were invisible from inside the editor: a search that \
               answered a Manhattan address with a Brooklyn building, a failed lookup that kept the \
               previous result on screen.",
    },
    Rule {
        head: "An absent answer is still an answer.",
        body: "The honest design often has three states, not two: signed, unsigned but intact, \
               tampered. Collapsing the middle one is how software states something reassuring it \
               has not earned.",
    },
];

pub struct Job {
    pub when: &'static str,
    pub org: &'static str,
    pub role: &'static str,
    pub body: &'static str,
}

pub const JOBS: &[Job] = &[
    Job {
        when: "Sep 2026 – now",
        org: "Pursuit client capstone (NYC nonprofit)",
        role: "Backend lead & repo admin",
        body: "Gala guest list, sponsor portal and offline iPad check-in on a team of four. Postgres \
               row-level security, hashed single-purpose tokens, an append-only change log; 150 of the \
               team's 189 merged pull requests.",
    },
    Job {
        when: "Apr 2026 – now",
        org: "Nisaba Capital Charting",
        role: "Founder & builder",
        body: "A ~25k-line native-Rust financial-intelligence terminal, built solo in 30 days; 132 \
               tests, zero warnings, Windows installer.",
    },
    Job {
        when: "Sep 2025 – Mar 2026",
        org: "Architechne",
        role: "Data analyst & partnership coordinator",
        body: "A Python + AI scraper and a MySQL territory database for luxury-design lead \
               generation, cutting manual research about 50%.",
    },
    Job {
        when: "Jul 2017 – Jan 2020",
        org: "Airbnb",
        role: "Self-employed property manager",
        body: "Ran a short-term-rental business solo for 2.5 years: listing, marketing, bookkeeping, \
               vendors, guests.",
    },
];

pub const SCHOOLS: &[Job] = &[
    Job {
        when: "Mar 2025 – now",
        org: "Pursuit",
        role: "AI-Native Builder, inaugural cohort",
        body: "600+ hour, 7-month in-person intensive: 10+ end-to-end builds and 8+ PRDs. Client \
               capstone Demo Day: November 10, 2026.",
    },
    Job {
        when: "Oct 2025",
        org: "Per Scholas",
        role: "IT Support",
        body: "CompTIA A+, Google IT Support Professional Certificate, ServiceNow ITSM Fundamentals.",
    },
    Job {
        when: "Coursework complete",
        org: "Vaughn College",
        role: "Aircraft Mechanic, Airframe & Powerplant",
        body: "All technical coursework toward the FAA A&P rating.",
    },
];

pub struct Row {
    pub dt: &'static str,
    pub dd: &'static str,
}

pub const STACK: &[Row] = &[
    Row { dt: "Rust", dd: "axum · SQLx · rusqlite · Tauri 2 · Dioxus · Iced · whisper-rs · cpal" },
    Row { dt: "Integrity", dd: "Ed25519 · SHA-256 hash chains · Postgres row-level security · hashed tokens · gitleaks · egress allowlists" },
    Row { dt: "Web", dd: "TypeScript · React · Vite · Tailwind · Supabase" },
    Row { dt: "Also", dd: "Python · C · C++ · Assembly · SQL (Postgres, MySQL) · WGSL" },
    Row { dt: "AI", dd: "Claude Code · Anthropic + OpenAI APIs · MCP · whisper.cpp · local models · tool-calling with grounding" },
    Row { dt: "Ops", dd: "GitHub admin · Actions (3-OS matrix) · Docker · Fly.io · Vercel · Cloudflare Pages + Access" },
];

pub struct Doc {
    pub name: &'static str,
    pub files: &'static [(&'static str, &'static str)],
}

pub const DOCS: &[Doc] = &[
    Doc {
        name: "Résumé",
        files: &[
            ("PDF · Letter", "assets/docs/resume/resume-letter.pdf"),
            ("PDF · Legal", "assets/docs/resume/resume-legal.pdf"),
            ("Word · Letter", "assets/docs/resume/resume.docx"),
            ("Word · Legal", "assets/docs/resume/resume-legal.docx"),
            ("Plain text", "assets/docs/resume/resume.txt"),
        ],
    },
    Doc {
        name: "Cover letter",
        files: &[
            ("PDF", "assets/docs/cover-letter/cover-letter.pdf"),
            ("Word", "assets/docs/cover-letter/cover-letter.docx"),
        ],
    },
    Doc {
        name: "Berkshire redesign: design rationale",
        files: &[
            ("PDF", "assets/docs/berkshire-rationale/berkshire-rationale.pdf"),
            ("Word", "assets/docs/berkshire-rationale/berkshire-rationale.docx"),
        ],
    },
    Doc {
        name: "AI and the creator economy: impact report",
        files: &[
            ("PDF", "assets/docs/ai-creator-impact/ai-creator-impact.pdf"),
            ("Word", "assets/docs/ai-creator-impact/ai-creator-impact.docx"),
        ],
    },
    Doc {
        name: "WRC AI presentation",
        files: &[
            ("PDF", "assets/docs/wrc-ai-deck/wrc-ai-deck.pdf"),
            ("PowerPoint", "assets/docs/wrc-ai-deck/wrc-ai-deck.pptx"),
        ],
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn root() -> &'static Path {
        Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/.."))
    }

    /// The first section has to stand on its own: most readers stop after it.
    #[test]
    fn every_case_opens_with_a_standalone_section() {
        for b in BUILDS {
            let first = &b.sections[0];
            assert_eq!(first.title, "What it is", "{} must open with What it is", b.name);
            assert!(first.points.is_empty(), "{} opening section must carry its claim in prose", b.name);
            assert!(b.sections.iter().any(|s| s.title == "What it doesn't do"), "{} hides its limits", b.name);
        }
    }

    /// Home shows exactly three. Two looks thin; four is a list, not a selection.
    #[test]
    fn three_featured_builds() {
        assert_eq!(BUILDS.iter().filter(|b| b.featured).count(), 3);
    }

    #[test]
    fn slugs_are_unique_and_url_safe() {
        for (i, a) in BUILDS.iter().enumerate() {
            assert!(a.slug.chars().all(|c| c.is_ascii_lowercase() || c == '-'), "{}", a.slug);
            for b in &BUILDS[i + 1..] {
                assert_ne!(a.slug, b.slug);
            }
        }
    }

    /// Every local file the site links to has to exist, or a reader gets a 404.
    #[test]
    fn every_local_asset_exists() {
        let mut paths: Vec<&str> = DOCS.iter().flat_map(|d| d.files.iter().map(|f| f.1)).collect();
        for b in BUILDS {
            if let Some(m) = &b.media {
                paths.push(m.video);
                paths.push(m.poster);
            }
        }
        paths.extend(GALLERY.iter().map(|s| s.src));
        for p in paths {
            assert!(root().join(p).is_file(), "missing asset: {p}");
        }
    }

    /// The client is unnamed on the public site until they confirm. A name slipping into
    /// this file would publish it.
    #[test]
    fn client_stays_unnamed() {
        let src = include_str!("content.rs").to_lowercase();
        let banned = [["aa", "fe"].concat(), ["asian ", "americans"].concat()];
        for b in banned {
            assert!(!src.contains(&b), "content.rs names the client");
        }
    }
}
