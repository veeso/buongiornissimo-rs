use buongiornissimo_rs::{IlMondoDiGrazia, Scrape};
use chrono::Local;
use rand::Rng;

fn choice<T>(choices: &[T]) -> &T {
    let mut rng = rand::thread_rng();
    &choices[rng.gen_range(0..choices.len())]
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let motd = buongiornissimo_rs::greeting_of_the_day(Local::today().naive_local(), true);
    let urls = IlMondoDiGrazia::default().scrape(motd).await?;
    let url = choice(&urls);
    open::that(url.to_string())?;
    Ok(())
}
