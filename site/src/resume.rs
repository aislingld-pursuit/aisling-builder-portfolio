//! The résumé, once. Every format is generated from this file:
//!
//! - `resume.tex` (the fixed preamble in assets/ plus a generated body) → the PDFs
//! - `resume.md`, in career-ops' CV format → txt and html via pandoc, and career-ops' cv.md
//! - `resume.json` → tools/build-resume-docx.js → the Word files
//! - the /resume/ page on the site
//!
//! Before this, the same text lived in three hand-edited files and they drifted. Text
//! here is plain Unicode: "~" means approximately, "–" is a date range, "→" an arrow.
//! Each writer translates those for its format.

pub struct Header {
    pub name: &'static str,
    pub contact: &'static [&'static str],
    pub location: &'static str,
    pub linkedin: (&'static str, &'static str),
    pub github: &'static [(&'static str, &'static str)],
}

#[derive(PartialEq)]
pub struct Entry {
    pub org: &'static str,
    pub role: &'static str,
    /// Right-hand date. Empty for entries whose right column shows the place instead.
    pub when: &'static str,
    pub desc: &'static str,
    pub place: &'static str,
    pub bullets: &'static [&'static str],
    /// One line, no description or bullets: "org | role, place" with the date at right.
    pub compact: bool,
}

pub struct Project {
    pub name: &'static str,
    pub kind: &'static str,
    pub year: &'static str,
    pub desc: &'static str,
    pub url: &'static str,
    pub bullets: &'static [&'static str],
}

pub const HEADER: Header = Header {
    name: "Stan (Aisling) Leiva-Davila",
    contact: &["ness.aisling@gmail.com", "aisling.ld@pursuit.org", "201-471-1178"],
    location: "New York, NY",
    linkedin: ("stan-leiva-davila", "https://www.linkedin.com/in/stan-leiva-davila/"),
    github: &[
        ("nessaisling-lab", "https://github.com/nessaisling-lab"),
        ("aislingld-pursuit", "https://github.com/aislingld-pursuit"),
    ],
};

pub const PORTFOLIO: &str = "https://nessaisling-lab.github.io/aisling-builder-portfolio/";

pub const SUMMARY: &str = "Self-taught software engineer and AI-native builder, Rust-first with a C, \
C++, and Assembly foundation. I build systems whose guarantees can be checked: Ed25519-signed, \
hash-chained exports with an independent verifier; on-device speech-to-text where audio never \
leaves the machine; Postgres row-level security for a nonprofit client. I ship end-to-end, solo and \
on small teams, with Claude Code. Inaugural cohort, Pursuit AI-Native Builder.";

pub const EXPERIENCE: &[Entry] = &[
    Entry {
        org: "Pursuit Client Capstone (NYC Nonprofit)",
        role: "Backend Lead & Repo Admin",
        when: "Sep. 2026 – Present",
        desc: "Gala guest list, private sponsor portal, and offline-first iPad check-in (React + \
               TypeScript on Supabase); team of 4.",
        place: "New York, NY",
        bullets: &[
            "Led the Postgres backend: row-level security for 3 staff roles, an append-only change \
             log, and sponsor links and QR e-tickets on 128-bit random tokens stored only as SHA-256 \
             hashes; 787 access-rule checks in ~2,375 per run",
            "As repo admin, authored 150 of the team's 189 merged PRs, enforced branch protection and \
             gitleaks secret scanning, and built a local CI fast lane that kept full checks on every \
             PR after GitHub Actions minutes ran out",
        ],
        compact: false,
    },
    Entry {
        org: "Nisaba Capital Charting",
        role: "Founder & Builder",
        when: "Apr. 2026 – Present",
        desc: "Native-Rust financial-intelligence desktop terminal merging market data with a \
               Hellenistic astrology engine.",
        place: "New York, NY",
        bullets: &[
            "Built a ~25k-line Rust app solo in 30 days: 12 data APIs, 10 native scrapers, 46 SQLx \
             migrations, an axum sidecar, a sub-arcsecond ephemeris engine; 132 tests, zero \
             warnings, Windows installer",
            "Reached +37.06% verified paper-trade ROI; presented at Pursuit Demo Day to staff, fellow \
             Builders, and a Fortune 500 reviewer",
        ],
        compact: false,
    },
    Entry {
        org: "Architechne",
        role: "Data Analyst & Partnership Coordinator (1099)",
        when: "Sep. 2025 – Mar. 2026",
        desc: "Sourced luxury interior-designer, architect, and residential-developer leads for a NYC \
               design partnership business.",
        place: "New York, NY",
        bullets: &["Built a Python + AI web scraper over NYC tri-state directories and a MySQL \
                    territory database, cutting manual research ~50%"],
        compact: false,
    },
    Entry {
        org: "Airbnb",
        role: "Self-Employed Property Manager (Owner-Operator)",
        when: "Jul. 2017 – Jan. 2020",
        desc: "",
        place: "Flushing, NY",
        bullets: &[],
        compact: true,
    },
];

pub const PROJECTS: &[Project] = &[
    Project {
        name: "HouseCheck",
        kind: "Verifiable Building Records for NYC Renters",
        year: "2026",
        desc: "Building Health Card scored from public NYC data, with an export a stranger can verify \
               offline. Live; 176 tests.",
        url: "https://github.com/nessaisling-lab/housecheck",
        bullets: &[
            "Built a Rust/axum service whose exports carry an append-only SHA-256 hash chain with an \
             Ed25519-signed head and three verification outcomes (signed and intact, intact but \
             unsigned, tampered), with source dataset IDs inside the signed region",
            "Wrote an independent Python verifier and found a forgery path: a rewritten chain \
             re-signed with a forger's own key verified as intact. Closed it by publishing the \
             signing key at a stable endpoint",
        ],
    },
    Project {
        name: "Ziqpu",
        kind: "Multi-Agent Astrology Synastry Engine",
        year: "2026",
        desc: "Scores a natal chart against the chart of anything with a date of origin (v1: stocks by \
               IPO); reflection, not advice.",
        url: "https://github.com/nessaisling-lab/Ziqpu-L2-Cycle-3",
        bullets: &["Led a 3-person team on a Rust workspace: a measurer agent and an interpreter agent, \
                    a read-only axum sidecar, an MCP server, and a Dioxus UI"],
    },
    Project {
        name: "Resona / Wisper",
        kind: "Privacy-First On-Device Voice-to-Text",
        year: "2026",
        desc: "Cross-platform desktop dictation app: live streaming + file/YouTube transcription, 100% \
               on-device via whisper.cpp.",
        url: "https://github.com/aislingld-pursuit/L2-Clone-Prodject",
        bullets: &[
            "Real-time pipeline (cpal → energy-VAD → incremental Whisper) with zero cloud STT and no \
             accounts; CUDA/Vulkan/Metal/CPU backends from one Tauri 2 + Rust codebase",
            "Ed25519-signed licenses with client-vs-server entitlement gating; fixed SSRF and \
             unsafe-export issues; shipped v0.2.0-beta.29 on Windows, macOS, Linux",
        ],
    },
];

pub const EDUCATION: &[Entry] = &[
    Entry {
        org: "Pursuit",
        role: "AI-Native Builder (Inaugural Cohort)",
        when: "Mar. 2025 – Present",
        desc: "600+ hour, 7-month in-person intensive: 10+ end-to-end builds, 8+ PRDs; client capstone \
               Demo Day Nov. 10, 2026.",
        place: "Long Island City, NY",
        bullets: &[],
        compact: false,
    },
    Entry {
        org: "Per Scholas",
        role: "IT Support — CompTIA A+, Google IT Support, ServiceNow ITSM",
        when: "Grad. Oct. 2025",
        desc: "",
        place: "",
        bullets: &[],
        compact: true,
    },
    Entry {
        org: "Vaughn College",
        role: "Aircraft Mechanic Airframe & Powerplant (coursework complete)",
        when: "",
        desc: "",
        place: "Flushing, NY",
        bullets: &[],
        compact: true,
    },
];

pub const SKILLS: &[(&str, &str)] = &[
    ("Languages & Frameworks", "Rust (axum, SQLx, Tauri 2, Dioxus, Iced), Python, C, C++, Assembly, TypeScript/React, SQL (Postgres, MySQL)"),
    ("Security & Integrity", "Ed25519 signatures, SHA-256 hash chains, Postgres row-level security, hashed bearer tokens, gitleaks, SSRF hardening"),
    ("AI & Tooling", "Claude Code, Cursor, Anthropic + OpenAI APIs, MCP, whisper.cpp, RAG, agentic workflows"),
    ("Systems & Ops", "Linux, Windows, GitHub admin (branch protection, CI), Supabase, Cloudflare Pages + Access, Fly.io, CUDA/Vulkan/Metal"),
];

impl Entry {
    /// Left text of a compact line: "role" plus ", place" when the date takes the right.
    pub fn compact_rest(&self) -> String {
        if !self.when.is_empty() && !self.place.is_empty() {
            format!("{}, {}", self.role, self.place)
        } else {
            self.role.to_string()
        }
    }
    pub fn compact_right(&self) -> &'static str {
        if self.when.is_empty() { self.place } else { self.when }
    }
}

// ── LaTeX ────────────────────────────────────────────────────────────────────

fn tex(s: &str) -> String {
    let mut o = String::with_capacity(s.len() + 8);
    for c in s.chars() {
        match c {
            '&' => o.push_str("\\&"),
            '%' => o.push_str("\\%"),
            '#' => o.push_str("\\#"),
            '_' => o.push_str("\\_"),
            '$' => o.push_str("\\$"),
            '~' => o.push_str("$\\sim$"),
            '→' => o.push_str("$\\rightarrow$"),
            '–' => o.push_str("--"),
            '—' => o.push_str("---"),
            _ => o.push(c),
        }
    }
    o
}

pub fn to_tex() -> String {
    let h = &HEADER;
    let contact = h.contact.iter().map(|c| format!("{{{c}}}")).collect::<Vec<_>>().join(" $|$ ");
    let gh = h
        .github
        .iter()
        .map(|(n, u)| format!("  \\faGithub $ $ \\href{{{u}}}{{{n}}}"))
        .collect::<Vec<_>>()
        .join(" $|$\n");
    let mut s = String::from(include_str!("../assets/resume-preamble.tex"));
    s.push_str("%-------------------------------------------\n");
    s.push_str("% GENERATED from site/src/resume.rs. Edit the Rust, then run tools/build-resume.sh.\n");
    s.push_str("%-------------------------------------------\n\n\\begin{document}\n\n");
    s.push_str(&format!(
        "\\begin{{center}}\n  \\textbf{{{{\\Huge {name}}}}} \\\\ \\medskip \\smallskip\n  \
         {{\\large {contact}}} \\\\ \\smallskip\n  {{\\large {loc} $|$\n  \
         \\faLinkedinSquare $ $ \\href{{{lu}}}{{{ln}}} $|$\n{gh}}}\n\\end{{center}}\n\n",
        name = tex(h.name),
        loc = tex(h.location),
        lu = h.linkedin.1,
        ln = h.linkedin.0,
    ));
    s.push_str(&format!("\\section{{Professional Summary}}\n{{\\small {}}}\n\n", tex(SUMMARY)));
    s.push_str(&tex_entries("Experience", EXPERIENCE));

    s.push_str("\\section{Technical Projects}\n  \\resumeSubHeadingListStart\n");
    for (i, p) in PROJECTS.iter().enumerate() {
        s.push_str(&format!(
            "    \\resumeProjectHeading\n      {{{} $|$ {}}}{{{}}}\n      {{{}}}\n      {{{}}}\n",
            tex(p.name), tex(p.kind), p.year, tex(p.desc), p.url
        ));
        s.push_str(&tex_bullets(p.bullets));
        if i + 1 < PROJECTS.len() {
            s.push_str("      \\smallskip\n");
        }
    }
    s.push_str("  \\resumeSubHeadingListEnd\n\n");

    s.push_str(&tex_entries("Education", EDUCATION));

    s.push_str("\\section{Skills \\& Tools}\n \\resumeSubHeadingListStart\n");
    for (k, v) in SKILLS {
        s.push_str(&format!("   \\resumeSubItem{{{}: }}\n     {{{}}}\n", tex(k), tex(v)));
    }
    s.push_str(" \\resumeSubHeadingListEnd\n\n\\end{document}\n");
    s
}

fn tex_bullets(b: &[&str]) -> String {
    if b.is_empty() {
        return String::new();
    }
    let mut s = String::from("      \\resumeItemListStart\n");
    for x in b {
        s.push_str(&format!("        \\resumeItem{{}}\n            {{{}}}\n", tex(x)));
    }
    s.push_str("      \\resumeItemListEnd\n");
    s
}

fn tex_entries(title: &str, list: &[Entry]) -> String {
    let mut s = format!("\\section{{{title}}}\n  \\resumeSubHeadingListStart\n\n");
    for (i, e) in list.iter().enumerate() {
        if e.compact {
            s.push_str(&format!(
                "    \\resumeSubheadingShort\n      {{{} $|$ {}}}{{{}}}\n\n",
                tex(e.org), tex(&e.compact_rest()), tex(e.compact_right())
            ));
            continue;
        }
        s.push_str(&format!(
            "    \\resumeSubheadingEd\n      {{{} $|$ {}}}{{{}}}\n      {{{}}}{{{}}}\n",
            tex(e.org), tex(e.role), tex(e.when), tex(e.desc), tex(e.place)
        ));
        s.push_str(&tex_bullets(e.bullets));
        if i + 1 < list.len() {
            s.push_str("      \\smallskip\n");
        }
        s.push('\n');
    }
    s.push_str("  \\resumeSubHeadingListEnd\n\n");
    s
}

// ── Markdown, in career-ops' CV format ───────────────────────────────────────

pub fn to_md() -> String {
    let h = &HEADER;
    let mut s = format!("# {}\n\n", h.name);
    s.push_str(&format!("**Location:** {}\n", h.location));
    s.push_str(&format!("**Email:** {}\n", h.contact[0]));
    s.push_str(&format!("**Phone:** {}\n", h.contact[2]));
    s.push_str(&format!("**LinkedIn:** linkedin.com/in/{}\n", h.linkedin.0));
    s.push_str(&format!("**Portfolio:** {}\n", PORTFOLIO.trim_start_matches("https://").trim_end_matches('/')));
    let gh = h.github.iter().map(|(n, _)| format!("github.com/{n}")).collect::<Vec<_>>().join(" · ");
    s.push_str(&format!("**GitHub:** {gh}\n\n"));
    s.push_str(&format!("## Professional Summary\n\n{SUMMARY}\n\n## Work Experience\n\n"));
    for e in EXPERIENCE {
        s.push_str(&format!("### {} -- {}\n\n**{}**\n{}\n\n", e.org, e.place, e.role, e.when));
        if !e.desc.is_empty() {
            s.push_str(&format!("- {}\n", e.desc.trim_end_matches('.')));
        }
        for b in e.bullets {
            s.push_str(&format!("- {b}\n"));
        }
        s.push('\n');
    }
    s.push_str("## Projects\n\n");
    for p in PROJECTS {
        let more = p.bullets.join(". ");
        s.push_str(&format!("- **{}: {}** ({}) -- {} {}\n", p.name, p.kind, p.url, p.desc, more));
    }
    s.push_str("\n## Education\n\n");
    for e in EDUCATION {
        let mut line = format!("- {}, {}", e.org, e.role);
        if !e.place.is_empty() {
            line.push_str(&format!(", {}", e.place));
        }
        if !e.when.is_empty() {
            line.push_str(&format!(" ({})", e.when));
        }
        if !e.desc.is_empty() {
            line.push_str(&format!(" -- {}", e.desc));
        }
        s.push_str(&line);
        s.push('\n');
    }
    s.push_str("\n## Skills\n\n");
    for (k, v) in SKILLS {
        s.push_str(&format!("- **{k}:** {v}\n"));
    }
    s
}

// ── JSON, for the Word builder ───────────────────────────────────────────────

fn js(s: &str) -> String {
    let mut o = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)),
            c => o.push(c),
        }
    }
    o.push('"');
    o
}

fn js_list(v: &[&str]) -> String {
    format!("[{}]", v.iter().map(|s| js(s)).collect::<Vec<_>>().join(", "))
}

fn js_entry(e: &Entry) -> String {
    format!(
        "{{\"org\": {}, \"role\": {}, \"when\": {}, \"desc\": {}, \"place\": {}, \"bullets\": {}, \"compact\": {}, \"compactRest\": {}, \"compactRight\": {}}}",
        js(e.org), js(e.role), js(e.when), js(e.desc), js(e.place), js_list(e.bullets), e.compact,
        js(&e.compact_rest()), js(e.compact_right())
    )
}

pub fn to_json() -> String {
    let h = &HEADER;
    let gh = h.github.iter().map(|(n, u)| format!("{{\"handle\": {}, \"url\": {}}}", js(n), js(u))).collect::<Vec<_>>().join(", ");
    let exp = EXPERIENCE.iter().map(js_entry).collect::<Vec<_>>().join(",\n    ");
    let edu = EDUCATION.iter().map(js_entry).collect::<Vec<_>>().join(",\n    ");
    let proj = PROJECTS
        .iter()
        .map(|p| format!(
            "{{\"name\": {}, \"kind\": {}, \"year\": {}, \"desc\": {}, \"url\": {}, \"bullets\": {}}}",
            js(p.name), js(p.kind), js(p.year), js(p.desc), js(p.url), js_list(p.bullets)
        ))
        .collect::<Vec<_>>()
        .join(",\n    ");
    let skills = SKILLS.iter().map(|(k, v)| format!("[{}, {}]", js(k), js(v))).collect::<Vec<_>>().join(",\n    ");
    format!(
        "{{\n  \"_generated\": \"from site/src/resume.rs; edit the Rust, not this file\",\n  \
         \"header\": {{\"name\": {}, \"contact\": {}, \"location\": {}, \"linkedin\": {{\"handle\": {}, \"url\": {}}}, \"github\": [{}]}},\n  \
         \"summary\": {},\n  \"experience\": [\n    {}\n  ],\n  \"projects\": [\n    {}\n  ],\n  \"education\": [\n    {}\n  ],\n  \"skills\": [\n    {}\n  ]\n}}\n",
        js(h.name), js_list(h.contact), js(h.location), js(h.linkedin.0), js(h.linkedin.1), gh,
        js(SUMMARY), exp, proj, edu, skills
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every LaTeX special character in the text has to come out escaped, or pdflatex
    /// silently drops the rest of a line (a bare % starts a comment).
    #[test]
    fn tex_escapes_specials() {
        assert_eq!(tex("~50% & #1 → x – y"), "$\\sim$50\\% \\& \\#1 $\\rightarrow$ x -- y");
        let body = to_tex();
        let doc = &body[body.find("\\begin{document}").unwrap()..];
        for line in doc.lines() {
            let bare_percent = line.match_indices('%').any(|(i, _)| i > 0 && &line[i - 1..i] != "\\");
            assert!(!bare_percent, "unescaped % in: {line}");
        }
    }

    #[test]
    fn json_is_well_formed_enough() {
        let j = to_json();
        assert_eq!(j.matches('{').count(), j.matches('}').count());
        assert_eq!(j.matches('[').count(), j.matches(']').count());
        assert!(!j.contains("\n\""), "a raw newline inside a string");
    }

    /// The résumé goes to outside companies; the capstone client stays unnamed in it too.
    #[test]
    fn client_stays_unnamed() {
        let all = format!("{}{}{}", to_tex(), to_md(), to_json()).to_lowercase();
        assert!(!all.contains(&["aa", "fe"].concat()));
        assert!(!all.contains(&["asian ", "americans"].concat()));
    }
}
