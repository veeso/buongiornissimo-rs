use buongiornissimo_rs::{BuongiornissimoCaffe, Scrape};
use chrono::Local;
use rand::Rng;

fn choice<T>(choices: &[T]) -> &T {
    let mut rng = rand::rng();
    &choices[rng.random_range(0..choices.len())]
}

fn get_provider() -> Box<dyn Scrape> {
    match *choice(&[0]) {
        0 => Box::new(BuongiornissimoCaffe::default()),
        _ => panic!("out of range"),
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let motd = buongiornissimo_rs::greeting_of_the_day(Local::now().date_naive(), true);
    let urls = get_provider().scrape(motd).await?;
    let url = choice(&urls);
    open::that(url.to_string())?;
    Ok(())
}
