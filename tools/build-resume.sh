#!/usr/bin/env bash
# Regenerate every résumé format for the Documents hub.
#
# One source of truth: site/src/resume.rs. Edit the résumé there, then run this.
#   cargo (site/)  -> resume.tex, resume.md, resume.json  (+ the site, including /resume/)
#   pandoc         -> resume.txt, resume.html             (from resume.md)
#   node + docx    -> resume.docx, resume-legal.docx      (from resume.json, laid out like the PDF)
#   pdflatex       -> resume-letter.pdf, resume-legal.pdf (from resume.tex)
# Requires: cargo, pandoc, node + the `docx` npm package, pdflatex (MiKTeX/TeX Live).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
D="$ROOT/assets/docs/resume"

PANDOC="$(command -v pandoc || echo '/c/Program Files/Pandoc/pandoc.exe')"
PDFLATEX="$(command -v pdflatex || echo '/c/Program Files/MiKTeX/miktex/bin/x64/pdflatex.exe')"

echo "==> sources from site/src/resume.rs"
cargo run --quiet --manifest-path "$ROOT/site/Cargo.toml" --locked

echo "==> pandoc formats from resume.md"
"$PANDOC" "$D/resume.md" -t plain -o "$D/resume.txt"
"$PANDOC" "$D/resume.md" -s --embed-resources --metadata title="Stan (Aisling) Leiva-Davila — Resume" -o "$D/resume.html"

echo "==> Word files (letter + legal), matched to the PDF layout"
node "$ROOT/tools/build-resume-docx.js"

echo "==> PDFs from resume.tex (letter + legal)"
TMP="$(mktemp -d)"
cp "$D/resume.tex" "$TMP/resume-letter.tex"
sed 's/\[letterpaper,10pt\]/[legalpaper,10pt]/' "$D/resume.tex" > "$TMP/resume-legal.tex"
( cd "$TMP"
  for v in letter legal; do
    "$PDFLATEX" -enable-installer -interaction=nonstopmode "resume-$v.tex" >/dev/null 2>&1 || true
  done )
cp "$TMP/resume-letter.pdf" "$D/resume-letter.pdf"
cp "$TMP/resume-legal.pdf"  "$D/resume-legal.pdf"
rm -rf "$TMP"

# career-ops reads the same Markdown as its cv.md. Only when the hub layout is present.
CAREER_OPS="$ROOT/../career-ops"
if [ -d "$CAREER_OPS" ]; then
  cp "$D/resume.md" "$CAREER_OPS/cv.md"
  echo "==> refreshed $CAREER_OPS/cv.md"
fi

echo "==> done. Formats in $D:"
ls -1 "$D"
