#!/usr/bin/env node
// Build the Word résumés so they match the LaTeX PDFs (resume.tex).
//   node tools/build-resume-docx.js            -> letter + legal
//   node tools/build-resume-docx.js letter     -> letter only
// Output: assets/docs/resume/resume.docx (Letter) and resume-legal.docx (Legal).
// Content comes from assets/docs/resume/resume.json, which site/src/resume.rs generates.
// Edit the résumé there, never here; this file only owns the Word layout.
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

// ---------- content: from resume.json (generated by site/src/resume.rs) ----------
const CV = JSON.parse(fs.readFileSync(path.join(OUT, "resume.json"), "utf8"));

// Spacing before an entry, matching the PDF: tight under a section title, a little
// air between full entries, less between consecutive one-line entries.
function entries(list) {
  const out = [];
  list.forEach((e, i) => {
    const prev = list[i - 1];
    if (e.compact) {
      const before = i === 0 ? 20 : prev.compact ? 30 : 60;
      out.push(...subheadingShort(e.org, e.compactRest, e.compactRight, before));
      return;
    }
    out.push(...subheadingEd(e.org, e.role, e.when, e.desc, e.place, i === 0 ? 20 : 75));
    e.bullets.forEach((b, j) => out.push(bullet(b, j === e.bullets.length - 1)));
  });
  return out;
}

function body() {
  const h = CV.header;
  const contactSize = SZ.large;
  const header = [
    new Paragraph({
      alignment: AlignmentType.CENTER,
      spacing: { before: 0, after: 160, ...lead("huge") },
      children: [t(h.name, { bold: true, size: SZ.huge })],
    }),
    new Paragraph({
      alignment: AlignmentType.CENTER,
      spacing: { after: 50, ...lead("large") },
      children: [t(h.contact.join(" | "), { size: contactSize })],
    }),
    new Paragraph({
      alignment: AlignmentType.CENTER,
      spacing: { after: 170, ...lead("large") },
      children: [
        t(`${h.location} | `, { size: contactSize }),
        iconRun("linkedin-black", 11), t(" ", { size: contactSize }),
        link(h.linkedin.url, [blueText(h.linkedin.handle, contactSize)]),
        ...h.github.flatMap((g) => [
          t(" | ", { size: contactSize }),
          iconRun("github-black", 11), t(" ", { size: contactSize }),
          link(g.url, [blueText(g.handle, contactSize)]),
        ]),
      ],
    }),
  ];

  const summary = [
    sectionTitle("Professional Summary"),
    new Paragraph({ spacing: { after: 40, ...lead("small") }, children: [t(CV.summary, { size: SZ.small })] }),
  ];

  const projects = [sectionTitle("Technical Projects")];
  CV.projects.forEach((p, i) => {
    projects.push(...projectHeading(p.name, p.kind, p.year, p.desc, p.url, i === 0 ? 20 : 70));
    p.bullets.forEach((b, j) => projects.push(bullet(b, j === p.bullets.length - 1)));
  });

  return [
    ...header,
    ...summary,
    sectionTitle("Experience"), ...entries(CV.experience),
    ...projects,
    sectionTitle("Education"), ...entries(CV.education),
    sectionTitle("Skills & Tools"), ...CV.skills.map(([k, v]) => skill(`${k}: `, v)),
  ];
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
