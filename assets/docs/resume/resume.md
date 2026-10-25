# Stan (Aisling) Leiva-Davila

**Location:** New York, NY
**Email:** ness.aisling@gmail.com
**Phone:** 201-471-1178
**LinkedIn:** linkedin.com/in/stan-leiva-davila
**Portfolio:** nessaisling-lab.github.io/aisling-builder-portfolio
**GitHub:** github.com/nessaisling-lab · github.com/aislingld-pursuit

## Professional Summary

Self-taught software engineer and AI-native builder, Rust-first with a C, C++, and Assembly foundation. I build systems whose guarantees can be checked: Ed25519-signed, hash-chained exports with an independent verifier; on-device speech-to-text where audio never leaves the machine; Postgres row-level security for a nonprofit client. I ship end-to-end, solo and on small teams, with Claude Code. Inaugural cohort, Pursuit AI-Native Builder.

## Work Experience

### Pursuit Client Capstone (NYC Nonprofit) -- New York, NY

**Backend Lead & Repo Admin**
Sep. 2026 – Present

- Gala guest list, private sponsor portal, and offline-first iPad check-in (React + TypeScript on Supabase); team of 4
- Led the Postgres backend: row-level security for 3 staff roles, an append-only change log, and sponsor links and QR e-tickets on 128-bit random tokens stored only as SHA-256 hashes; 787 access-rule checks in ~2,375 per run
- As repo admin, authored 150 of the team's 189 merged PRs, enforced branch protection and gitleaks secret scanning, and built a local CI fast lane that kept full checks on every PR after GitHub Actions minutes ran out

### Nisaba Capital Charting -- New York, NY

**Founder & Builder**
Apr. 2026 – Present

- Native-Rust financial-intelligence desktop terminal merging market data with a Hellenistic astrology engine
- Built a ~25k-line Rust app solo in 30 days: 12 data APIs, 10 native scrapers, 46 SQLx migrations, an axum sidecar, a sub-arcsecond ephemeris engine; 132 tests, zero warnings, Windows installer
- Reached +37.06% verified paper-trade ROI; presented at Pursuit Demo Day to staff, fellow Builders, and a Fortune 500 reviewer

### Architechne -- New York, NY

**Data Analyst & Partnership Coordinator (1099)**
Sep. 2025 – Mar. 2026

- Sourced luxury interior-designer, architect, and residential-developer leads for a NYC design partnership business
- Built a Python + AI web scraper over NYC tri-state directories and a MySQL territory database, cutting manual research ~50%

### Airbnb -- Flushing, NY

**Self-Employed Property Manager (Owner-Operator)**
Jul. 2017 – Jan. 2020


## Projects

- **HouseCheck: Verifiable Building Records for NYC Renters** (https://github.com/nessaisling-lab/housecheck) -- Building Health Card scored from public NYC data, with an export a stranger can verify offline. Live; 176 tests. Built a Rust/axum service whose exports carry an append-only SHA-256 hash chain with an Ed25519-signed head and three verification outcomes (signed and intact, intact but unsigned, tampered), with source dataset IDs inside the signed region. Wrote an independent Python verifier and found a forgery path: a rewritten chain re-signed with a forger's own key verified as intact. Closed it by publishing the signing key at a stable endpoint
- **Ziqpu: Multi-Agent Astrology Synastry Engine** (https://github.com/nessaisling-lab/Ziqpu-L2-Cycle-3) -- Scores a natal chart against the chart of anything with a date of origin (v1: stocks by IPO); reflection, not advice. Led a 3-person team on a Rust workspace: a measurer agent and an interpreter agent, a read-only axum sidecar, an MCP server, and a Dioxus UI
- **Resona / Wisper: Privacy-First On-Device Voice-to-Text** (https://github.com/aislingld-pursuit/L2-Clone-Prodject) -- Cross-platform desktop dictation app: live streaming + file/YouTube transcription, 100% on-device via whisper.cpp. Real-time pipeline (cpal → energy-VAD → incremental Whisper) with zero cloud STT and no accounts; CUDA/Vulkan/Metal/CPU backends from one Tauri 2 + Rust codebase. Ed25519-signed licenses with client-vs-server entitlement gating; fixed SSRF and unsafe-export issues; shipped v0.2.0-beta.29 on Windows, macOS, Linux

## Education

- Pursuit, AI-Native Builder (Inaugural Cohort), Long Island City, NY (Mar. 2025 – Present) -- 600+ hour, 7-month in-person intensive: 10+ end-to-end builds, 8+ PRDs; client capstone Demo Day Nov. 10, 2026.
- Per Scholas, IT Support — CompTIA A+, Google IT Support, ServiceNow ITSM (Grad. Oct. 2025)
- Vaughn College, Aircraft Mechanic Airframe & Powerplant (coursework complete), Flushing, NY

## Skills

- **Languages & Frameworks:** Rust (axum, SQLx, Tauri 2, Dioxus, Iced), Python, C, C++, Assembly, TypeScript/React, SQL (Postgres, MySQL)
- **Security & Integrity:** Ed25519 signatures, SHA-256 hash chains, Postgres row-level security, hashed bearer tokens, gitleaks, SSRF hardening
- **AI & Tooling:** Claude Code, Cursor, Anthropic + OpenAI APIs, MCP, whisper.cpp, RAG, agentic workflows
- **Systems & Ops:** Linux, Windows, GitHub admin (branch protection, CI), Supabase, Cloudflare Pages + Access, Fly.io, CUDA/Vulkan/Metal
