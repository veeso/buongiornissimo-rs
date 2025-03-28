//! # Buongiornissimo caffe
//!
//! This provider provides images from <https://www.buongiornissimocaffe.it/>

use std::str::FromStr;

use async_trait::async_trait;
use chrono::Weekday;
use const_format::concatcp;
use scraper::{Html, Selector};

use super::{Greeting, Scrape, ScrapeError, ScrapeResult, Url};

const BASE_URL: &str = "https://www.buongiornissimocaffe.it/category";
const BUONGIORNO_URL: &str = concatcp!(BASE_URL, "/immagini-buongiorno/");
const BUONGIORNO_WEEKDAY_BASE_URL: &str = concatcp!(BASE_URL, "/buon-");
const BUONA_DOMENICA_URL: &str = concatcp!(BASE_URL, "/buona-domenica/");
const BUONA_NOTTE_URL: &str = concatcp!(BASE_URL, "/immagini-buonanotte/");
const BUONA_SERATA_URL: &str = concatcp!(BASE_URL, "/buona-serata/");
// festività
const CAPODANNO_URL: &str = concatcp!(BASE_URL, "/buon-anno/");
const BEFANA_URL: &str = concatcp!(BASE_URL, "/befana/");
const SAN_VALENTINO_URL: &str = concatcp!(BASE_URL, "/auguri-san-valentino/");
const GIOVEDI_GRASSO_URL: &str = concatcp!(BASE_URL, "/buon-giovedi-grasso/");
const MARTEDI_GRASSO_URL: &str = concatcp!(BASE_URL, "/buon-martedi-grasso/");
const FESTA_DELLE_DONNE: &str = concatcp!(BASE_URL, "/auguri-festa-delle-donne/");
const PALME_URL: &str = concatcp!(BASE_URL, "/buona-domenica-delle-palme/");
const PASQUA_URL: &str = concatcp!(BASE_URL, "/buona-pasqua/");
const PASQUETTA_URL: &str = concatcp!(BASE_URL, "/pasquetta/");
const LIBERAZIONE_URL: &str = concatcp!(BASE_URL, "/buon-25-aprile/");
const PRIMO_MAGGIO_URL: &str = concatcp!(BASE_URL, "/buon-1-maggio/");
const HALLOWEEN_URL: &str = concatcp!(BASE_URL, "/halloween/");
const OGNISSANTI_URL: &str = concatcp!(BASE_URL, "/ognissanti/");
const DEFUNTI_URL: &str = concatcp!(BASE_URL, "/commemorazione-dei-defunti/");
const IMMACOLATA_CONCEZIONE_URL: &str = concatcp!(BASE_URL, "/8-dicembre/");
const VIGILIA_URL: &str = concatcp!(BASE_URL, "/24-dicembre/");
const NATALE_URL: &str = concatcp!(BASE_URL, "/buon-natale/");
const SANTO_STEFANO_URL: &str = concatcp!(BASE_URL, "/santo-stefano/");

/// Buongiornissimo provider which scrapes images from <https://www.buongiornissimocaffe.it>.
///
/// Supported [`Greeting`]s:
///
/// - [`Greeting::BuonGiorno`]
/// - [`Greeting::BuonGiornoWeekday`]
/// - [`Greeting::BuonaSerata`]
/// - [`Greeting::BuonaNotte`]
/// - [`Greeting::Capodanno`]
/// - [`Greeting::Epifania`]
/// - [`Greeting::SanValentino`]
/// - [`Greeting::GiovediGrasso`]
/// - [`Greeting::MartediGrasso`]
/// - [`Greeting::FestaDelleDonne`]
/// - [`Greeting::DomenicaDellePalme`]
/// - [`Greeting::Pasqua`]
/// - [`Greeting::Pasquetta`]
/// - [`Greeting::Liberazione`]
/// - [`Greeting::FestaDeiLavoratori`]
/// - [`Greeting::Halloween`]
/// - [`Greeting::Ognissanti`]
/// - [`Greeting::Defunti`]
/// - [`Greeting::ImmacolataConcenzione`]
/// - [`Greeting::VigiliaDiNatale`]
/// - [`Greeting::Natale`]
/// - [`Greeting::SantoStefano`]
#[derive(Default)]
pub struct BuongiornissimoCaffe;

impl BuongiornissimoCaffe {
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

    fn get_url(greeting: Greeting) -> ScrapeResult<String> {
        match greeting {
            Greeting::BuonGiorno => Ok(BUONGIORNO_URL.to_string()),
            Greeting::BuonGiornoWeekday(Weekday::Sun) => Ok(BUONA_DOMENICA_URL.to_string()),
            Greeting::BuonGiornoWeekday(weekday) => Ok(format!(
                "{}{}/",
                BUONGIORNO_WEEKDAY_BASE_URL,
                Self::weekday(weekday)
            )),
            Greeting::BuonaSerata => Ok(BUONA_SERATA_URL.to_string()),
            Greeting::BuonaNotte => Ok(BUONA_NOTTE_URL.to_string()),
            Greeting::Capodanno => Ok(CAPODANNO_URL.to_string()),
            Greeting::Epifania => Ok(BEFANA_URL.to_string()),
            Greeting::SanValentino => Ok(SAN_VALENTINO_URL.to_string()),
            Greeting::GiovediGrasso => Ok(GIOVEDI_GRASSO_URL.to_string()),
            Greeting::MartediGrasso => Ok(MARTEDI_GRASSO_URL.to_string()),
            Greeting::FestaDelleDonne => Ok(FESTA_DELLE_DONNE.to_string()),
            Greeting::DomenicaDellePalme => Ok(PALME_URL.to_string()),
            Greeting::Pasqua => Ok(PASQUA_URL.to_string()),
            Greeting::Pasquetta => Ok(PASQUETTA_URL.to_string()),
            Greeting::Liberazione => Ok(LIBERAZIONE_URL.to_string()),
            Greeting::FestaDeiLavoratori => Ok(PRIMO_MAGGIO_URL.to_string()),
            Greeting::Halloween => Ok(HALLOWEEN_URL.to_string()),
            Greeting::Ognissanti => Ok(OGNISSANTI_URL.to_string()),
            Greeting::Defunti => Ok(DEFUNTI_URL.to_string()),
            Greeting::ImmacolataConcenzione => Ok(IMMACOLATA_CONCEZIONE_URL.to_string()),
            Greeting::VigiliaDiNatale => Ok(VIGILIA_URL.to_string()),
            Greeting::Natale => Ok(NATALE_URL.to_string()),
            Greeting::SantoStefano => Ok(SANTO_STEFANO_URL.to_string()),
            _ => Err(ScrapeError::UnsupportedGreeting),
        }
    }
}

#[async_trait]
impl Scrape for BuongiornissimoCaffe {
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
        if main.is_none() {
            error!("main is none");
            return Err(ScrapeError::UnexpectedHtml(
                "could not find a div with class entry-content".to_string(),
            ));
        }
        debug!("searching for entry-content div");
        let entry_content_selector = Selector::parse(r#"div[class="masonry"]"#).unwrap();
        let mut containers = main.unwrap().select(&entry_content_selector);
        debug!("getting first container");
        let container = containers.next();
        if container.is_none() {
            error!("container is none");
            return Err(ScrapeError::UnexpectedHtml(
                "could not find a div with class entry-content".to_string(),
            ));
        }
        let thumbnail_selector = Selector::parse(r#"div[class="thumbnail"]"#).unwrap();
        let thumbnails = container.unwrap().select(&thumbnail_selector);
        let mut urls: Vec<Url> = Vec::new();
        // search images in thumbnails
        for thumbnail in thumbnails {
            debug!("selecting images in thumbnail");
            let img_selector = Selector::parse("img").unwrap();
            let images = thumbnail.select(&img_selector);
            for image in images {
                if let Some(Ok(url)) = image
                    .value()
                    .attr("src")
                    .filter(|s| s.starts_with("http") || s.starts_with("https"))
                    .map(Url::from_str)
                {
                    debug!("found image with url {}", url);
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

    use pretty_assertions::assert_eq;

    use super::*;

    #[tokio::test]

    async fn should_get_goodmorning_images() {
        assert!(
            !BuongiornissimoCaffe::default()
                .scrape(Greeting::BuonGiorno)
                .await
                .unwrap()
                .is_empty()
        );
    }

    #[tokio::test]
    async fn should_get_weekday_images() {
        assert!(
            !BuongiornissimoCaffe::default()
                .scrape(Greeting::BuonGiornoWeekday(Weekday::Mon))
                .await
                .unwrap()
                .is_empty()
        );
        assert!(
            !BuongiornissimoCaffe::default()
                .scrape(Greeting::BuonGiornoWeekday(Weekday::Tue))
                .await
                .unwrap()
                .is_empty()
        );
        assert!(
            !BuongiornissimoCaffe::default()
                .scrape(Greeting::BuonGiornoWeekday(Weekday::Wed))
                .await
                .unwrap()
                .is_empty()
        );
        assert!(
            !BuongiornissimoCaffe::default()
                .scrape(Greeting::BuonGiornoWeekday(Weekday::Thu))
                .await
                .unwrap()
                .is_empty()
        );
        assert!(
            !BuongiornissimoCaffe::default()
                .scrape(Greeting::BuonGiornoWeekday(Weekday::Fri))
                .await
                .unwrap()
                .is_empty()
        );
        assert!(
            !BuongiornissimoCaffe::default()
                .scrape(Greeting::BuonGiornoWeekday(Weekday::Sat))
                .await
                .unwrap()
                .is_empty()
        );
        assert!(
            !BuongiornissimoCaffe::default()
                .scrape(Greeting::BuonGiornoWeekday(Weekday::Sun))
                .await
                .unwrap()
                .is_empty()
        );
    }

    #[tokio::test]
    async fn should_scrape_for_greeting_buona_notte() {
        assert!(
            !BuongiornissimoCaffe::default()
                .scrape(Greeting::BuonaNotte)
                .await
                .unwrap()
                .is_empty()
        );
    }

    #[tokio::test]
    async fn should_scrape_for_greeting_capodanno() {
        assert!(
            !BuongiornissimoCaffe::default()
                .scrape(Greeting::Capodanno)
                .await
                .unwrap()
                .is_empty()
        );
    }

    #[tokio::test]
    async fn should_scrape_for_greeting_epifania() {
        assert!(
            !BuongiornissimoCaffe::default()
                .scrape(Greeting::Epifania)
                .await
                .unwrap()
                .is_empty()
        );
    }

    #[tokio::test]
    async fn should_scrape_for_greeting_san_valentino() {
        assert!(
            !BuongiornissimoCaffe::default()
                .scrape(Greeting::SanValentino)
                .await
                .unwrap()
                .is_empty()
        );
    }

    #[tokio::test]
    async fn should_scrape_for_greeting_giovedi_grasso() {
        assert!(
            !BuongiornissimoCaffe::default()
                .scrape(Greeting::GiovediGrasso)
                .await
                .unwrap()
                .is_empty()
        );
    }

    #[tokio::test]
    async fn should_scrape_for_greeting_martedi_grasso() {
        assert!(
            !BuongiornissimoCaffe::default()
                .scrape(Greeting::MartediGrasso)
                .await
                .unwrap()
                .is_empty()
        );
    }

    #[tokio::test]
    async fn should_scrape_for_greeting_festa_delle_donne() {
        assert!(
            !BuongiornissimoCaffe::default()
                .scrape(Greeting::FestaDelleDonne)
                .await
                .unwrap()
                .is_empty()
        );
    }

    #[tokio::test]
    async fn should_scrape_for_greeting_domenica_delle_palme() {
        assert!(
            !BuongiornissimoCaffe::default()
                .scrape(Greeting::DomenicaDellePalme)
                .await
                .unwrap()
                .is_empty()
        );
    }

    #[tokio::test]
    async fn should_scrape_for_greeting_pasqua() {
        assert!(
            !BuongiornissimoCaffe::default()
                .scrape(Greeting::Pasqua)
                .await
                .unwrap()
                .is_empty()
        );
    }

    #[tokio::test]
    async fn should_scrape_for_greeting_pasquetta() {
        assert!(
            !BuongiornissimoCaffe::default()
                .scrape(Greeting::Pasquetta)
                .await
                .unwrap()
                .is_empty()
        );
    }

    #[tokio::test]
    async fn should_scrape_for_greeting_liberazione() {
        assert!(
            !BuongiornissimoCaffe::default()
                .scrape(Greeting::Liberazione)
                .await
                .unwrap()
                .is_empty()
        );
    }

    #[tokio::test]
    async fn should_scrape_for_greeting_festa_dei_lavoratori() {
        assert!(
            !BuongiornissimoCaffe::default()
                .scrape(Greeting::FestaDeiLavoratori)
                .await
                .unwrap()
                .is_empty()
        );
    }

    #[tokio::test]
    async fn should_scrape_for_greeting_halloween() {
        assert!(
            !BuongiornissimoCaffe::default()
                .scrape(Greeting::Halloween)
                .await
                .unwrap()
                .is_empty()
        );
    }

    #[tokio::test]
    async fn should_scrape_for_greeting_ognissanti() {
        assert!(
            !BuongiornissimoCaffe::default()
                .scrape(Greeting::Ognissanti)
                .await
                .unwrap()
                .is_empty()
        );
    }

    #[tokio::test]
    async fn should_scrape_for_greeting_defunti() {
        assert!(
            !BuongiornissimoCaffe::default()
                .scrape(Greeting::Defunti)
                .await
                .unwrap()
                .is_empty()
        );
    }

    #[tokio::test]
    async fn should_scrape_for_greeting_immacolata_concenzione() {
        assert!(
            !BuongiornissimoCaffe::default()
                .scrape(Greeting::ImmacolataConcenzione)
                .await
                .unwrap()
                .is_empty()
        );
    }

    #[tokio::test]
    async fn should_scrape_for_greeting_vigilia_di_natale() {
        assert!(
            !BuongiornissimoCaffe::default()
                .scrape(Greeting::VigiliaDiNatale)
                .await
                .unwrap()
                .is_empty()
        );
    }

    #[tokio::test]
    async fn should_scrape_for_greeting_natale() {
        assert!(
            !BuongiornissimoCaffe::default()
                .scrape(Greeting::Natale)
                .await
                .unwrap()
                .is_empty()
        );
    }

    #[tokio::test]
    async fn should_scrape_for_greeting_santo_stefano() {
        assert!(
            !BuongiornissimoCaffe::default()
                .scrape(Greeting::SantoStefano)
                .await
                .unwrap()
                .is_empty()
        );
    }

    #[tokio::test]
    async fn should_not_scrape_for_greeting_compleanno() {
        assert_eq!(
            BuongiornissimoCaffe::default()
                .scrape(Greeting::Compleanno)
                .await
                .err()
                .unwrap(),
            ScrapeError::UnsupportedGreeting
        );
    }

    #[tokio::test]
    async fn should_not_scrape_for_greeting_buon_pomeriggio() {
        assert_eq!(
            BuongiornissimoCaffe::default()
                .scrape(Greeting::BuonPomeriggio)
                .await
                .err()
                .unwrap(),
            ScrapeError::UnsupportedGreeting
        );
    }

    #[tokio::test]
    async fn should_not_scrape_for_greeting_pentecoste() {
        assert_eq!(
            BuongiornissimoCaffe::default()
                .scrape(Greeting::Pentecoste)
                .await
                .err()
                .unwrap(),
            ScrapeError::UnsupportedGreeting
        );
    }

    #[tokio::test]
    async fn should_not_scrape_for_greeting_festa_della_repubblica() {
        assert_eq!(
            BuongiornissimoCaffe::default()
                .scrape(Greeting::FestaDellaRepubblica)
                .await
                .err()
                .unwrap(),
            ScrapeError::UnsupportedGreeting
        );
    }

    #[tokio::test]
    async fn should_not_scrape_for_greeting_corpus_domini() {
        assert_eq!(
            BuongiornissimoCaffe::default()
                .scrape(Greeting::CorpusDomini)
                .await
                .err()
                .unwrap(),
            ScrapeError::UnsupportedGreeting
        );
    }

    #[tokio::test]
    async fn should_not_scrape_for_greeting_sacro_cuore_di_gesu() {
        assert_eq!(
            BuongiornissimoCaffe::default()
                .scrape(Greeting::SacroCuoreDiGesu)
                .await
                .err()
                .unwrap(),
            ScrapeError::UnsupportedGreeting
        );
    }

    #[tokio::test]
    async fn should_not_scrape_for_greeting_ferragosto() {
        assert_eq!(
            BuongiornissimoCaffe::default()
                .scrape(Greeting::Ferragosto)
                .await
                .err()
                .unwrap(),
            ScrapeError::UnsupportedGreeting
        );
    }

    #[tokio::test]
    async fn should_not_scrape_for_greeting_mercoledi_ceneri() {
        assert_eq!(
            BuongiornissimoCaffe::default()
                .scrape(Greeting::MercolediCeneri)
                .await
                .err()
                .unwrap(),
            ScrapeError::UnsupportedGreeting
        );
    }

    #[tokio::test]
    async fn should_not_scrape_for_greeting_ascensione() {
        assert_eq!(
            BuongiornissimoCaffe::default()
                .scrape(Greeting::Ascensione)
                .await
                .err()
                .unwrap(),
            ScrapeError::UnsupportedGreeting
        );
    }

    #[tokio::test]
    async fn should_not_scrape_for_greeting_santissima_trinita() {
        assert_eq!(
            BuongiornissimoCaffe::default()
                .scrape(Greeting::SantissimaTrinita)
                .await
                .err()
                .unwrap(),
            ScrapeError::UnsupportedGreeting
        );
    }

    #[tokio::test]
    async fn should_not_scrape_for_greeting_cuore_immacolato_di_maria() {
        assert_eq!(
            BuongiornissimoCaffe::default()
                .scrape(Greeting::CuoreImmacolatoDiMaria)
                .await
                .err()
                .unwrap(),
            ScrapeError::UnsupportedGreeting
        );
    }
}
