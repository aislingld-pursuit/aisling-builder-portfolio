//! Renders the portfolio: Dioxus RSX in, static HTML pages out.
//!
//! Same approach as the L2 portfolio, for the same reason: GitHub Pages serves this repo
//! under `/aisling-builder-portfolio/`, and Dioxus's own static-site mode cannot take a
//! base path (DioxusLabs/dioxus#3942). Rendering each VirtualDom to a String and writing
//! plain files sidesteps it. No router, no hydration, no WASM; links are relative, so the
//! subdirectory never has to be spelled out.

mod content;
mod theme;

use content::*;
use dioxus::prelude::*;
use std::path::{Path, PathBuf};

const STYLE: &str = include_str!("../assets/style.css");
const SCRIPT: &str = include_str!("../assets/site.js");
const FONTS: &str = "https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:wght@400;500\
&family=IBM+Plex+Sans:ital,wght@0,400;0,500;0,600;1,400\
&family=Newsreader:ital,opsz,wght@0,6..72,400;0,6..72,500;1,6..72,400&display=swap";
const SITE_URL: &str = "https://nessaisling-lab.github.io/aisling-builder-portfolio/";
const EMAIL: &str = "ness.aisling@gmail.com";
/// Web3Forms public access key. It is designed to sit in page source.
const FORM_KEY: &str = "a0888a12-c951-4b99-bc7c-fe8da519a5f6";

fn main() -> std::io::Result<()> {
    let out = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let mut written = Vec::new();

    write(&out.join("assets/site.css"), &css(), &mut written)?;
    write(&out.join("assets/site.js"), SCRIPT, &mut written)?;

    let home = rsx! { Home { root: "".to_string() } };
    write(&out.join("index.html"), &page("", "", "Software engineer, Rust-first",
        "Stan (Aisling) Leiva-Davila: Rust-first software engineer in NYC who builds systems whose guarantees can be checked.",
        home), &mut written)?;

    let work = rsx! { Work { root: "../".to_string() } };
    write(&out.join("work/index.html"), &page("../", "work/", "Work",
        "Builds in Rust and on small teams: signed exports, on-device AI, Postgres row-level security.",
        work), &mut written)?;

    for (i, b) in BUILDS.iter().enumerate() {
        let body = rsx! { Case { root: "../../".to_string(), i: i } };
        let path = format!("work/{}/", b.slug);
        write(&out.join(&path).join("index.html"), &page("../../", &path, b.name, b.line, body), &mut written)?;
    }

    let about = rsx! { About { root: "../".to_string() } };
    write(&out.join("about/index.html"), &page("../", "about/", "About",
        "Self-taught in C, C++ and Assembly; A&P coursework; Pursuit AI-Native Builder, inaugural cohort.",
        about), &mut written)?;

    let resume = rsx! { Resume { root: "../".to_string() } };
    write(&out.join("resume/index.html"), &page("../", "resume/", "Résumé",
        "Résumé as PDF (Letter, Legal), Word and plain text, plus writing samples.",
        resume), &mut written)?;

    // GitHub Pages serves 404.html at whatever path was missed, so relative links would
    // resolve against the wrong directory. This one page uses the absolute site root.
    let lost = rsx! { NotFound { root: SITE_URL.to_string() } };
    write(&out.join("404.html"), &page(SITE_URL, "404", "Not found", "That page does not exist.", lost), &mut written)?;

    eprintln!("wrote {} files:", written.len());
    for p in written {
        eprintln!("  {}", p.display());
    }
    Ok(())
}

fn write(path: &Path, body: &str, log: &mut Vec<PathBuf>) -> std::io::Result<()> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    std::fs::write(path, body)?;
    log.push(path.to_path_buf());
    Ok(())
}

/// The stylesheet, with both palettes generated from theme.rs.
fn css() -> String {
    let light = theme::css_vars(&theme::LIGHT);
    let dark = theme::css_vars(&theme::DARK);
    let palettes = format!(
        "/* GENERATED from site/src/theme.rs. Edit the Rust, not this block. Every text\n\
         \x20  colour is held to WCAG 4.5:1 on both grounds by a const assertion. */\n\
         :root{{\n{light}}}\n\
         @media (prefers-color-scheme: dark){{:root:not([data-theme=light]){{\n{dark}}}}}\n\
         :root[data-theme=dark]{{\n{dark}}}\n"
    );
    STYLE.replace("/*@PALETTES@*/", &palettes)
}

fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

/// Wraps a rendered body in the document shell. `root` is the relative path back to the
/// site root from this page; `path` is this page's path from the root, for canonical URLs.
fn page(root: &str, path: &str, title: &str, desc: &str, body: Element) -> String {
    let body = dioxus_ssr::render_element(body);
    let full_title = if path.is_empty() {
        format!("Stan (Aisling) Leiva-Davila · {title}")
    } else {
        format!("{title} · Stan (Aisling) Leiva-Davila")
    };
    let (t, d) = (esc(&full_title), esc(desc));
    let canonical = if path == "404" { SITE_URL.to_string() } else { format!("{SITE_URL}{path}") };
    let header = dioxus_ssr::render_element(rsx! { Top { root: root.to_string(), here: path.to_string() } });
    let footer = dioxus_ssr::render_element(rsx! { Foot { root: root.to_string() } });
    format!(
        "<!doctype html>\n\
         <html lang=\"en\">\n\
         <head>\n\
         <meta charset=\"utf-8\">\n\
         <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n\
         <title>{t}</title>\n\
         <meta name=\"description\" content=\"{d}\">\n\
         <link rel=\"canonical\" href=\"{canonical}\">\n\
         <meta property=\"og:type\" content=\"website\">\n\
         <meta property=\"og:title\" content=\"{t}\">\n\
         <meta property=\"og:description\" content=\"{d}\">\n\
         <meta property=\"og:url\" content=\"{canonical}\">\n\
         <meta property=\"og:image\" content=\"{SITE_URL}assets/og-image.png\">\n\
         <meta name=\"twitter:card\" content=\"summary_large_image\">\n\
         <meta name=\"theme-color\" content=\"#F4F1E9\" media=\"(prefers-color-scheme: light)\">\n\
         <meta name=\"theme-color\" content=\"#121417\" media=\"(prefers-color-scheme: dark)\">\n\
         <link rel=\"icon\" href=\"{root}assets/favicon.svg\" type=\"image/svg+xml\">\n\
         <link rel=\"icon\" href=\"{root}assets/favicon-32.png\" sizes=\"32x32\">\n\
         <link rel=\"apple-touch-icon\" href=\"{root}assets/apple-touch-icon.png\">\n\
         <script>try{{var m=localStorage.getItem('theme');if(m)document.documentElement.dataset.theme=m}}catch(e){{}}</script>\n\
         <link rel=\"preconnect\" href=\"https://fonts.googleapis.com\">\n\
         <link rel=\"preconnect\" href=\"https://fonts.gstatic.com\" crossorigin>\n\
         <link href=\"{FONTS}\" rel=\"stylesheet\">\n\
         <link rel=\"stylesheet\" href=\"{root}assets/site.css\">\n\
         </head>\n\
         <body>\n\
         <a class=\"skip\" href=\"#main\">Skip to content</a>\n\
         {header}\n\
         <main id=\"main\">\n{body}\n</main>\n\
         {footer}\n\
         <script src=\"{root}assets/site.js\" defer></script>\n\
         </body>\n\
         </html>\n"
    )
}

// ── shell ────────────────────────────────────────────────────────────────────

#[component]
fn Top(root: String, here: String) -> Element {
    let cur = |p: &str| if here.starts_with(p) { "page" } else { "false" };
    rsx! {
        header { class: "top",
            div { class: "wrap top-in",
                a { class: "brand", href: "{root}",
                    span { class: "brand-name", "Stan (Aisling) Leiva-Davila" }
                    span { class: "brand-tag", "rust · integrity · nyc" }
                }
                nav { "aria-label": "Site",
                    a { href: "{root}work/", "aria-current": cur("work/"), "Work" }
                    a { href: "{root}about/", "aria-current": cur("about/"), "About" }
                    a { href: "{root}resume/", "aria-current": cur("resume/"), "Résumé" }
                    button {
                        class: "theme",
                        "type": "button",
                        "aria-label": "Switch between light and dark theme",
                        span { class: "theme-ic", "aria-hidden": "true" }
                    }
                }
            }
        }
    }
}

#[component]
fn Foot(root: String) -> Element {
    rsx! {
        footer { class: "foot",
            div { class: "wrap foot-in",
                div {
                    p { class: "foot-name", "Stan (Aisling) Leiva-Davila" }
                    p {
                        a { href: "mailto:{EMAIL}", "{EMAIL}" }
                        " · "
                        a { href: "https://www.linkedin.com/in/stan-leiva-davila/", "LinkedIn" }
                        " · "
                        a { href: "https://github.com/nessaisling-lab", "GitHub" }
                    }
                }
                p { class: "colophon",
                    "Rendered from Rust: Dioxus RSX in, static HTML out, no WASM. Every text colour is
                     held to WCAG 4.5:1 by a compile-time assertion. "
                    a { href: "https://github.com/nessaisling-lab/aisling-builder-portfolio/tree/main/site", "Source" }
                }
            }
        }
    }
}

#[component]
fn Label(prompt: String, text: String) -> Element {
    rsx! {
        p { class: "label",
            span { class: "prompt", "aria-hidden": "true", "{prompt}" }
            " {text}"
        }
    }
}

// ── home ─────────────────────────────────────────────────────────────────────

#[component]
fn Home(root: String) -> Element {
    rsx! {
        section { class: "wrap hero",
            div { class: "hero-text",
                p { class: "status",
                    span { class: "dot", "aria-hidden": "true" }
                    "open to work · New York · on-site or hybrid"
                }
                h1 { "I build software whose guarantees "
                    em { "you can check" }
                    "."
                }
                p { class: "deck",
                    "Rust-first software engineer with a C, C++ and Assembly foundation. Signed,
                     hash-chained exports a stranger can verify offline; speech-to-text where the audio
                     never leaves the machine; the Postgres backend for a nonprofit client's gala. I ship
                     end-to-end, solo and on small teams, with Claude Code."
                }
                div { class: "actions",
                    a { class: "btn", href: "{root}assets/docs/resume/resume-letter.pdf", "Résumé (PDF)" }
                    a { class: "btn ghost", href: "{root}work/", "See the work" }
                    a { class: "text-link", href: "mailto:{EMAIL}", "{EMAIL}" }
                }
            }
            figure { class: "portrait",
                img {
                    src: "{root}assets/portrait-placeholder.jpg",
                    alt: "Portrait of Stan (Aisling) Leiva-Davila",
                    width: "440",
                    height: "550",
                    fetchpriority: "high",
                }
                figcaption { "fig. 0 · the builder, NYC" }
            }
        }

        section { class: "wrap block",
            Label { prompt: "$".to_string(), text: "ls work/ --featured".to_string() }
            h2 { class: "block-h", "Three builds, three kinds of guarantee." }
            ol { class: "featured",
                for (i, b) in BUILDS.iter().filter(|b| b.featured).enumerate() {
                    FeaturedRow { root: root.clone(), n: i + 1, slug: b.slug.to_string() }
                }
            }
            p { class: "more",
                a { class: "text-link", href: "{root}work/", "All builds, including Nisaba, SiteAssure and Ziqpu →" }
            }
        }

        section { class: "wrap block now",
            Label { prompt: "$".to_string(), text: "cat now.txt".to_string() }
            dl { class: "now-list",
                div { dt { "Nov 10" } dd { "Client capstone presents at Pursuit Demo Day. I'm the backend lead and repo admin." } }
                div { dt { "Looking" } dd { "Rust, security and privacy, AI, and backend roles in New York, on-site or hybrid." } }
                div { dt { "Recently" } dd { "Found and closed a forgery path in HouseCheck's signed export by writing an independent verifier." } }
            }
        }

        section { class: "wrap block cta",
            h2 { class: "block-h", "Building something that has to be right?" }
            p { class: "deck", "I'd like to hear about it." }
            div { class: "actions",
                a { class: "btn", href: "mailto:{EMAIL}", "Email me" }
                a { class: "btn ghost", href: "{root}about/#contact", "Contact form" }
            }
        }
    }
}

#[component]
fn FeaturedRow(root: String, n: usize, slug: String) -> Element {
    let b = BUILDS.iter().find(|b| b.slug == slug).expect("featured build exists");
    let num = format!("{n:02}");
    rsx! {
        li { class: "frow",
            a { class: "frow-link", href: "{root}work/{b.slug}/",
                span { class: "frow-n", "{num}" }
                span { class: "frow-main",
                    span { class: "frow-kicker", "{b.kicker}" }
                    span { class: "frow-name", "{b.name}" }
                    span { class: "frow-line", "{b.line}" }
                }
                span { class: "frow-stat", "{b.stat}" }
                span { class: "frow-go", "aria-hidden": "true", "→" }
            }
        }
    }
}

// ── work ─────────────────────────────────────────────────────────────────────

#[component]
fn Work(root: String) -> Element {
    rsx! {
        section { class: "wrap page-head",
            Label { prompt: "$".to_string(), text: "ls work/".to_string() }
            h1 { "Work" }
            p { class: "deck",
                "Most of it is Rust, because most of it makes a promise about what the program will "
                em { "not" }
                " do, and that is easier to hold when a compiler is checking it. Each build has a page:
                 what it is, how it works, the hard part, and what it doesn't do."
            }
        }
        WorkGroup { root: root.clone(), integrity: true,
            title: "Rust, and a promise the program keeps".to_string() }
        WorkGroup { root: root.clone(), integrity: false,
            title: "Team and client work".to_string() }
        section { class: "wrap block",
            Label { prompt: "#".to_string(), text: "earlier".to_string() }
            ul { class: "earlier",
                for e in EARLIER {
                    li {
                        if e.href.is_empty() {
                            span { class: "earlier-name", "{e.name}" }
                        } else {
                            a { class: "earlier-name", href: "{e.href}", "{e.name}" }
                        }
                        span { class: "earlier-line", "{e.line}" }
                    }
                }
            }
            p { class: "more",
                "The four Pursuit L2 builds also have their original write-ups, with measured numbers: "
                a { class: "text-link", href: "https://nessaisling-lab.github.io/pursuit-l2-portfolio/", "pursuit-l2-portfolio →" }
            }
        }
    }
}

#[component]
fn WorkGroup(root: String, integrity: bool, title: String) -> Element {
    let want = if integrity { Group::Integrity } else { Group::Team };
    rsx! {
        section { class: "wrap block",
            h2 { class: "block-h", "{title}" }
            ul { class: "log",
                for b in BUILDS.iter().filter(|b| b.group == want) {
                    li {
                        a { class: "log-row", href: "{root}work/{b.slug}/",
                            span { class: "log-name", "{b.name}" }
                            span { class: "log-line", "{b.line}" }
                            span { class: "log-stat", "{b.stat}" }
                            span { class: "log-year", "{b.year}" }
                        }
                    }
                }
            }
        }
    }
}

// ── case study ───────────────────────────────────────────────────────────────

#[component]
fn Case(root: String, i: usize) -> Element {
    let b = &BUILDS[i];
    let prev = &BUILDS[(i + BUILDS.len() - 1) % BUILDS.len()];
    let next = &BUILDS[(i + 1) % BUILDS.len()];
    rsx! {
        article { class: "wrap case",
            nav { class: "crumbs", "aria-label": "Breadcrumb",
                a { href: "{root}work/", "work" }
                span { "aria-hidden": "true", " / " }
                span { "aria-current": "page", "{b.slug}" }
            }
            header { class: "case-head",
                p { class: "kicker", "{b.kicker} · {b.year}" }
                h1 { "{b.name}" }
                p { class: "deck", "{b.line}" }
                p { class: "status-line",
                    span { class: "dot", "aria-hidden": "true" }
                    "{b.status}"
                }
            }

            div { class: "case-grid",
                aside { class: "spec", "aria-label": "Project facts",
                    div { class: "figs",
                        for f in b.figs {
                            div { class: "fig",
                                span { class: "fig-v", "{f.v}" }
                                span { class: "fig-k", "{f.k}" }
                            }
                        }
                    }
                    dl { class: "spec-dl",
                        div { dt { "Role" } dd { "{b.role}" } }
                        if !b.team.is_empty() {
                            div {
                                dt { "With" }
                                dd {
                                    for (j, p) in b.team.iter().enumerate() {
                                        TeamMember { j: j, name: p.name.to_string(), handle: p.handle.to_string(), role: p.role.to_string() }
                                    }
                                }
                            }
                        }
                        div { dt { "Stack" } dd { class: "stack-chips",
                            for s in b.stack { span { class: "chip", "{s}" } }
                        } }
                    }
                    if !b.links.is_empty() {
                        div { class: "spec-links",
                            for l in b.links {
                                a { class: "btn small ghost", href: "{l.href}", "{l.label} →" }
                            }
                        }
                    }
                }

                div { class: "case-body",
                    if let Some(m) = &b.media {
                        figure { class: if m.w > m.h { "media wide" } else { "media tall" },
                            video {
                                src: "{root}{m.video}",
                                poster: "{root}{m.poster}",
                                width: "{m.w}",
                                height: "{m.h}",
                                controls: true,
                                muted: true,
                                "loop": "true",
                                playsinline: true,
                                preload: "none",
                                "aria-label": "{m.alt}",
                                "data-autoplay": "true",
                            }
                            figcaption { "fig. 1 · {m.alt}" }
                        }
                    }
                    for k in 0..b.sections.len() {
                        CaseSection { i: i, k: k }
                    }
                    if let Some((lead, cmd)) = b.verify {
                        div { class: "verify",
                            Label { prompt: "$".to_string(), text: "verify it yourself".to_string() }
                            p { "{lead}" }
                            pre { code { "{cmd}" } }
                        }
                    }
                    if let Some(n) = b.note {
                        p { class: "note", "{n}" }
                    }
                }
            }

            nav { class: "pager", "aria-label": "More builds",
                a { class: "pager-prev", href: "{root}work/{prev.slug}/",
                    span { class: "pager-k", "← previous" }
                    span { class: "pager-n", "{prev.name}" }
                }
                a { class: "pager-next", href: "{root}work/{next.slug}/",
                    span { class: "pager-k", "next →" }
                    span { class: "pager-n", "{next.name}" }
                }
            }
        }
    }
}

#[component]
fn TeamMember(j: usize, name: String, handle: String, role: String) -> Element {
    let sep = if j > 0 { ", " } else { "" };
    rsx! {
        "{sep}"
        a { href: "https://github.com/{handle}", "{name}" }
        if !role.is_empty() {
            span { class: "quiet", " ({role})" }
        }
    }
}

#[component]
fn CaseSection(i: usize, k: usize) -> Element {
    let s = &BUILDS[i].sections[k];
    let num = format!("{:02}", k + 1);
    rsx! {
        section { class: "cs",
            h2 {
                span { class: "cs-n", "aria-hidden": "true", "{num}" }
                "{s.title}"
            }
            p { class: if s.points.is_empty() { "cs-lead first" } else { "cs-lead" }, "{s.lead}" }
            if !s.points.is_empty() {
                ul {
                    for p in s.points {
                        li { "{p}" }
                    }
                }
            }
        }
    }
}

// ── about ────────────────────────────────────────────────────────────────────

#[component]
fn About(root: String) -> Element {
    rsx! {
        section { class: "wrap page-head",
            Label { prompt: "$".to_string(), text: "whoami".to_string() }
            h1 { "About" }
            div { class: "prose",
                p {
                    "I taught myself C, C++ and Assembly in my late teens, and installed Debian on a
                     PlayStation 3 with a custom Wi-Fi driver because I wanted to know if I could. Then
                     aircraft-mechanic A&P coursework at Vaughn College: methodical troubleshooting, risk
                     awareness, precision under pressure. A family emergency paused that degree; the
                     habits stayed."
                }
                p {
                    "Now I'm in the inaugural cohort of Pursuit's AI-Native Builder program, a 600+ hour
                     in-person intensive in Long Island City, after Per Scholas IT Support (CompTIA A+,
                     Google IT Support, ServiceNow ITSM). Most of what I build is Rust, and most of it
                     makes a promise about what the program will not do."
                }
                p {
                    "Before that: data analysis and lead generation at Architechne, and two and a half
                     years running an Airbnb business solo. The same end-to-end ownership, applied to
                     software."
                }
            }
        }

        section { class: "wrap block",
            Label { prompt: "#".to_string(), text: "how I work".to_string() }
            div { class: "rules",
                for r in RULES {
                    div { class: "rule",
                        h3 { "{r.head}" }
                        p { "{r.body}" }
                    }
                }
            }
        }

        section { class: "wrap block", id: "experience",
            h2 { class: "block-h", "Experience" }
            Timeline { education: false }
        }
        section { class: "wrap block", id: "education",
            h2 { class: "block-h", "Education" }
            Timeline { education: true }
        }

        section { class: "wrap block", id: "stack",
            h2 { class: "block-h", "Stack" }
            dl { class: "stack-dl",
                for r in STACK {
                    div { dt { "{r.dt}" } dd { "{r.dd}" } }
                }
            }
        }

        section { class: "wrap block", id: "contact",
            h2 { class: "block-h", "Contact" }
            div { class: "contact",
                div { class: "contact-direct",
                    p { "The fastest way to reach me is email." }
                    p { class: "contact-mail", a { href: "mailto:{EMAIL}", "{EMAIL}" } }
                    p {
                        a { class: "text-link", href: "https://www.linkedin.com/in/stan-leiva-davila/", "LinkedIn" }
                        " · "
                        a { class: "text-link", href: "https://github.com/nessaisling-lab", "github.com/nessaisling-lab" }
                        " · "
                        a { class: "text-link", href: "https://github.com/aislingld-pursuit", "github.com/aislingld-pursuit" }
                    }
                }
                form { class: "form", id: "contactForm", action: "https://api.web3forms.com/submit", method: "POST",
                    input { "type": "hidden", name: "access_key", value: "{FORM_KEY}" }
                    input { "type": "hidden", name: "subject", value: "New message from the portfolio" }
                    input { "type": "hidden", name: "from_name", value: "Portfolio" }
                    input { class: "hp", "type": "checkbox", name: "botcheck", tabindex: "-1", autocomplete: "off" }
                    label { r#for: "cf-name", "Name" }
                    input { id: "cf-name", "type": "text", name: "name", autocomplete: "name", required: true }
                    label { r#for: "cf-email", "Email" }
                    input { id: "cf-email", "type": "email", name: "email", autocomplete: "email", required: true }
                    label { r#for: "cf-msg", "Message" }
                    textarea { id: "cf-msg", name: "message", rows: "5", required: true }
                    button { class: "btn", "type": "submit", "Send" }
                    p { class: "form-status", id: "formStatus", role: "status", "aria-live": "polite" }
                }
            }
        }
    }
}

#[component]
fn Timeline(education: bool) -> Element {
    let rows = if education { SCHOOLS } else { JOBS };
    rsx! {
        ol { class: "timeline",
            for r in rows {
                li {
                    span { class: "tl-when", "{r.when}" }
                    div {
                        h3 { "{r.org} " span { class: "quiet", "· {r.role}" } }
                        p { "{r.body}" }
                    }
                }
            }
        }
    }
}

// ── résumé ───────────────────────────────────────────────────────────────────

#[component]
fn Resume(root: String) -> Element {
    rsx! {
        section { class: "wrap page-head",
            Label { prompt: "$".to_string(), text: "ls resume/".to_string() }
            h1 { "Résumé" }
            p { class: "deck", "One page, in Letter or Legal. Also as Word and plain text, built from the same source." }
            div { class: "actions",
                a { class: "btn", href: "{root}assets/docs/resume/resume-letter.pdf", "Download PDF (Letter)" }
                a { class: "btn ghost", href: "{root}assets/docs/resume/resume.docx", "Word (Letter)" }
            }
        }
        section { class: "wrap block",
            h2 { class: "block-h", "Documents" }
            ul { class: "docs",
                for d in DOCS {
                    li {
                        span { class: "doc-name", "{d.name}" }
                        span { class: "doc-files",
                            for f in d.files {
                                a { class: "chip link", href: "{root}{f.1}", "{f.0}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn NotFound(root: String) -> Element {
    rsx! {
        section { class: "wrap page-head",
            Label { prompt: "$".to_string(), text: "cd here".to_string() }
            h1 { "Not found" }
            p { class: "deck", "No such page. It may have moved when the site went multi-page." }
            div { class: "actions",
                a { class: "btn", href: "{root}", "Home" }
                a { class: "btn ghost", href: "{root}work/", "Work" }
            }
        }
    }
}
