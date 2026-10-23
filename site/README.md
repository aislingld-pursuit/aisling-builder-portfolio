# site: the portfolio, rendered from Rust

The pages at the repo root (`index.html`, `work/`, `about/`, `resume/`, `404.html`) and
`assets/site.css` + `assets/site.js` are **generated**. Edit this crate, not those files.

```bash
cargo run  --manifest-path site/Cargo.toml   # rewrites the pages
cargo test --manifest-path site/Cargo.toml
```

CI re-renders on every push and fails if the committed pages differ from what this crate
produces, so the two cannot drift apart.

## Where things live

| File | What |
|---|---|
| `src/content.rs` | Every fact on the site: builds, case studies, experience, stack, documents. A new build is one entry in `BUILDS`. |
| `src/theme.rs` | The light (lab notebook) and dark (terminal) palettes, with WCAG contrast checked at compile time. |
| `src/main.rs` | Layout: the page shell and one component per page. |
| `assets/style.css` | Type, layout, components. `/*@PALETTES@*/` is replaced with the generated colours. |
| `assets/site.js` | Progressive enhancement only: theme toggle, motion-safe video, contact form, old-anchor redirects. |

## Why not Dioxus's static-site mode

Same reason as the L2 portfolio: GitHub Pages serves this repo under
`/aisling-builder-portfolio/`, and `dx bundle --ssg` cannot take a base path
(DioxusLabs/dioxus#3942). This binary renders each VirtualDom to a String with
`dioxus_ssr` and writes plain files with relative links. No router, no hydration, no WASM.

## Guards

- **Contrast:** every text colour clears 4.5:1 on both the page and raised surfaces, in
  both themes, or the crate does not compile.
- **Content:** every case opens with a standalone "What it is" and states "What it doesn't
  do"; exactly three builds are featured on Home.
- **Assets:** every local file the site links to must exist.
- **Client:** a test fails if the capstone client's name appears in `content.rs`. The
  client stays unnamed until they confirm; media is held back for the same reason.
