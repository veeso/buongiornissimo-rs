//! # Providers
//!
//! This module exposes all the different providers

use super::{Greeting, Scrape, ScrapeError, ScrapeResult, Url};

mod buongiornissimo_caffe;
mod buongiorno_immagini;
mod ticondivido;

pub use self::buongiornissimo_caffe::BuongiornissimoCaffe;
pub use self::buongiorno_immagini::BuongiornoImmagini;
pub use self::ticondivido::TiCondivido;
