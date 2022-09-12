//! # Il mondo di grazia
//!
//! This provider provides images from <https://ilmondodigrazia.com>

use const_format::concatcp;
use std::str::FromStr;

use super::{Greeting, Scrape, ScrapeError, ScrapeResult, Url};

use async_trait::async_trait;
use chrono::Weekday;
use scraper::{Html, Selector};

const BASE_URL: &str = "https://ilmondodigrazia.com";
const AUGURI_URL: &str = concatcp!(BASE_URL, "/compleanno");
const BUONGIORNO_URL: &str = concatcp!(BASE_URL, "/buongiorno");
const BUONGIORNO_WEEKDAY_BASE_URL: &str = concatcp!(BASE_URL, "/buongiorno/buongiorno-");
const BUON_SABATO_URL: &str = concatcp!(BASE_URL, "/sabato");
const BUONA_DOMENICA_URL: &str = concatcp!(BASE_URL, "/buona-domenica");
const BUON_POMERIGGIO_URL: &str = concatcp!(BASE_URL, "/buon-pomeriggio");
const BUONA_NOTTE_URL: &str = concatcp!(BASE_URL, "/buonanotte");
const GIOVEDI_GRASSO_URL: &str = concatcp!(
    BASE_URL,
    "/buongiorno/buongiorno-giovedi/buon-giovedi-grasso"
);
const PENTECOSTE_URL: &str = concatcp!(BASE_URL, "/buona-domenica/domenica-di-pentecoste");
const DUE_GIUGNO_URL: &str = concatcp!(BASE_URL, "/festa/festa-della-repubblica");
const CORPUS_DOMINI_URL: &str = concatcp!(BASE_URL, "/corpus-domini");
const SACRO_CUORE_DI_GESU_URL: &str = concatcp!(BASE_URL, "/sacro-cuore-di-gesu");
const FERRAGOSTO_URL: &str = concatcp!(BASE_URL, "/buon-ferragosto");
const OGNISSANTI_URL: &str = concatcp!(BASE_URL, "/tutti-i-santi-immagini-festa-di-ognissanti");
const DUE_NOVEMBRE_URL: &str = concatcp!(BASE_URL, "/commemorazione-dei-defunti-2-novembre");
const HALLOWEEN_URL: &str = concatcp!(BASE_URL, "/halloween-31-ottobre-immagini-buongiorno");
const IMMACOLATA_CONCEZIONE_URL: &str =
    concatcp!(BASE_URL, "/immacolata-concezione-8-dicembre-buongiorno");
const VIGILIA_URL: &str = concatcp!(
    BASE_URL,
    "/buongiorno/buongiorno-vigilia-di-natale-per-il-24-dicembre"
);
const BUON_NATALE_URL: &str = concatcp!(BASE_URL, "/frasi-di-buon-natale-immagini-da-condividere");

/// Buongiornissimo provider which scrapes images from <https://ilmondodigrazia.com>
#[derive(Default)]
pub struct IlMondoDiGrazia;

impl IlMondoDiGrazia {
    fn weekday(weekday: chrono::Weekday) -> &'static str {
        match weekday {
            Weekday::Mon => "lunedi",
            Weekday::Tue => "martedi",
            Weekday::Wed => "mercoledi",
            Weekday::Thu => "giovedi",
            Weekday::Fri => "venerdi",
            Weekday::Sat => "sabato",
            Weekday::Sun => "domenica",
        }
    }

    fn buongiorno_weekday_url(weekday: chrono::Weekday) -> String {
        format!("{}{}", BUONGIORNO_WEEKDAY_BASE_URL, Self::weekday(weekday))
    }

    fn get_url(greeting: Greeting) -> ScrapeResult<String> {
        match greeting {
            Greeting::Compleanno => Ok(AUGURI_URL.to_string()),
            Greeting::BuonGiorno => Ok(BUONGIORNO_URL.to_string()),
            Greeting::BuonGiornoWeekday(Weekday::Sat) => Ok(BUON_SABATO_URL.to_string()),
            Greeting::BuonGiornoWeekday(Weekday::Sun) => Ok(BUONA_DOMENICA_URL.to_string()),
            Greeting::BuonGiornoWeekday(weekday) => Ok(Self::buongiorno_weekday_url(weekday)),
            Greeting::BuonPomeriggio => Ok(BUON_POMERIGGIO_URL.to_string()),
            Greeting::BuonaNotte => Ok(BUONA_NOTTE_URL.to_string()),
            Greeting::GiovediGrasso => Ok(GIOVEDI_GRASSO_URL.to_string()),
            Greeting::Pentecoste => Ok(PENTECOSTE_URL.to_string()),
            Greeting::FestaDellaRepubblica => Ok(DUE_GIUGNO_URL.to_string()),
            Greeting::CorpusDomini => Ok(CORPUS_DOMINI_URL.to_string()),
            Greeting::SacroCuoreDiGesu => Ok(SACRO_CUORE_DI_GESU_URL.to_string()),
            Greeting::Ferragosto => Ok(FERRAGOSTO_URL.to_string()),
            Greeting::Ognissanti => Ok(OGNISSANTI_URL.to_string()),
            Greeting::Defunti => Ok(DUE_NOVEMBRE_URL.to_string()),
            Greeting::Halloween => Ok(HALLOWEEN_URL.to_string()),
            Greeting::ImmacolataConcenzione => Ok(IMMACOLATA_CONCEZIONE_URL.to_string()),
            Greeting::VigiliaDiNatale => Ok(VIGILIA_URL.to_string()),
            Greeting::Natale => Ok(BUON_NATALE_URL.to_string()),
            Greeting::Capodanno
            | Greeting::Epifania
            | Greeting::MartediGrasso
            | Greeting::MercolediCeneri
            | Greeting::SanValentino
            | Greeting::FestaDelleDonne
            | Greeting::DomenicaDellePalme
            | Greeting::Pasqua
            | Greeting::Pasquetta
            | Greeting::Liberazione
            | Greeting::FestaDeiLavoratori
            | Greeting::Ascensione
            | Greeting::SantissimaTrinita
            | Greeting::SantoStefano
            | Greeting::CuoreImmacolatoDiMaria => Err(ScrapeError::UnsupportedGreeting),
        }
    }
}

#[async_trait]
impl Scrape for IlMondoDiGrazia {
    async fn scrape(&self, greeting: Greeting) -> ScrapeResult<Vec<Url>> {
        let url = Self::get_url(greeting)?;
        debug!("scraping greeting of kind {:?} at {}", greeting, url);
        // send request
        let body = reqwest::get(&url).await?.text().await?;
        debug!("got body of length {}", body.len());
        // parse document
        let document = Html::parse_document(&body);
        debug!("html document parsed");
        // search for entry content selector
        debug!("searching for entry-content div");
        let entry_content_selector = Selector::parse(r#"div[class="entry-content"]"#).unwrap();
        let mut containers = document.select(&entry_content_selector);
        debug!("getting first container");
        let container = containers.next();
        if container.is_none() {
            error!("container is none");
            return Err(ScrapeError::UnexpectedHtml(
                "could not find a div with class entry-content".to_string(),
            ));
        }
        debug!("selecting images");
        let img_selector = Selector::parse("img").unwrap();
        let images = container.unwrap().select(&img_selector);
        let mut urls: Vec<Url> = Vec::new();
        for image in images {
            if let Some(Ok(url)) = image.value().attr("src").map(Url::from_str) {
                debug!("found image with url {}", url);
                // check domain
                if url
                    .domain()
                    .map(|x| x == "ilmondodigrazia.com")
                    .unwrap_or(false)
                {
                    debug!("image belongs to ilmondodigrazia.com; pushing to urls");
                    urls.push(url)
                }
            }
        }
        if urls.is_empty() {
            error!("urls is empty");
            return Err(ScrapeError::NoImages);
        }
        Ok(urls)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;
    use serial_test::serial;

    #[tokio::test]
    #[cfg(not(feature = "github-ci"))]
    #[serial]
    async fn should_get_birthday_images() {
        assert!(!IlMondoDiGrazia::default()
            .scrape(Greeting::Compleanno)
            .await
            .unwrap()
            .is_empty());
    }

    #[tokio::test]
    #[cfg(not(feature = "github-ci"))]
    #[serial]
    async fn should_get_goodmorning_images() {
        assert!(!IlMondoDiGrazia::default()
            .scrape(Greeting::BuonGiorno)
            .await
            .unwrap()
            .is_empty());
    }

    #[tokio::test]
    #[cfg(not(feature = "github-ci"))]
    #[serial]
    async fn should_get_weekday_images() {
        assert!(!IlMondoDiGrazia::default()
            .scrape(Greeting::BuonGiornoWeekday(Weekday::Mon))
            .await
            .unwrap()
            .is_empty());
        assert!(!IlMondoDiGrazia::default()
            .scrape(Greeting::BuonGiornoWeekday(Weekday::Tue))
            .await
            .unwrap()
            .is_empty());
        assert!(!IlMondoDiGrazia::default()
            .scrape(Greeting::BuonGiornoWeekday(Weekday::Wed))
            .await
            .unwrap()
            .is_empty());
        assert!(!IlMondoDiGrazia::default()
            .scrape(Greeting::BuonGiornoWeekday(Weekday::Thu))
            .await
            .unwrap()
            .is_empty());
        assert!(!IlMondoDiGrazia::default()
            .scrape(Greeting::BuonGiornoWeekday(Weekday::Fri))
            .await
            .unwrap()
            .is_empty());
        assert!(!IlMondoDiGrazia::default()
            .scrape(Greeting::BuonGiornoWeekday(Weekday::Sat))
            .await
            .unwrap()
            .is_empty());
        assert!(!IlMondoDiGrazia::default()
            .scrape(Greeting::BuonGiornoWeekday(Weekday::Sun))
            .await
            .unwrap()
            .is_empty());
    }

    #[tokio::test]
    #[cfg(not(feature = "github-ci"))]
    #[serial]
    async fn should_get_christmas_images() {
        assert!(!IlMondoDiGrazia::default()
            .scrape(Greeting::Natale)
            .await
            .unwrap()
            .is_empty());
    }

    #[tokio::test]
    #[cfg(not(feature = "github-ci"))]
    #[serial]
    async fn should_get_afternoon_images() {
        assert!(!IlMondoDiGrazia::default()
            .scrape(Greeting::BuonPomeriggio)
            .await
            .unwrap()
            .is_empty());
    }

    #[tokio::test]
    #[cfg(not(feature = "github-ci"))]
    #[serial]
    async fn should_get_greeting_giovedi_grasso_images() {
        assert!(!IlMondoDiGrazia::default()
            .scrape(Greeting::GiovediGrasso)
            .await
            .unwrap()
            .is_empty());
    }

    #[tokio::test]
    #[cfg(not(feature = "github-ci"))]
    #[serial]
    async fn should_get_greeting_pentecoste_images() {
        assert!(!IlMondoDiGrazia::default()
            .scrape(Greeting::Pentecoste)
            .await
            .unwrap()
            .is_empty());
    }

    #[tokio::test]
    #[cfg(not(feature = "github-ci"))]
    #[serial]
    async fn should_get_greeting_festa_della_repubblica_images() {
        assert!(!IlMondoDiGrazia::default()
            .scrape(Greeting::FestaDellaRepubblica)
            .await
            .unwrap()
            .is_empty());
    }

    #[tokio::test]
    #[cfg(not(feature = "github-ci"))]
    #[serial]
    async fn should_get_greeting_corpus_domini_images() {
        assert!(!IlMondoDiGrazia::default()
            .scrape(Greeting::CorpusDomini)
            .await
            .unwrap()
            .is_empty());
    }

    #[tokio::test]
    #[cfg(not(feature = "github-ci"))]
    #[serial]
    async fn should_get_greeting_sacro_cuore_di_gesu_images() {
        assert!(!IlMondoDiGrazia::default()
            .scrape(Greeting::SacroCuoreDiGesu)
            .await
            .unwrap()
            .is_empty());
    }

    #[tokio::test]
    #[cfg(not(feature = "github-ci"))]
    #[serial]
    async fn should_get_greeting_ferragosto_images() {
        assert!(!IlMondoDiGrazia::default()
            .scrape(Greeting::Ferragosto)
            .await
            .unwrap()
            .is_empty());
    }

    #[tokio::test]
    #[cfg(not(feature = "github-ci"))]
    #[serial]
    async fn should_get_greeting_ognissanti_images() {
        assert!(!IlMondoDiGrazia::default()
            .scrape(Greeting::Ognissanti)
            .await
            .unwrap()
            .is_empty());
    }

    #[tokio::test]
    #[cfg(not(feature = "github-ci"))]
    #[serial]
    async fn should_get_greeting_defunti_images() {
        assert!(!IlMondoDiGrazia::default()
            .scrape(Greeting::Defunti)
            .await
            .unwrap()
            .is_empty());
    }

    #[tokio::test]
    #[cfg(not(feature = "github-ci"))]
    #[serial]
    async fn should_get_greeting_halloween_images() {
        assert!(!IlMondoDiGrazia::default()
            .scrape(Greeting::Halloween)
            .await
            .unwrap()
            .is_empty());
    }

    #[tokio::test]
    #[cfg(not(feature = "github-ci"))]
    #[serial]
    async fn should_get_greeting_immacolata_concenzione_images() {
        assert!(!IlMondoDiGrazia::default()
            .scrape(Greeting::ImmacolataConcenzione)
            .await
            .unwrap()
            .is_empty());
    }

    #[tokio::test]
    #[cfg(not(feature = "github-ci"))]
    #[serial]
    async fn should_get_greeting_vigilia_di_natale_images() {
        assert!(!IlMondoDiGrazia::default()
            .scrape(Greeting::VigiliaDiNatale)
            .await
            .unwrap()
            .is_empty());
    }

    #[tokio::test]
    #[cfg(not(feature = "github-ci"))]
    #[serial]
    async fn should_get_greeting_natale_images() {
        assert!(!IlMondoDiGrazia::default()
            .scrape(Greeting::Natale)
            .await
            .unwrap()
            .is_empty());
    }

    #[tokio::test]
    async fn should_be_unsupported_greeting_capodanno() {
        assert_eq!(
            IlMondoDiGrazia::default()
                .scrape(Greeting::Capodanno)
                .await
                .err()
                .unwrap(),
            ScrapeError::UnsupportedGreeting
        );
    }
    #[tokio::test]
    async fn should_be_unsupported_greeting_epifania() {
        assert_eq!(
            IlMondoDiGrazia::default()
                .scrape(Greeting::Epifania)
                .await
                .err()
                .unwrap(),
            ScrapeError::UnsupportedGreeting
        );
    }

    #[tokio::test]
    async fn should_be_unsupported_greeting_valentino() {
        assert_eq!(
            IlMondoDiGrazia::default()
                .scrape(Greeting::SanValentino)
                .await
                .err()
                .unwrap(),
            ScrapeError::UnsupportedGreeting
        );
    }

    #[tokio::test]
    async fn should_be_unsupported_greeting_donne() {
        assert_eq!(
            IlMondoDiGrazia::default()
                .scrape(Greeting::FestaDelleDonne)
                .await
                .err()
                .unwrap(),
            ScrapeError::UnsupportedGreeting
        );
    }

    #[tokio::test]
    async fn should_be_unsupported_greeting_martedi_grasso() {
        assert_eq!(
            IlMondoDiGrazia::default()
                .scrape(Greeting::MartediGrasso)
                .await
                .err()
                .unwrap(),
            ScrapeError::UnsupportedGreeting
        );
    }
    #[tokio::test]
    async fn should_be_unsupported_greeting_mercoledi_ceneri() {
        assert_eq!(
            IlMondoDiGrazia::default()
                .scrape(Greeting::MercolediCeneri)
                .await
                .err()
                .unwrap(),
            ScrapeError::UnsupportedGreeting
        );
    }

    #[tokio::test]
    async fn should_be_unsupported_greeting_palme() {
        assert_eq!(
            IlMondoDiGrazia::default()
                .scrape(Greeting::DomenicaDellePalme)
                .await
                .err()
                .unwrap(),
            ScrapeError::UnsupportedGreeting
        );
    }

    #[tokio::test]
    async fn should_be_unsupported_greeting_pasqua() {
        assert_eq!(
            IlMondoDiGrazia::default()
                .scrape(Greeting::Pasqua)
                .await
                .err()
                .unwrap(),
            ScrapeError::UnsupportedGreeting
        );
    }
    #[tokio::test]
    async fn should_be_unsupported_greeting_pasquetta() {
        assert_eq!(
            IlMondoDiGrazia::default()
                .scrape(Greeting::Pasquetta)
                .await
                .err()
                .unwrap(),
            ScrapeError::UnsupportedGreeting
        );
    }
    #[tokio::test]
    async fn should_be_unsupported_greeting_ascensione() {
        assert_eq!(
            IlMondoDiGrazia::default()
                .scrape(Greeting::Ascensione)
                .await
                .err()
                .unwrap(),
            ScrapeError::UnsupportedGreeting
        );
    }
    #[tokio::test]
    async fn should_be_unsupported_greeting_santissima_trinita() {
        assert_eq!(
            IlMondoDiGrazia::default()
                .scrape(Greeting::SantissimaTrinita)
                .await
                .err()
                .unwrap(),
            ScrapeError::UnsupportedGreeting
        );
    }
    #[tokio::test]
    async fn should_be_unsupported_greeting_cuore_immacolato_di_maria() {
        assert_eq!(
            IlMondoDiGrazia::default()
                .scrape(Greeting::CuoreImmacolatoDiMaria)
                .await
                .err()
                .unwrap(),
            ScrapeError::UnsupportedGreeting
        );
    }

    #[tokio::test]
    async fn should_be_unsupported_greeting_25_aprile() {
        assert_eq!(
            IlMondoDiGrazia::default()
                .scrape(Greeting::Liberazione)
                .await
                .err()
                .unwrap(),
            ScrapeError::UnsupportedGreeting
        );
    }

    #[tokio::test]
    async fn should_be_unsupported_greeting_lavoratori() {
        assert_eq!(
            IlMondoDiGrazia::default()
                .scrape(Greeting::FestaDeiLavoratori)
                .await
                .err()
                .unwrap(),
            ScrapeError::UnsupportedGreeting
        );
    }

    #[tokio::test]
    async fn should_be_unsupported_greeting_santostefano() {
        assert_eq!(
            IlMondoDiGrazia::default()
                .scrape(Greeting::SantoStefano)
                .await
                .err()
                .unwrap(),
            ScrapeError::UnsupportedGreeting
        );
    }
}
