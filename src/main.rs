extern crate clap;

use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("Scraper")
        .version("1.0.0")
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
    println!("Website: {}", w);
}