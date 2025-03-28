# Changelog

- [Changelog](#changelog)
  - [0.3.1](#031)
  - [0.3.0](#030)
  - [0.2.1](#021)
  - [0.2.0](#020)
  - [0.1.0](#010)

---

## 0.3.1

Released on 28/03/2025

- filter out Non http sources when scraping
- some sites use `data-src` instead of `src` for images, so we need to check for that too

## 0.3.0

Released on 28/03/2025

- removed `IlMondoDiGrazia` provider since it's not working anymore
- New providers:
  - `Augurando`: <https://augurando.it>
  - `BuongiornoImmagini` <https://www.buongiornoimmagini.it>
  - `Ticondivido` <https://ticondivido.it/>
- New greetings:
  - `BuonaSerata`
  - `BuonaCena`
  - `BuonPomeriggio`
  - `Weekend`
  - `SanNicola`
  - `SantAmbrogio`
  - `SantaLucia`
  - `SanSilvestro`
  - `FestaDelPapa` (Not the pope, but the father's day)
  - `FestaDellaMamma`
  - `DueGiugno`
- Added getter for festa della mamma `festa_della_mamma`

## 0.2.1

Released on 23/05/2023

- Derive `Hash` for greeting.
- Deps updated

## 0.2.0

Released on 12/09/2022

- Added new greetings
  - san valentino
  - festa della donna
  - domenica delle palme
  - santo stefano
- added `BuongiornissimoCaffe` provider from <https://www.buongiornissimocaffe.it>

## 0.1.0

Released on 12/09/2022

- First release
