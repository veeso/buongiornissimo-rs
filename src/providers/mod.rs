//! # Providers
//!
//! This module exposes all the different providers

use super::{Greeting, Scrape, ScrapeError, ScrapeResult, Url};

mod buongiornissimo_caffe;
mod ilmondodigrazia;

pub use buongiornissimo_caffe::BuongiornissimoCaffe;
pub use ilmondodigrazia::IlMondoDiGrazia;
