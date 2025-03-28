//! # Ticondivido provider
//!
//! This provider provides images from <https://ticondivido.it>

use std::str::FromStr;

use async_trait::async_trait;
use chrono::Weekday;
use const_format::concatcp;
use scraper::{Html, Selector};

use super::{Greeting, Scrape, ScrapeError, ScrapeResult, Url};

const BASE_URL: &str = "https://ticondivido.it";
const BUONGIORNO_URL: &str = concatcp!(BASE_URL, "/immagini-buongiorno/");

// weekdays
const BUONGIORNO_LUNEDI_URL: &str = concatcp!(BASE_URL, "/immagini-buon-lunedi/");
const BUONGIORNO_MARTEDI_URL: &str = concatcp!(BASE_URL, "/immagini-buon-martedi/");
const BUONGIORNO_MERCOLEDI_URL: &str = concatcp!(BASE_URL, "/immagini-buon-mercoledi/");
const BUONGIORNO_GIOVEDI_URL: &str = concatcp!(BASE_URL, "/immagini-buon-giovedi/");
const BUONGIORNO_VENERDI_URL: &str = concatcp!(BASE_URL, "/immagini-buon-venerdi/");
const BUONGIORNO_SABATO_URL: &str = concatcp!(BASE_URL, "/immagini-buon-sabato/");
const BUONGIORNO_DOMENICA_URL: &str = concatcp!(BASE_URL, "/buona-domenica/");

const WEEKEND_URL: &str = concatcp!(BASE_URL, "/buon-weekend/");
const BUON_COMPLEANNO_URL: &str = concatcp!(BASE_URL, "/buon-compleanno/");

const BUON_POMERIGGIO_URL: &str = concatcp!(BASE_URL, "/immagini-buon-pomeriggio/");
const BUONA_SERATA_URL: &str = concatcp!(BASE_URL, "/immagini-buona-serata/");
const BUONA_NOTTE_URL: &str = concatcp!(BASE_URL, "/immagini-buonanotte/");

// festivita
const CAPODANNO_URL: &str = concatcp!(BASE_URL, "/immagini-buon-anno-nuovo/");
const BEFANA_URL: &str = concatcp!(BASE_URL, "/immagini-befana/");
const SAN_VALENTINO_URL: &str = concatcp!(BASE_URL, "/immagini-buon-san-valentino/");
const CARNEVALE_URL: &str = concatcp!(BASE_URL, "/immagini-buon-carnevale/");
const FESTA_DELLA_DONNA_URL: &str = concatcp!(BASE_URL, "/immagini-festa-della-donna/");
const FESTA_DEL_PAPA_URL: &str = concatcp!(BASE_URL, "/immagini-festa-del-papa/");
const PALME_URL: &str = concatcp!(BASE_URL, "/immagini-domenica-delle-palme/");
const PASQUA_URL: &str = concatcp!(BASE_URL, "/immagini-buona-pasqua/");
const PASQUETTA_URL: &str = concatcp!(BASE_URL, "/immagini-buona-pasquetta/");
const URL_25_APRILE_URL: &str = concatcp!(BASE_URL, "/immagini-buon-25-aprile/");
const PRIMO_MAGGIO_URL: &str = concatcp!(BASE_URL, "/immagini-buon-1-maggio/");
const FESTA_DELLA_MAMMA_URL: &str = concatcp!(BASE_URL, "/immagini-festa-della-mamma/");
const DUE_GIUGNO_URL: &str = concatcp!(BASE_URL, "/immagini-buon-2-giugno/");
const FERRAGOSTO_URL: &str = concatcp!(BASE_URL, "/immagini-buon-ferragosto/");
const HALLOWEEEN_URL: &str = concatcp!(BASE_URL, "/immagini-halloween/");
const OGNISSANTI_URL: &str = concatcp!(BASE_URL, "/immagini-1-novembre/");
const DEFUNTI_URL: &str = concatcp!(BASE_URL, "/immagini-2-novembre/");
const IMMACOLATA_URL: &str = concatcp!(BASE_URL, "/immagini-immacolata-concezione/");
const SAN_NICOLA_URL: &str = concatcp!(BASE_URL, "/immagini-san-nicola/");
const SANTAMBROGIO_URL: &str = concatcp!(BASE_URL, "/immagini-sant-ambrogio/");
const BUON_NATALE_URL: &str = concatcp!(BASE_URL, "/buon-natale/");
const SANTO_STEFANO_URL: &str = concatcp!(BASE_URL, "/immagini-santo-stefano-26-dicembre/");
const SAN_SILVESTRO_URL: &str = concatcp!(BASE_URL, "/immagini-vigilia-di-capodanno/");
const SANTA_LUCIA_URL: &str = concatcp!(BASE_URL, "/immagini-santa-lucia/");

/// Buongiornissimo provider which scrapes images from <https://ticondivido.it>
///
/// Supported [`Greeting`]s:
///
/// - [`Greeting::BuonGiorno`]
/// - [`Greeting::BuonGiornoWeekday`]
/// - [`Greeting::BuonGiornoWeekday`]
/// - [`Greeting::BuonGiornoWeekday`]
/// - [`Greeting::BuonGiornoWeekday`]
/// - [`Greeting::BuonGiornoWeekday`]
/// - [`Greeting::BuonGiornoWeekday`]
/// - [`Greeting::BuonGiornoWeekday`]
/// - [`Greeting::Weekend`]
/// - [`Greeting::BuonPomeriggio`]
/// - [`Greeting::BuonaNotte`]
/// - [`Greeting::BuonaSerata`]
/// - [`Greeting::Compleanno`]
/// - [`Greeting::Capodanno`]
/// - [`Greeting::Epifania`]
/// - [`Greeting::SanValentino`]
/// - [`Greeting::GiovediGrasso`]
/// - [`Greeting::MartediGrasso`]
/// - [`Greeting::FestaDelleDonne`]
/// - [`Greeting::FestaDelPapa`]
/// - [`Greeting::DomenicaDellePalme`]
/// - [`Greeting::Pasqua`]
/// - [`Greeting::Pasquetta`]
/// - [`Greeting::Liberazione`]
/// - [`Greeting::FestaDeiLavoratori`]
/// - [`Greeting::FestaDellaMamma`]
/// - [`Greeting::DueGiugno`]
/// - [`Greeting::Ferragosto`]
/// - [`Greeting::Halloween`]
/// - [`Greeting::Ognissanti`]
/// - [`Greeting::Defunti`]
/// - [`Greeting::ImmacolataConcenzione`]
/// - [`Greeting::SanNicola`]
/// - [`Greeting::SantAmbrogio`]
/// - [`Greeting::Natale`]
/// - [`Greeting::SantoStefano`]
/// - [`Greeting::SanSilvestro`]
/// - [`Greeting::SantaLucia`]
#[derive(Default)]
pub struct TiCondivido;

impl TiCondivido {
    fn get_url(greeting: Greeting) -> ScrapeResult<String> {
        match greeting {
            Greeting::BuonGiorno => Ok(BUONGIORNO_URL.to_string()),
            Greeting::BuonGiornoWeekday(Weekday::Sun) => Ok(BUONGIORNO_DOMENICA_URL.to_string()),
            Greeting::BuonGiornoWeekday(Weekday::Mon) => Ok(BUONGIORNO_LUNEDI_URL.to_string()),
            Greeting::BuonGiornoWeekday(Weekday::Tue) => Ok(BUONGIORNO_MARTEDI_URL.to_string()),
            Greeting::BuonGiornoWeekday(Weekday::Wed) => Ok(BUONGIORNO_MERCOLEDI_URL.to_string()),
            Greeting::BuonGiornoWeekday(Weekday::Thu) => Ok(BUONGIORNO_GIOVEDI_URL.to_string()),
            Greeting::BuonGiornoWeekday(Weekday::Fri) => Ok(BUONGIORNO_VENERDI_URL.to_string()),
            Greeting::BuonGiornoWeekday(Weekday::Sat) => Ok(BUONGIORNO_SABATO_URL.to_string()),
            Greeting::Weekend => Ok(WEEKEND_URL.to_string()),
            Greeting::BuonPomeriggio => Ok(BUON_POMERIGGIO_URL.to_string()),
            Greeting::BuonaNotte => Ok(BUONA_NOTTE_URL.to_string()),
            Greeting::BuonaSerata => Ok(BUONA_SERATA_URL.to_string()),
            Greeting::Compleanno => Ok(BUON_COMPLEANNO_URL.to_string()),
            Greeting::Capodanno => Ok(CAPODANNO_URL.to_string()),
            Greeting::Epifania => Ok(BEFANA_URL.to_string()),
            Greeting::SanValentino => Ok(SAN_VALENTINO_URL.to_string()),
            Greeting::GiovediGrasso | Greeting::MartediGrasso => Ok(CARNEVALE_URL.to_string()),
            Greeting::FestaDelleDonne => Ok(FESTA_DELLA_DONNA_URL.to_string()),
            Greeting::FestaDelPapa => Ok(FESTA_DEL_PAPA_URL.to_string()),
            Greeting::DomenicaDellePalme => Ok(PALME_URL.to_string()),
            Greeting::Pasqua => Ok(PASQUA_URL.to_string()),
            Greeting::Pasquetta => Ok(PASQUETTA_URL.to_string()),
            Greeting::Liberazione => Ok(URL_25_APRILE_URL.to_string()),
            Greeting::FestaDeiLavoratori => Ok(PRIMO_MAGGIO_URL.to_string()),
            Greeting::FestaDellaMamma => Ok(FESTA_DELLA_MAMMA_URL.to_string()),
            Greeting::DueGiugno => Ok(DUE_GIUGNO_URL.to_string()),
            Greeting::Ferragosto => Ok(FERRAGOSTO_URL.to_string()),
            Greeting::Halloween => Ok(HALLOWEEEN_URL.to_string()),
            Greeting::Ognissanti => Ok(OGNISSANTI_URL.to_string()),
            Greeting::Defunti => Ok(DEFUNTI_URL.to_string()),
            Greeting::ImmacolataConcenzione => Ok(IMMACOLATA_URL.to_string()),
            Greeting::SanNicola => Ok(SAN_NICOLA_URL.to_string()),
            Greeting::SantAmbrogio => Ok(SANTAMBROGIO_URL.to_string()),
            Greeting::Natale => Ok(BUON_NATALE_URL.to_string()),
            Greeting::SantoStefano => Ok(SANTO_STEFANO_URL.to_string()),
            Greeting::SanSilvestro => Ok(SAN_SILVESTRO_URL.to_string()),
            Greeting::SantaLucia => Ok(SANTA_LUCIA_URL.to_string()),
            _ => Err(ScrapeError::UnsupportedGreeting),
        }
    }
}

#[async_trait]
impl Scrape for TiCondivido {
    async fn scrape(&self, greeting: Greeting) -> ScrapeResult<Vec<Url>> {
        let url = Self::get_url(greeting)?;
        debug!("scraping greeting of kind {:?} at {}", greeting, url);
        // send request
        let body = reqwest::get(&url).await?.text().await?;
        debug!("got body of length {}", body.len());
        trace!("body: {}", body);
        // parse document
        let document = Html::parse_document(&body);
        debug!("html document parsed");
        // search for entry content selector
        let main_selector = Selector::parse(r#"div[class="entry-content clear"]"#).unwrap();
        let mut main_elements = document.select(&main_selector);
        let main = main_elements.next();
        let Some(main) = main else {
            error!("main is none");
            return Err(ScrapeError::UnexpectedHtml(
                "could not find a div with class entry-content".to_string(),
            ));
        };

        debug!("selecting images in thumbnail");
        let mut urls = Vec::new();
        let img_selector = Selector::parse("img").unwrap();
        let images = main.select(&img_selector);
        for image in images {
            if let Some(Ok(url)) = image
                .value()
                .attr("data-src")
                .filter(|s| s.starts_with("http") || s.starts_with("https"))
                .map(Url::from_str)
            {
                debug!("found image with url {}", url);
                urls.push(url)
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

    #[tokio::test]
    async fn test_buongiorno() {
        crate::test_log();
        let provider = TiCondivido::default();
        let urls = provider.scrape(Greeting::BuonGiorno).await.unwrap();
        assert!(!urls.is_empty());
    }

    #[tokio::test]
    async fn test_buongiorno_weekday() {
        crate::test_log();
        let provider = TiCondivido::default();
        let urls = provider
            .scrape(Greeting::BuonGiornoWeekday(Weekday::Mon))
            .await
            .unwrap();
        assert!(!urls.is_empty());
    }

    #[tokio::test]
    async fn test_weekend() {
        crate::test_log();
        let provider = TiCondivido::default();
        let urls = provider.scrape(Greeting::Weekend).await.unwrap();
        assert!(!urls.is_empty());
    }

    #[tokio::test]
    async fn test_buona_notte() {
        crate::test_log();
        let provider = TiCondivido::default();
        let urls = provider.scrape(Greeting::BuonaNotte).await.unwrap();
        assert!(!urls.is_empty());
    }

    #[tokio::test]
    async fn test_buon_pomeriggio() {
        crate::test_log();
        let provider = TiCondivido::default();
        let urls = provider.scrape(Greeting::BuonPomeriggio).await.unwrap();
        assert!(!urls.is_empty());
    }

    #[tokio::test]
    async fn test_buona_serata() {
        crate::test_log();
        let provider = TiCondivido::default();
        let urls = provider.scrape(Greeting::BuonaSerata).await.unwrap();
        assert!(!urls.is_empty());
    }

    #[tokio::test]
    async fn test_compleanno() {
        crate::test_log();
        let provider = TiCondivido::default();
        let urls = provider.scrape(Greeting::Compleanno).await.unwrap();
        assert!(!urls.is_empty());
    }

    #[tokio::test]
    async fn test_capodanno() {
        crate::test_log();
        let provider = TiCondivido::default();
        let urls = provider.scrape(Greeting::Capodanno).await.unwrap();
        assert!(!urls.is_empty());
    }

    #[tokio::test]
    async fn test_epifania() {
        crate::test_log();
        let provider = TiCondivido::default();
        let urls = provider.scrape(Greeting::Epifania).await.unwrap();
        assert!(!urls.is_empty());
    }

    #[tokio::test]
    async fn test_san_valentino() {
        crate::test_log();
        let provider = TiCondivido::default();
        let urls = provider.scrape(Greeting::SanValentino).await.unwrap();
        assert!(!urls.is_empty());
    }

    #[tokio::test]
    async fn test_carnevale() {
        crate::test_log();
        let provider = TiCondivido::default();
        let urls = provider.scrape(Greeting::GiovediGrasso).await.unwrap();
        assert!(!urls.is_empty());
    }

    #[tokio::test]
    async fn test_festa_delle_donne() {
        crate::test_log();
        let provider = TiCondivido::default();
        let urls = provider.scrape(Greeting::FestaDelleDonne).await.unwrap();
        assert!(!urls.is_empty());
    }

    #[tokio::test]
    async fn test_festa_del_papa() {
        crate::test_log();
        let provider = TiCondivido::default();
        let urls = provider.scrape(Greeting::FestaDelPapa).await.unwrap();
        assert!(!urls.is_empty());
    }

    #[tokio::test]
    async fn test_domenica_delle_palme() {
        crate::test_log();
        let provider = TiCondivido::default();
        let urls = provider.scrape(Greeting::DomenicaDellePalme).await.unwrap();
        assert!(!urls.is_empty());
    }

    #[tokio::test]
    async fn test_pasqua() {
        crate::test_log();
        let provider = TiCondivido::default();
        let urls = provider.scrape(Greeting::Pasqua).await.unwrap();
        assert!(!urls.is_empty());
    }

    #[tokio::test]
    async fn test_pasquetta() {
        crate::test_log();
        let provider = TiCondivido::default();
        let urls = provider.scrape(Greeting::Pasquetta).await.unwrap();
        assert!(!urls.is_empty());
    }

    #[tokio::test]
    async fn test_liberazione() {
        crate::test_log();
        let provider = TiCondivido::default();
        let urls = provider.scrape(Greeting::Liberazione).await.unwrap();
        assert!(!urls.is_empty());
    }

    #[tokio::test]
    async fn test_festa_dei_lavoratori() {
        crate::test_log();
        let provider = TiCondivido::default();
        let urls = provider.scrape(Greeting::FestaDeiLavoratori).await.unwrap();
        assert!(!urls.is_empty());
    }

    #[tokio::test]
    async fn test_festa_della_mamma() {
        crate::test_log();
        let provider = TiCondivido::default();
        let urls = provider.scrape(Greeting::FestaDellaMamma).await.unwrap();
        assert!(!urls.is_empty());
    }

    #[tokio::test]
    async fn test_due_giugno() {
        crate::test_log();
        let provider = TiCondivido::default();
        let urls = provider.scrape(Greeting::DueGiugno).await.unwrap();
        assert!(!urls.is_empty());
    }

    #[tokio::test]
    async fn test_ferragosto() {
        crate::test_log();
        let provider = TiCondivido::default();
        let urls = provider.scrape(Greeting::Ferragosto).await.unwrap();
        assert!(!urls.is_empty());
    }

    #[tokio::test]
    async fn test_halloween() {
        crate::test_log();
        let provider = TiCondivido::default();
        let urls = provider.scrape(Greeting::Halloween).await.unwrap();
        assert!(!urls.is_empty());
    }

    #[tokio::test]
    async fn test_ognissanti() {
        crate::test_log();
        let provider = TiCondivido::default();
        let urls = provider.scrape(Greeting::Ognissanti).await.unwrap();
        assert!(!urls.is_empty());
    }

    #[tokio::test]
    async fn test_defunti() {
        crate::test_log();
        let provider = TiCondivido::default();
        let urls = provider.scrape(Greeting::Defunti).await.unwrap();
        assert!(!urls.is_empty());
    }

    #[tokio::test]
    async fn test_immacolata_concenzione() {
        crate::test_log();
        let provider = TiCondivido::default();
        let urls = provider
            .scrape(Greeting::ImmacolataConcenzione)
            .await
            .unwrap();
        assert!(!urls.is_empty());
    }

    #[tokio::test]
    async fn test_san_nicola() {
        crate::test_log();
        let provider = TiCondivido::default();
        let urls = provider.scrape(Greeting::SanNicola).await.unwrap();
        assert!(!urls.is_empty());
    }

    #[tokio::test]
    async fn test_sant_ambrogio() {
        crate::test_log();
        let provider = TiCondivido::default();
        let urls = provider.scrape(Greeting::SantAmbrogio).await.unwrap();
        assert!(!urls.is_empty());
    }

    #[tokio::test]
    async fn test_natale() {
        crate::test_log();
        let provider = TiCondivido::default();
        let urls = provider.scrape(Greeting::Natale).await.unwrap();
        assert!(!urls.is_empty());
    }

    #[tokio::test]
    async fn test_santo_stefano() {
        crate::test_log();
        let provider = TiCondivido::default();
        let urls = provider.scrape(Greeting::SantoStefano).await.unwrap();
        assert!(!urls.is_empty());
    }

    #[tokio::test]
    async fn test_san_silvestro() {
        crate::test_log();
        let provider = TiCondivido::default();
        let urls = provider.scrape(Greeting::SanSilvestro).await.unwrap();
        assert!(!urls.is_empty());
    }
}
