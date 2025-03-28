//! # buongiornissimo-rs
//!
//! Buongiornissimo-rs is a Rust library to scrape for Buongiornissimo caffè Italian boomer flavoured images from a Rust application.
//! It supports different buongiornissimo providers to scrape the images from. It supports many kind of "greetings", such as the classic "buongiornissimo", but also the holiday-based greetings, like "natale", "sacro cuore di Gesù" and "Giovedì grasso". Everthing is provided through a simple and totally async API.
//!
//! ## Features 🎁
//!
//! - Different providers to prevent api outages and to differentiate the contents.
//! - Support for different kind of greetings based on the current date
//! - Utilities functions to retrieve the moveable feasts date (such as Easter, Carnival, Corpus domini...). *requires the `moveable-feasts` feature*
//! - A super comfy function `greeting_of_the_day()` to retrieve the best greeting for the day
//!
//! ## Get started
//!
//! ### Add buongiornissimo-rs to your Cargo.toml 🦀
//!
//! ```toml
//! buongiornissimo-rs = "^0.3"
//! ```
//!
//! Supported features are:
//!
//! - `no-log`: disable logging
//! - `moveable-feasts` (*default*): enable getters for moveable feasts
//!
//! ### Scrape for buongiornissimo ☕
//!
//! ```rust
//! use buongiornissimo_rs::{BuongiornissimoCaffe, Scrape};
//! use chrono::Local;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let motd = buongiornissimo_rs::greeting_of_the_day(Local::today().naive_local(), true);
//!     let urls = BuongiornissimoCaffe::default().scrape(motd).await?;
//!     // Do whatever you want with the scraped images...
//!     Ok(())
//! }
//! ```
//!

#![doc(html_playground_url = "https://play.rust-lang.org")]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/veeso/buongiornissimo-rs/main/docs/images/cargo/buongiornissimo-rs-128.png"
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/veeso/buongiornissimo-rs/main/docs/images/cargo/buongiornissimo-rs-512.png"
)]

#[macro_use]
extern crate tracing;

use async_trait::async_trait;
use chrono::NaiveDate;
use thiserror::Error;
use url::Url;

// modules
#[cfg(feature = "moveable-feasts")]
pub mod moveable_feasts;
mod providers;

// exports
pub use providers::{BuongiornissimoCaffe, BuongiornoImmagini, TiCondivido};

/// Describes the Greeting type
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Greeting {
    BuonGiorno,
    /// Buongiorno based on the weekday
    BuonGiornoWeekday(chrono::Weekday),
    Weekend,
    BuonPomeriggio,
    BuonPranzo,
    BuonaNotte,
    BuonaSerata,
    BuonaCena,
    Compleanno,
    // feasts
    Capodanno,
    Epifania,
    SanValentino,
    GiovediGrasso,
    MartediGrasso,
    MercolediCeneri,
    FestaDelleDonne,
    FestaDelPapa,
    FestaDellaMamma,
    DomenicaDellePalme,
    Pasqua,
    Pasquetta,
    Liberazione,
    FestaDeiLavoratori,
    Ascensione,
    Pentecoste,
    DueGiugno,
    SantissimaTrinita,
    FestaDellaRepubblica,
    SacroCuoreDiGesu,
    CuoreImmacolatoDiMaria,
    CorpusDomini,
    Ferragosto,
    Halloween,
    /// Primo novembre
    Ognissanti,
    /// Refers to the 2nd of november
    Defunti,
    /// 6 dicembre
    SanNicola,
    /// 7 dicembre
    SantAmbrogio,
    /// 8 dicembre
    ImmacolataConcenzione,
    /// 13 dicembre
    SantaLucia,
    VigiliaDiNatale,
    Natale,
    SantoStefano,
    /// 31 dicembre
    SanSilvestro,
}

/// Scrape trait result
pub type ScrapeResult<T> = Result<T, ScrapeError>;

/// Scrape error
#[derive(Debug, Error, Eq, PartialEq)]
pub enum ScrapeError {
    #[error("this scraper doesn't support this greeting type")]
    UnsupportedGreeting,
    #[error("http error: {0}")]
    Http(String),
    #[error("css parser error: {0}")]
    Css(String),
    #[error("unexpected HTML: {0}")]
    UnexpectedHtml(String),
    #[error("could not find any image in the page")]
    NoImages,
}

impl From<reqwest::Error> for ScrapeError {
    fn from(e: reqwest::Error) -> Self {
        Self::Http(e.to_string())
    }
}

/// The Scrape trait defines the behaviour to scrape the images from the different boomer images providers
#[async_trait]
pub trait Scrape {
    /// Scrape for a certain kind of greeting.
    /// Returns the list of the image urls
    async fn scrape(&self, greeting: Greeting) -> ScrapeResult<Vec<Url>>;
}

/// A utility function to return the greeting for the day based on the current date (considers holiday).
///
/// If `use_weekday` is `true` the greeting returned for regular days will be `BuongiornoWeekday(today.weekday)` otherwise `Buongiorno`
/// If the `moveable-feasts` feature is enabled, moveable feasts dates will be considered
pub fn greeting_of_the_day(date: NaiveDate, use_weekday: bool) -> Greeting {
    use chrono::Datelike;
    match date {
        date if date.month() == 1 && date.day() == 1 => Greeting::Capodanno,
        date if date.month() == 1 && date.day() == 6 => Greeting::Epifania,
        date if date.month() == 2 && date.day() == 14 => Greeting::SanValentino,
        date if date.month() == 3 && date.day() == 8 => Greeting::FestaDelleDonne,
        #[cfg(feature = "moveable-feasts")]
        date if date == moveable_feasts::giovedi_grasso_date(date.year()) => {
            Greeting::GiovediGrasso
        }
        #[cfg(feature = "moveable-feasts")]
        date if date == moveable_feasts::martedi_grasso_date(date.year()) => {
            Greeting::MartediGrasso
        }
        #[cfg(feature = "moveable-feasts")]
        date if date == moveable_feasts::mercoled_ceneri_date(date.year()) => {
            Greeting::MercolediCeneri
        }
        #[cfg(feature = "moveable-feasts")]
        date if date == moveable_feasts::domenica_delle_palme_date(date.year()) => {
            Greeting::DomenicaDellePalme
        }
        #[cfg(feature = "moveable-feasts")]
        date if date == moveable_feasts::easter_date(date.year()) => Greeting::Pasqua,
        #[cfg(feature = "moveable-feasts")]
        date if date == moveable_feasts::pasquetta_date(date.year()) => Greeting::Pasquetta,
        #[cfg(feature = "moveable-feasts")]
        date if date == moveable_feasts::ascensione_date(date.year()) => Greeting::Ascensione,
        #[cfg(feature = "moveable-feasts")]
        date if date == moveable_feasts::pentecoste_date(date.year()) => Greeting::Pentecoste,
        #[cfg(feature = "moveable-feasts")]
        date if date == moveable_feasts::santissima_trinita_date(date.year()) => {
            Greeting::SantissimaTrinita
        }
        date if date.month() == 4 && date.day() == 25 => Greeting::Liberazione,
        date if date.month() == 5 && date.day() == 1 => Greeting::FestaDeiLavoratori,
        date if date.month() == 6 && date.day() == 2 => Greeting::FestaDellaRepubblica,
        #[cfg(feature = "moveable-feasts")]
        date if date == moveable_feasts::corpus_domini_date(date.year()) => Greeting::CorpusDomini,
        #[cfg(feature = "moveable-feasts")]
        date if date == moveable_feasts::sacro_cuore_di_gesu_date(date.year()) => {
            Greeting::SacroCuoreDiGesu
        }
        #[cfg(feature = "moveable-feasts")]
        date if date == moveable_feasts::cuore_immacolato_di_maria_date(date.year()) => {
            Greeting::CuoreImmacolatoDiMaria
        }
        date if date.month() == 8 && date.day() == 15 => Greeting::Ferragosto,
        date if date.month() == 10 && date.day() == 31 => Greeting::Halloween,
        date if date.month() == 11 && date.day() == 1 => Greeting::Ognissanti,
        date if date.month() == 11 && date.day() == 2 => Greeting::Defunti,
        date if date.month() == 12 && date.day() == 8 => Greeting::ImmacolataConcenzione,
        date if date.month() == 12 && date.day() == 24 => Greeting::VigiliaDiNatale,
        date if date.month() == 12 && date.day() == 25 => Greeting::Natale,
        date if date.month() == 12 && date.day() == 26 => Greeting::SantoStefano,
        date if use_weekday => Greeting::BuonGiornoWeekday(date.weekday()),
        _ => Greeting::BuonGiorno,
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn should_get_greeting_of_the_day_ordinary() {
        assert_eq!(
            greeting_of_the_day(NaiveDate::from_ymd_opt(2022, 12, 5).unwrap(), false),
            Greeting::BuonGiorno
        );
    }

    #[test]
    fn should_get_greeting_of_the_day_ordinary_weekday() {
        assert_eq!(
            greeting_of_the_day(NaiveDate::from_ymd_opt(2022, 12, 5).unwrap(), true),
            Greeting::BuonGiornoWeekday(chrono::Weekday::Mon)
        );
    }

    #[test]
    fn should_get_greeting_of_the_day_capodanno() {
        assert_eq!(
            greeting_of_the_day(NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(), true),
            Greeting::Capodanno
        );
    }

    #[test]
    fn should_get_greeting_of_the_day_epifania() {
        assert_eq!(
            greeting_of_the_day(NaiveDate::from_ymd_opt(2022, 1, 6).unwrap(), true),
            Greeting::Epifania
        );
    }

    #[test]
    fn should_get_greeting_of_the_day_san_valentino() {
        assert_eq!(
            greeting_of_the_day(NaiveDate::from_ymd_opt(2022, 2, 14).unwrap(), true),
            Greeting::SanValentino
        );
    }

    #[test]
    fn should_get_greeting_of_the_day_festadonne() {
        assert_eq!(
            greeting_of_the_day(NaiveDate::from_ymd_opt(2022, 3, 8).unwrap(), true),
            Greeting::FestaDelleDonne
        );
    }

    #[test]
    #[cfg(feature = "moveable-feasts")]
    fn should_get_greeting_of_the_day_giovedi_grasso() {
        assert_eq!(
            greeting_of_the_day(NaiveDate::from_ymd_opt(2023, 2, 16).unwrap(), true),
            Greeting::GiovediGrasso
        );
    }

    #[test]
    #[cfg(feature = "moveable-feasts")]
    fn should_get_greeting_of_the_day_martedi_grasso() {
        assert_eq!(
            greeting_of_the_day(NaiveDate::from_ymd_opt(2023, 2, 21).unwrap(), true),
            Greeting::MartediGrasso
        );
    }

    #[test]
    #[cfg(feature = "moveable-feasts")]
    fn should_get_greeting_of_the_day_ceneri() {
        assert_eq!(
            greeting_of_the_day(NaiveDate::from_ymd_opt(2023, 2, 22).unwrap(), true),
            Greeting::MercolediCeneri
        );
    }

    #[test]
    #[cfg(feature = "moveable-feasts")]
    fn should_get_greeting_of_the_day_palme() {
        assert_eq!(
            greeting_of_the_day(NaiveDate::from_ymd_opt(2023, 4, 2).unwrap(), true),
            Greeting::DomenicaDellePalme
        );
    }

    #[test]
    #[cfg(feature = "moveable-feasts")]
    fn should_get_greeting_of_the_day_pasqua() {
        assert_eq!(
            greeting_of_the_day(NaiveDate::from_ymd_opt(2023, 4, 9).unwrap(), true),
            Greeting::Pasqua
        );
    }

    #[test]
    #[cfg(feature = "moveable-feasts")]
    fn should_get_greeting_of_the_day_pasquetta() {
        assert_eq!(
            greeting_of_the_day(NaiveDate::from_ymd_opt(2023, 4, 10).unwrap(), true),
            Greeting::Pasquetta
        );
    }

    #[test]
    #[cfg(feature = "moveable-feasts")]
    fn should_get_greeting_of_the_day_ascensione() {
        assert_eq!(
            greeting_of_the_day(NaiveDate::from_ymd_opt(2023, 5, 21).unwrap(), true),
            Greeting::Ascensione
        );
    }

    #[test]
    #[cfg(feature = "moveable-feasts")]
    fn should_get_greeting_of_the_day_trinita() {
        assert_eq!(
            greeting_of_the_day(NaiveDate::from_ymd_opt(2023, 6, 4).unwrap(), true),
            Greeting::SantissimaTrinita
        );
    }

    #[test]
    #[cfg(feature = "moveable-feasts")]
    fn should_get_greeting_of_the_day_corpus_domini() {
        assert_eq!(
            greeting_of_the_day(NaiveDate::from_ymd_opt(2023, 6, 11).unwrap(), true),
            Greeting::CorpusDomini
        );
    }

    #[test]
    #[cfg(feature = "moveable-feasts")]
    fn should_get_greeting_of_the_day_sacro_cuore() {
        assert_eq!(
            greeting_of_the_day(NaiveDate::from_ymd_opt(2023, 6, 16).unwrap(), true),
            Greeting::SacroCuoreDiGesu
        );
    }

    #[test]
    #[cfg(feature = "moveable-feasts")]
    fn should_get_greeting_of_the_day_cuore_immacolato() {
        assert_eq!(
            greeting_of_the_day(NaiveDate::from_ymd_opt(2023, 6, 17).unwrap(), true),
            Greeting::CuoreImmacolatoDiMaria
        );
    }

    #[test]
    fn should_get_greeting_of_the_day_repubblica() {
        assert_eq!(
            greeting_of_the_day(NaiveDate::from_ymd_opt(2023, 6, 2).unwrap(), true),
            Greeting::FestaDellaRepubblica
        );
    }

    #[test]
    fn should_get_greeting_of_the_day_25_aprile() {
        assert_eq!(
            greeting_of_the_day(NaiveDate::from_ymd_opt(2023, 4, 25).unwrap(), true),
            Greeting::Liberazione
        );
    }

    #[test]
    fn should_get_greeting_of_the_day_1_maggio() {
        assert_eq!(
            greeting_of_the_day(NaiveDate::from_ymd_opt(2023, 5, 1).unwrap(), true),
            Greeting::FestaDeiLavoratori
        );
    }

    #[test]
    fn should_get_greeting_of_the_day_ferragosto() {
        assert_eq!(
            greeting_of_the_day(NaiveDate::from_ymd_opt(2023, 8, 15).unwrap(), true),
            Greeting::Ferragosto
        );
    }

    #[test]
    fn should_get_greeting_of_the_day_halloween() {
        assert_eq!(
            greeting_of_the_day(NaiveDate::from_ymd_opt(2023, 10, 31).unwrap(), true),
            Greeting::Halloween
        );
    }

    #[test]
    fn should_get_greeting_of_the_day_ognissanti() {
        assert_eq!(
            greeting_of_the_day(NaiveDate::from_ymd_opt(2023, 11, 1).unwrap(), true),
            Greeting::Ognissanti
        );
    }

    #[test]
    fn should_get_greeting_of_the_day_defunti() {
        assert_eq!(
            greeting_of_the_day(NaiveDate::from_ymd_opt(2023, 11, 2).unwrap(), true),
            Greeting::Defunti
        );
    }

    #[test]
    fn should_get_greeting_of_the_day_immacolata() {
        assert_eq!(
            greeting_of_the_day(NaiveDate::from_ymd_opt(2023, 12, 8).unwrap(), true),
            Greeting::ImmacolataConcenzione
        );
    }

    #[test]
    fn should_get_greeting_of_the_day_vigilia() {
        assert_eq!(
            greeting_of_the_day(NaiveDate::from_ymd_opt(2023, 12, 24).unwrap(), true),
            Greeting::VigiliaDiNatale
        );
    }

    #[test]
    fn should_get_greeting_of_the_day_natale() {
        assert_eq!(
            greeting_of_the_day(NaiveDate::from_ymd_opt(2023, 12, 25).unwrap(), true),
            Greeting::Natale
        );
    }

    #[test]
    fn should_get_greeting_of_the_day_santostefano() {
        assert_eq!(
            greeting_of_the_day(NaiveDate::from_ymd_opt(2023, 12, 26).unwrap(), true),
            Greeting::SantoStefano
        );
    }
}
