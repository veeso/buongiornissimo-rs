# AGENTS.md

This file is the shared repository contract for contributors and coding agents.
Prioritize correctness, maintainability, test coverage, and idiomatic Rust.

## Project Context

**buongiornissimo-rs** is a Rust library that scrapes Italian "boomer
flavoured" greeting images (buongiorno, buonanotte, buon pranzo, weekday
greetings, holidays) from several websites.

## Common Commands

Use `just --list` for the complete interface. The main quality gate is:

```sh
just check
```

Useful focused commands:

```sh
just build
just test
just fmt
just fmt_check
just clippy "-- -D warnings"
just doc
just deny
just zizmor
just scan_secrets .
just changelog_preview 0.4.0
```

`just fmt` uses dprint for Rust, Markdown, TOML, and YAML. Rust files are
delegated to the nightly rustfmt command configured in `dprint.json`.

## Architecture

```text
src/
├── lib.rs               # Greeting enum, greeting_of_the_day, Scrape trait
├── moveable_feasts.rs   # Easter and other moveable feasts (feature gated)
├── providers.rs         # Provider re-exports
└── providers/           # One module per scraped website
    ├── augurando.rs
    ├── buongiornissimo_caffe.rs
    └── ticondivido.rs
```

Each provider implements the `Scrape` trait and returns a list of image URLs
for a given `Greeting`, or `ScrapeError::UnsupportedGreeting`.

### Provider tests

Provider tests hit the live websites. A failing provider test usually means the
website changed its layout or went offline: fetch the page first and check what
it really returns (domains can expire and redirect to parking pages) before
changing selectors.

## Build and Tooling Requirements

- The toolchain is Rust `1.99.0`, pinned by `rust-toolchain.toml`.
- `just` is the stable command interface.
- dprint formats Rust, Markdown, TOML, and YAML.
- `cargo-deny` checks advisories, licenses, bans, and sources (`deny.toml`).
- git-cliff generates release notes from Conventional Commits (`cliff.toml`).
- GitHub Actions use least-privilege permissions, disabled persisted checkout
  credentials, and full-SHA action pins. Run `just zizmor` after workflow
  changes.
- Follow the Cargo.toml conventions: sorted dependencies and features, bare
  versions, `dep:` syntax for optional dependencies.

## Conventions

- Use Conventional Commits. Do not add agent-attribution lines to commits.
- Keep plans and progress files under `.superpowers/`; they must not be
  committed.
- Read [`CONTRIBUTING.md`](./CONTRIBUTING.md) and
  [`AI_POLICY.md`](./AI_POLICY.md) before contributing through an AI-assisted
  workflow.
