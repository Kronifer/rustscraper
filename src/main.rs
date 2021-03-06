use clap::{Arg, App, SubCommand};

#[tokio::main]
async fn main() {
    let matches = App::new("Scraper")
        .version("1.0")
        .author("Kronifer")
        .about("Scrapes a website")
        .arg(Arg::with_name("website")
            .short("w")
            .long("website")
            .help("Sets website to scrape")
            .required(true)
            .takes_value(true))
        .get_matches();

    let w = matches.value_of("website").unwrap();
    let body = reqwest::get(w)
        .await
        .unwrap()
        .text()
        .await;
    println!("{:?}", body);
}