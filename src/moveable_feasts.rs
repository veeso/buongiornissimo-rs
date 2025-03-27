//! # Moveable feasts
//!
//! This module provides optional functions to get the date for the different Italian moveable feasts

use bdays::easter::easter_naive_date;
use chrono::NaiveDate;

/// Return easter date
///
/// panics if year is < 1582
pub fn easter_date(year: i32) -> NaiveDate {
    easter_naive_date(year).unwrap()
}

/// Return "domenica delle palme" date
///
/// panics if year is < 1582
pub fn domenica_delle_palme_date(year: i32) -> NaiveDate {
    let easter = easter_date(year);
    easter - chrono::Duration::days(7)
}

/// Return pasquetta date
///
/// panics if year is < 1582
pub fn pasquetta_date(year: i32) -> NaiveDate {
    let easter = easter_date(year);
    easter + chrono::Duration::days(1)
}

/// Return the "giovedi grasso" date for the provided year
///
/// panics if year is < 1582
pub fn giovedi_grasso_date(year: i32) -> NaiveDate {
    let easter = easter_date(year);
    easter - chrono::Duration::days(52)
}

/// Return the "martedi grasso" date for the provided year
///
/// panics if year is < 1582
pub fn martedi_grasso_date(year: i32) -> NaiveDate {
    let easter = easter_date(year);
    easter - chrono::Duration::days(47)
}

/// Return the "mercoledi delle ceneri" date for the provided year
///
/// panics if year is < 1582
pub fn mercoled_ceneri_date(year: i32) -> NaiveDate {
    let easter = easter_date(year);
    easter - chrono::Duration::days(46)
}

/// Return ascensione date
///
/// panics if year is < 1582
pub fn ascensione_date(year: i32) -> NaiveDate {
    let easter = easter_date(year);
    easter + chrono::Duration::days(42)
}

/// Return the pentecoste date for the provided year
///
/// panics if year is < 1582
pub fn pentecoste_date(year: i32) -> NaiveDate {
    let easter = easter_date(year);
    easter + chrono::Duration::days(49)
}

/// Return the "santissima trinità" date for the provided year
///
/// panics if year is < 1582
pub fn santissima_trinita_date(year: i32) -> NaiveDate {
    let easter = easter_date(year);
    easter + chrono::Duration::days(56)
}

/// Return the corpus_domini date for the provided year
///
/// panics if year is < 1582
pub fn corpus_domini_date(year: i32) -> NaiveDate {
    let easter = easter_date(year);
    easter + chrono::Duration::days(63)
}

/// Return the sacro cuore di gesu date for the provided year
///
/// panics if year is < 1582
pub fn sacro_cuore_di_gesu_date(year: i32) -> NaiveDate {
    let corpus_domini = corpus_domini_date(year);
    corpus_domini + chrono::Duration::days(5)
}

/// Returns the "cuore immacolato di Maria" date for the provided year
///
/// panics if year is < 1582
pub fn cuore_immacolato_di_maria_date(year: i32) -> NaiveDate {
    let corpus_domini = corpus_domini_date(year);
    corpus_domini + chrono::Duration::days(6)
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn should_calc_easter_date() {
        assert_eq!(
            easter_date(2023),
            NaiveDate::from_ymd_opt(2023, 4, 9).unwrap()
        );
    }

    #[test]
    fn should_calc_palme_date() {
        assert_eq!(
            domenica_delle_palme_date(2023),
            NaiveDate::from_ymd_opt(2023, 4, 2).unwrap()
        );
    }

    #[test]
    fn should_calc_pasquetta_date() {
        assert_eq!(
            pasquetta_date(2023),
            NaiveDate::from_ymd_opt(2023, 4, 10).unwrap()
        );
    }

    #[test]
    fn should_calc_giovedi_grasso_date() {
        assert_eq!(
            giovedi_grasso_date(2023),
            NaiveDate::from_ymd_opt(2023, 2, 16).unwrap()
        );
    }

    #[test]
    fn should_calc_martedi_grasso_date() {
        assert_eq!(
            martedi_grasso_date(2023),
            NaiveDate::from_ymd_opt(2023, 2, 21).unwrap()
        );
    }

    #[test]
    fn should_calc_mercoledi_ceneri_date() {
        assert_eq!(
            mercoled_ceneri_date(2023),
            NaiveDate::from_ymd_opt(2023, 2, 22).unwrap()
        );
    }

    #[test]
    fn should_calc_ascensione_date() {
        assert_eq!(
            ascensione_date(2023),
            NaiveDate::from_ymd_opt(2023, 5, 21).unwrap()
        );
    }

    #[test]
    fn should_calc_pentecoste_date() {
        assert_eq!(
            pentecoste_date(2023),
            NaiveDate::from_ymd_opt(2023, 5, 28).unwrap()
        );
    }

    #[test]
    fn should_calc_santissima_trinita_date() {
        assert_eq!(
            santissima_trinita_date(2023),
            NaiveDate::from_ymd_opt(2023, 6, 4).unwrap()
        );
    }

    #[test]
    fn should_calc_corpus_domini_date() {
        assert_eq!(
            corpus_domini_date(2023),
            NaiveDate::from_ymd_opt(2023, 6, 11).unwrap()
        );
    }

    #[test]
    fn should_calc_sacro_cuore_di_gesu_date() {
        assert_eq!(
            sacro_cuore_di_gesu_date(2023),
            NaiveDate::from_ymd_opt(2023, 6, 16).unwrap()
        );
    }

    #[test]
    fn should_calc_cuore_immacolato_di_maria_date() {
        assert_eq!(
            cuore_immacolato_di_maria_date(2023),
            NaiveDate::from_ymd_opt(2023, 6, 17).unwrap()
        );
    }
}
