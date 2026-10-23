#!/usr/bin/env node
// Build the Word résumés so they match the LaTeX PDFs (resume.tex).
//   node tools/build-resume-docx.js            -> letter + legal
//   node tools/build-resume-docx.js letter     -> letter only
// Output: assets/docs/resume/resume.docx (Letter) and resume-legal.docx (Legal).
// Content mirrors resume.tex. When you edit the résumé, update both files.
// Requires: npm install docx  (run once, from the repo root or globally)

const fs = require("fs");
const path = require("path");
const {
  Document, Packer, Paragraph, TextRun, ImageRun, ExternalHyperlink,
  AlignmentType, TabStopType, BorderStyle, LevelFormat, LineRuleType,
} = require("docx");

const ROOT = path.resolve(__dirname, "..");
const OUT = path.join(ROOT, "assets/docs/resume");
const ICONS = path.join(__dirname, "resume-icons");

// ---------- geometry (DXA: 1440 = 1 inch) ----------
const PAGE_W = 12240;                      // 8.5in
const PAGE_H = { letter: 15840, legal: 20160 };
const MARGIN = 576;                        // 0.4in, same as \geometry{margin=0.4in}
const TEXT_W = PAGE_W - 2 * MARGIN;        // 7.7in
const ITEM_INDENT = 90;                    // the itemize offset on each entry
const RIGHT_TAB = ITEM_INDENT + Math.round(TEXT_W * 0.97); // tabular* {0.97\textwidth}

// ---------- type (half-points) ----------
const FONT = "Times New Roman";            // PDF uses Times (mathptmx / Nimbus Roman)
const SZ = { normal: 20, small: 18, large: 24, huge: 50 }; // 10 / 9 / 12 / 25pt
const BLUE = "0000FF";                     // hyperref urlcolor=blue
// LaTeX leading is ~1.2x the font size; Word "single" is looser, so lines are set exactly.
const LEAD = { small: 220, normal: 240, large: 290, huge: 520 }; // 11 / 12 / 14.5 / 26pt
const lead = (k) => ({ line: LEAD[k], lineRule: LineRuleType.EXACT });

const icon = (name) => fs.readFileSync(path.join(ICONS, `icon-${name}.png`));
const ALT = { linkedin: "LinkedIn", github: "GitHub" };
const iconRun = (name, pt) => {
  const label = ALT[name.split("-")[0]];
  return new ImageRun({
    type: "png", data: icon(name),
    transformation: { width: pt * 1.333, height: pt * 1.333 },
    altText: { title: label, description: label, name: label },
  });
};

const t = (text, o = {}) => new TextRun({ text, font: FONT, size: SZ.normal, ...o });
const link = (url, children) => new ExternalHyperlink({ link: url, children });
const blueText = (text, size) => t(text, { color: BLUE, size });

// ---------- building blocks ----------
function sectionTitle(text) {
  return new Paragraph({
    spacing: { before: 160, after: 70, ...lead("large") },
    border: { bottom: { style: BorderStyle.SINGLE, size: 4, color: "000000", space: 1 } },
    keepNext: true,
    children: [t(text, { size: SZ.large, smallCaps: true })],
  });
}

// Two-column row: left text, right text flush to the 0.97 tab stop.
function row(left, right, { after = 0, before = 0, keepNext = true, leading = "normal" } = {}) {
  return new Paragraph({
    indent: { left: ITEM_INDENT },
    tabStops: [{ type: TabStopType.RIGHT, position: RIGHT_TAB }],
    spacing: { before, after, ...lead(leading) },
    keepNext,
    children: [...left, t("\t"), ...right],
  });
}

const sep = (size) => t(" | ", { size });

// \resumeSubheadingEd: bold title | date, italic description | italic location
function subheadingEd(org, role, date, desc, place, before = 75) {
  return [
    row([t(org, { bold: true }), sep(), t(role, { bold: true })], [t(date, { size: SZ.small })], { before }),
    row([t(desc, { italics: true, size: SZ.small })], [t(place, { italics: true, size: SZ.small })], { after: 40, leading: "small" }),
  ];
}

// \resumeSubheadingShort: bold title | small right text
function subheadingShort(org, rest, right, before = 80) {
  return [row([t(org, { bold: true }), sep(), t(rest, { bold: true })], [t(right, { size: SZ.small })], { before, keepNext: false })];
}

// \resumeProjectHeading: bold name | year, italic description | GitHub icon link
function projectHeading(name, kind, year, desc, url, before = 70) {
  return [
    row([t(name, { bold: true }), sep(), t(kind, { bold: true })], [t(year, { size: SZ.small })], { before }),
    row([t(desc, { italics: true, size: SZ.small })], [link(url, [iconRun("github-blue", 8)])], { after: 30, leading: "small" }),
  ];
}

// Hollow-circle bullet (\labelitemii = $\circ$), small text.
function bullet(text, last = false, { overhang = 0 } = {}) {
  return new Paragraph({
    numbering: { reference: "circle", level: 0 },
    // overhang: let one line run into the right margin, as LaTeX does in the PDF
    ...(overhang ? { indent: { left: 537, hanging: 197, right: -overhang } } : {}),
    spacing: { after: last ? 40 : 10, ...lead("small") },
    children: [t(text, { size: SZ.small })],
  });
}

// \resumeSubItem: bold label + text, small.
function skill(label, text) {
  return new Paragraph({
    indent: { left: ITEM_INDENT },
    spacing: { after: 30, ...lead("small") },
    children: [t(label, { bold: true, size: SZ.small }), t(text, { size: SZ.small })],
  });
}

// ---------- content (mirrors resume.tex) ----------
function body() {
  const contactSize = SZ.large;
  const header = [
    new Paragraph({
      alignment: AlignmentType.CENTER,
      spacing: { before: 0, after: 160, ...lead("huge") },
      children: [t("Stan (Aisling) Leiva-Davila", { bold: true, size: SZ.huge })],
    }),
    new Paragraph({
      alignment: AlignmentType.CENTER,
      spacing: { after: 50, ...lead("large") },
      children: [t("ness.aisling@gmail.com | aisling.ld@pursuit.org | 201-471-1178", { size: contactSize })],
    }),
    new Paragraph({
      alignment: AlignmentType.CENTER,
      spacing: { after: 170, ...lead("large") },
      children: [
        t("New York, NY | ", { size: contactSize }),
        iconRun("linkedin-black", 11), t(" ", { size: contactSize }),
        link("https://www.linkedin.com/in/stan-leiva-davila/", [blueText("stan-leiva-davila", contactSize)]),
        t(" | ", { size: contactSize }),
        iconRun("github-black", 11), t(" ", { size: contactSize }),
        link("https://github.com/nessaisling-lab", [blueText("nessaisling-lab", contactSize)]),
        t(" | ", { size: contactSize }),
        iconRun("github-black", 11), t(" ", { size: contactSize }),
        link("https://github.com/aislingld-pursuit", [blueText("aislingld-pursuit", contactSize)]),
      ],
    }),
  ];

  const summary = [
    sectionTitle("Professional Summary"),
    new Paragraph({
      spacing: { after: 40, ...lead("small") },
      children: [t(
        "Self-taught software engineer and AI-native builder, Rust-first with a C, C++, and Assembly foundation. I build systems whose guarantees can be checked: Ed25519-signed, hash-chained exports with an independent verifier; on-device speech-to-text where audio never leaves the machine; Postgres row-level security for a nonprofit client. I ship end-to-end, solo and on small teams, with Claude Code. Inaugural cohort, Pursuit AI‑Native Builder.",
        { size: SZ.small })],
    }),
  ];

  const experience = [
    sectionTitle("Experience"),
    ...subheadingEd("Pursuit Client Capstone (NYC Nonprofit)", "Backend Lead & Repo Admin", "Sep. 2026 – Present",
      "Gala guest list, private sponsor portal, and offline-first iPad check-in (React + TypeScript on Supabase); team of 4.", "New York, NY", 20),
    bullet("Led the Postgres backend: row-level security for 3 staff roles, an append-only change log, and sponsor links and QR e-tickets on 128-bit random tokens stored only as SHA-256 hashes; 787 access-rule checks in ~2,375 per run"),
    bullet("As repo admin, authored 150 of the team’s 189 merged PRs, enforced branch protection and gitleaks secret scanning, and built a local CI fast lane that kept full checks on every PR after GitHub Actions minutes ran out", true),
    ...subheadingEd("Nisaba Capital Charting", "Founder & Builder", "Apr. 2026 – Present",
      "Native-Rust financial-intelligence desktop terminal merging market data with a Hellenistic astrology engine.", "New York, NY"),
    bullet("Built a ~25k-line Rust app solo in 30 days: 12 data APIs, 10 native scrapers, 46 SQLx migrations, an axum sidecar, a sub-arcsecond ephemeris engine; 132 tests, zero warnings, Windows installer"),
    bullet("Reached +37.06% verified paper-trade ROI; presented at Pursuit Demo Day to staff, fellow Builders, and a Fortune 500 reviewer", true),
    ...subheadingEd("Architechne", "Data Analyst & Partnership Coordinator (1099)", "Sep. 2025 – Mar. 2026",
      "Sourced luxury interior-designer, architect, and residential-developer leads for a NYC design partnership business.", "New York, NY"),
    bullet("Built a Python + AI web scraper over NYC tri-state directories and a MySQL territory database, cutting manual research ~50%", true),
    ...subheadingShort("Airbnb", "Self-Employed Property Manager (Owner-Operator), Flushing, NY", "Jul. 2017 – Jan. 2020", 60),
  ];

  const projects = [
    sectionTitle("Technical Projects"),
    ...projectHeading("HouseCheck", "Verifiable Building Records for NYC Renters", "2026",
      "Building Health Card scored from public NYC data, with an export a stranger can verify offline. Live; 149 tests.",
      "https://github.com/nessaisling-lab/housecheck", 20),
    bullet("Built a Rust/axum service whose exports carry an append-only SHA-256 hash chain with an Ed25519-signed head and three verification outcomes (signed and intact, intact but unsigned, tampered), with source dataset IDs inside the signed region"),
    bullet("Wrote an independent Python verifier and found a forgery path: a rewritten chain re-signed with a forger’s own key verified as intact. Closed it by publishing the signing key at a stable endpoint", true),
    ...projectHeading("Ziqpu", "Multi-Agent Astrology Synastry Engine", "2026",
      "Scores a natal chart against the chart of anything with a date of origin (v1: stocks by IPO); reflection, not advice.",
      "https://github.com/nessaisling-lab/Ziqpu-L2-Cycle-3"),
    bullet("Led a 3-person team on a Rust workspace: a measurer agent and an interpreter agent, a read-only axum sidecar, an MCP server, and a Dioxus UI", true),
    ...projectHeading("Resona / Wisper", "Privacy-First On-Device Voice-to-Text", "2026",
      "Cross-platform desktop dictation app: live streaming + file/YouTube transcription, 100% on-device via whisper.cpp.",
      "https://github.com/aislingld-pursuit/L2-Clone-Prodject"),
    bullet("Real-time pipeline (cpal → energy-VAD → incremental Whisper) with zero cloud STT and no accounts; CUDA/Vulkan/Metal/CPU backends from one Tauri 2 + Rust codebase"),
    bullet("Ed25519-signed licenses with client-vs-server entitlement gating; fixed SSRF and unsafe-export issues; shipped v0.2.0-beta.29 on Windows, macOS, Linux", true),
  ];

  const education = [
    sectionTitle("Education"),
    ...subheadingEd("Pursuit", "AI-Native Builder (Inaugural Cohort)", "Mar. 2025 – Present",
      "600+ hour, 7-month in-person intensive: 10+ end-to-end builds, 8+ PRDs; client capstone Demo Day Nov. 10, 2026.", "Long Island City, NY", 20),
    ...subheadingShort("Per Scholas", "IT Support — CompTIA A+, Google IT Support, ServiceNow ITSM", "Grad. Oct. 2025", 60),
    ...subheadingShort("Vaughn College", "Aircraft Mechanic Airframe & Powerplant (coursework complete)", "Flushing, NY", 30),
  ];

  const skills = [
    sectionTitle("Skills & Tools"),
    skill("Languages & Frameworks: ", "Rust (axum, SQLx, Tauri 2, Dioxus, Iced), Python, C, C++, Assembly, TypeScript/React, SQL (Postgres, MySQL)"),
    skill("Security & Integrity: ", "Ed25519 signatures, SHA-256 hash chains, Postgres row-level security, hashed bearer tokens, gitleaks, SSRF hardening"),
    skill("AI & Tooling: ", "Claude Code, Cursor, Anthropic + OpenAI APIs, MCP, whisper.cpp, RAG, agentic workflows"),
    skill("Systems & Ops: ", "Linux, Windows, GitHub admin (branch protection, CI), Supabase, Cloudflare Pages + Access, Fly.io, CUDA/Vulkan/Metal"),
  ];

  return [...header, ...summary, ...experience, ...projects, ...education, ...skills];
}

function build(paper) {
  return new Document({
    creator: "Stan (Aisling) Leiva-Davila",
    title: "Stan (Aisling) Leiva-Davila — Résumé",
    styles: {
      default: { document: { run: { font: FONT, size: SZ.normal }, paragraph: { spacing: { line: 240, before: 0, after: 0 } } } },
      characterStyles: [{ id: "Hyperlink", name: "Hyperlink", run: { color: BLUE } }],
    },
    numbering: {
      config: [{
        reference: "circle",
        levels: [{
          level: 0, format: LevelFormat.BULLET, text: "○", alignment: AlignmentType.LEFT,
          style: { paragraph: { indent: { left: 537, hanging: 197 } }, run: { font: FONT, size: 16 } },
        }],
      }],
    },
    sections: [{
      properties: {
        page: {
          size: { width: PAGE_W, height: PAGE_H[paper] },
          margin: { top: MARGIN, bottom: MARGIN, left: MARGIN, right: MARGIN, header: 0, footer: 0 },
        },
      },
      children: body(),
    }],
  });
}

(async () => {
  const which = process.argv[2] ? [process.argv[2]] : ["letter", "legal"];
  for (const paper of which) {
    const file = path.join(OUT, paper === "letter" ? "resume.docx" : "resume-legal.docx");
    fs.writeFileSync(file, await Packer.toBuffer(build(paper)));
    console.log("wrote", path.relative(ROOT, file));
  }
})();
