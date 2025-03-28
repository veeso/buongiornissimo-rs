//! # Buongiornissimo caffe
//!
//! This provider provides images from <https://buongiornoimmagini.it/>

use std::str::FromStr;

use async_trait::async_trait;
use chrono::Weekday;
use const_format::concatcp;
use scraper::{Html, Selector};

use super::{Greeting, Scrape, ScrapeError, ScrapeResult, Url};

const BASE_URL: &str = "https://buongiornoimmagini.it";
const BUONGIORNO_URL: &str = concatcp!(BASE_URL, "/immagini-buongiorno/");

// weekdays
const BUONGIORNO_LUNEDI_URL: &str = concatcp!(BASE_URL, "/buon-giorno-lunedi/");
const BUONGIORNO_MARTEDI_URL: &str = concatcp!(BASE_URL, "/buon-giorno-buon-martedi/");
const BUONGIORNO_MERCOLEDI_URL: &str = concatcp!(BASE_URL, "/buon-giorno-mercoledi/");
const BUONGIORNO_GIOVEDI_URL: &str = concatcp!(BASE_URL, "/buon-giorno-buon-giovedi/");
const BUONGIORNO_VENERDI_URL: &str = concatcp!(BASE_URL, "/buon-giorno-buon-venerdi/");
const BUONGIORNO_SABATO_URL: &str = concatcp!(BASE_URL, "/buon-giorno-buon-sabato/");
const BUONGIORNO_DOMENICA_URL: &str = concatcp!(BASE_URL, "/buona-domenica/");
const WEEKEND_URL: &str = concatcp!(BASE_URL, "/buon-weekend/");

const BUON_PRANZO_URL: &str = concatcp!(BASE_URL, "/buon-pranzo/");
const BUONA_NOTTE_URL: &str = concatcp!(BASE_URL, "/buonanotte/");
const BUONA_SERATA_URL: &str = concatcp!(BASE_URL, "/buona-serata/");
const BUONA_CENA_URL: &str = concatcp!(BASE_URL, "/buona-cena/");

/// Buongiornissimo provider which scrapes images from <https://buongiornoimmagini.it>
///
/// Supported [`Greeting`]s:
///
/// - [`Greeting::BuonGiorno`]
/// - [`Greeting::BuonGiornoWeekday`]
/// - [`Greeting::BuonPranzo`]
/// - [`Greeting::BuonaNotte`]
/// - [`Greeting::BuonaSerata`]
/// - [`Greeting::BuonaCena`]
/// - [`Greeting::Weekend`]
#[derive(Default)]
pub struct BuongiornoImmagini;

impl BuongiornoImmagini {
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
            Greeting::BuonPranzo => Ok(BUON_PRANZO_URL.to_string()),
            Greeting::BuonaNotte => Ok(BUONA_NOTTE_URL.to_string()),
            Greeting::BuonaSerata => Ok(BUONA_SERATA_URL.to_string()),
            Greeting::BuonaCena => Ok(BUONA_CENA_URL.to_string()),
            _ => Err(ScrapeError::UnsupportedGreeting),
        }
    }
}

#[async_trait]
impl Scrape for BuongiornoImmagini {
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
        let main_selector = Selector::parse(r#"main"#).unwrap();
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
            if let Some(Ok(url)) = image.value().attr("src").map(Url::from_str) {
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
    async fn should_get_goodmorning_images() {
        assert!(
            !BuongiornoImmagini::default()
                .scrape(Greeting::BuonGiorno)
                .await
                .unwrap()
                .is_empty()
        );
    }

    #[tokio::test]
    async fn test_should_get_buon_pranzo() {
        assert!(
            !BuongiornoImmagini::default()
                .scrape(Greeting::BuonPranzo)
                .await
                .unwrap()
                .is_empty()
        );
    }

    #[tokio::test]
    async fn test_should_get_buona_notte() {
        assert!(
            !BuongiornoImmagini::default()
                .scrape(Greeting::BuonaNotte)
                .await
                .unwrap()
                .is_empty()
        );
    }

    #[tokio::test]
    async fn test_should_get_buona_serata() {
        assert!(
            !BuongiornoImmagini::default()
                .scrape(Greeting::BuonaSerata)
                .await
                .unwrap()
                .is_empty()
        );
    }

    #[tokio::test]
    async fn test_should_get_buona_cena() {
        assert!(
            !BuongiornoImmagini::default()
                .scrape(Greeting::BuonaCena)
                .await
                .unwrap()
                .is_empty()
        );
    }

    #[tokio::test]
    async fn test_should_get_weekend() {
        assert!(
            !BuongiornoImmagini::default()
                .scrape(Greeting::Weekend)
                .await
                .unwrap()
                .is_empty()
        );
    }

    #[tokio::test]
    async fn should_get_weekday_images() {
        assert!(
            !BuongiornoImmagini::default()
                .scrape(Greeting::BuonGiornoWeekday(Weekday::Mon))
                .await
                .unwrap()
                .is_empty()
        );
        assert!(
            !BuongiornoImmagini::default()
                .scrape(Greeting::BuonGiornoWeekday(Weekday::Tue))
                .await
                .unwrap()
                .is_empty()
        );
        assert!(
            !BuongiornoImmagini::default()
                .scrape(Greeting::BuonGiornoWeekday(Weekday::Wed))
                .await
                .unwrap()
                .is_empty()
        );
        assert!(
            !BuongiornoImmagini::default()
                .scrape(Greeting::BuonGiornoWeekday(Weekday::Thu))
                .await
                .unwrap()
                .is_empty()
        );
        assert!(
            !BuongiornoImmagini::default()
                .scrape(Greeting::BuonGiornoWeekday(Weekday::Fri))
                .await
                .unwrap()
                .is_empty()
        );
        assert!(
            !BuongiornoImmagini::default()
                .scrape(Greeting::BuonGiornoWeekday(Weekday::Sat))
                .await
                .unwrap()
                .is_empty()
        );
        assert!(
            !BuongiornoImmagini::default()
                .scrape(Greeting::BuonGiornoWeekday(Weekday::Sun))
                .await
                .unwrap()
                .is_empty()
        );
    }
}
