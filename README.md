# buongiornissimo-rs

<p align="center">
  <img src="docs/images/buongiornissimo-rs.png" width="256" height="256" />
</p>

<p align="center">~ Scrapes for the best Italian boomer flavoured images ~</p>
<p align="center">
  <a href="#get-started-">Get started</a>
  ·
  <a href="https://docs.rs/buongiornissimo-rs" target="_blank">Documentation</a>
</p>

<p align="center">Developed by <a href="https://veeso.github.io/" target="_blank">@veeso</a></p>
<p align="center">Current version: 0.2.0 (12/09/2022)</p>

<p align="center">
  <a href="https://opensource.org/licenses/MIT"
    ><img
      src="https://img.shields.io/badge/License-MIT-teal.svg"
      alt="License-MIT"
  /></a>
  <a href="https://github.com/veeso/buongiornissimo-rs/stargazers"
    ><img
      src="https://img.shields.io/github/stars/veeso/buongiornissimo-rs.svg"
      alt="Repo stars"
  /></a>
  <a href="https://crates.io/crates/buongiornissimo-rs"
    ><img
      src="https://img.shields.io/crates/d/buongiornissimo-rs.svg"
      alt="Downloads counter"
  /></a>
  <a href="https://crates.io/crates/buongiornissimo-rs"
    ><img
      src="https://img.shields.io/crates/v/buongiornissimo-rs.svg"
      alt="Latest version"
  /></a>
  <a href="https://ko-fi.com/veeso">
    <img
      src="https://img.shields.io/badge/donate-ko--fi-red"
      alt="Ko-fi"
  /></a>
</p>
<p align="center">
  <a href="https://github.com/veeso/buongiornissimo-rs/actions"
    ><img
      src="https://github.com/veeso/buongiornissimo-rs/workflows/Build/badge.svg"
      alt="Build CI"
  /></a>
  <a href="https://coveralls.io/github/veeso/buongiornissimo-rs"
    ><img
      src="https://coveralls.io/repos/github/veeso/buongiornissimo-rs/badge.svg"
      alt="Coveralls"
  /></a>
  <a href="https://docs.rs/buongiornissimo-rs"
    ><img
      src="https://docs.rs/buongiornissimo-rs/badge.svg"
      alt="Docs"
  /></a>
</p>

---

- [buongiornissimo-rs](#buongiornissimo-rs)
  - [About buongiornissimo-rs 📷](#about-buongiornissimo-rs-)
  - [Features 🎁](#features-)
  - [Get started 🏁](#get-started-)
    - [Add buongiornissimo-rs to your Cargo.toml 🦀](#add-buongiornissimo-rs-to-your-cargotoml-)
    - [Scrape for buongiornissimo ☕](#scrape-for-buongiornissimo-)
    - [Examples 🔍](#examples-)
  - [Documentation 📚](#documentation-)
  - [Support the developer ☕](#support-the-developer-)
  - [Contributing and issues 🤝🏻](#contributing-and-issues-)
  - [Changelog ⏳](#changelog-)
  - [License 📃](#license-)

---

## About buongiornissimo-rs 📷

Buongiornissimo-rs is a Rust library to scrape for Buongiornissimo caffè Italian boomer flavoured images from a Rust application.
It supports different buongiornissimo providers to scrape the images from. It supports many kind of "greetings", such as the classic "buongiornissimo", but also the holiday-based greetings, like "natale", "sacro cuore di Gesù" and "Giovedì grasso". Everthing is provided through a simple and totally async API.

## Features 🎁

- Different providers to prevent api outages and to differentiate the contents.
- Support for different kind of greetings based on the current date
- Utilities functions to retrieve the moveable feasts date (such as Easter, Carnival, Corpus domini...). *requires the `moveable-feasts` feature*
- A super comfy function `greeting_of_the_day()` to retrieve the best greeting for the day

---

## Get started 🏁

### Add buongiornissimo-rs to your Cargo.toml 🦀

```toml
buongiornissimo-rs = "^0.2.0"
```

Supported features are:

- `no-log`: disable logging
- `moveable-feasts` (*default*): enable getters for moveable feasts

### Scrape for buongiornissimo ☕

```rust
use buongiornissimo_rs::{IlMondoDiGrazia, Scrape};
use chrono::Local;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let motd = buongiornissimo_rs::greeting_of_the_day(Local::today().naive_local(), true);
    let urls = IlMondoDiGrazia::default().scrape(motd).await?;
    // Do whatever you want with the scraped images...
    Ok(())
}
```

Currently these providers are supported:

- BuongiornissimoCaffe <https://www.buongiornissimocaffe.it>
- IlMondoDiGrazia <https://ilmondodigrazia.com>

### Examples 🔍

You can check the example to scrape a buongiornissimo imagerunning the example, which is located at `examples/buongiornissimo.rs`:

```sh
cargo run --example buongiornissimo --features moveable-feasts
```

---

## Documentation 📚

The developer documentation can be found on Rust Docs at <https://docs.rs/buongiornissimo-rs>

---

## Support the developer ☕

If you like buongiornissimo-rs and you're grateful for the work I've done, please consider a little donation 🥳

You can make a donation with one of these platforms:

[![ko-fi](https://img.shields.io/badge/Ko--fi-F16061?style=for-the-badge&logo=ko-fi&logoColor=white)](https://ko-fi.com/veeso)
[![PayPal](https://img.shields.io/badge/PayPal-00457C?style=for-the-badge&logo=paypal&logoColor=white)](https://www.paypal.me/chrisintin)
[![bitcoin](https://img.shields.io/badge/Bitcoin-ff9416?style=for-the-badge&logo=bitcoin&logoColor=white)](https://btc.com/bc1qvlmykjn7htz0vuprmjrlkwtv9m9pan6kylsr8w)
[![litecoin](https://img.shields.io/badge/Litecoin-345d9d?style=for-the-badge&logo=Litecoin&logoColor=white)](https://blockchair.com/litecoin/address/ltc1q89a7f859gt7nuekvnuuc25wapkq2f8ny78mp8l)

---

## Contributing and issues 🤝🏻

Contributions, bug reports, new features and questions are welcome! 😉
If you have any question or concern, or you want to suggest a new feature, or you want just want to improve buongiornissimo-rs, feel free to open an issue or a PR.

Please follow [our contributing guidelines](CONTRIBUTING.md)

---

## Changelog ⏳

View buongiornissimo-rs's changelog [HERE](CHANGELOG.md)

---

## License 📃

buongiornissimo-rs is licensed under the MIT license.

You can read the entire license [HERE](LICENSE)
