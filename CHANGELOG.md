# Changelog

All notable changes to this project are documented in this file.

## 0.4.0

Released on 2026-10-09

### Breaking changes

- remove BuongiornoImmagini provider

> `BuongiornoImmagini` is removed from the public API.

### Changed

- Breaking: remove BuongiornoImmagini provider

> The buongiornoimmagini.it domain was re-registered and now redirects to a
> domain auction page, so the provider can no longer scrape any image.

### Build

- lint Cargo.toml and use dep: syntax for bdays

- **deps:** bump dependencies to latest versions

> Upgrade reqwest to 0.13, scraper to 0.27, rand to 0.10, serial_test to 4
> and tokio to 1.53. The example now imports rand::RngExt.

### Style

- apply clippy fixes and dprint formatting

## 0.3.1

Released on 2025-03-28

### Fixed

- filter out non http; use data-src

## 0.3.0

Released on 2025-03-28

### Breaking changes

- removed IlMondoDiGrazia since it seems to be down

> IlMondoDiGrazia has been removedà

### Added

- Breaking: removed IlMondoDiGrazia since it seems to be down

- new provider: BuongiornoImmagini; new Greetings BuonaSerata, BuonPranzo, BuonaCena

- new provider: BuongiornoImmagini; new Greetings BuonaSerata, BuonPranzo, BuonaCena (#1)

- ticondivido.it provider (#2)

- Augurando provider

## 0.2.1

Released on 2023-05-23

### Added

- Derive `Hash` for greeting

### Fixed

- release date

## 0.1.0

Released on 2022-09-12
