
#[macro_use] extern crate prettytable;
// #[macro_use] extern crate text_io;
extern crate reqwest;
extern crate select;

use std::io;
use std::io::prelude::*;
use select::document::Document;
use select::predicate::Name;
use prettytable::Table;

fn main() {

    print!("Masukkan URL: ");
    io::stdout().flush().unwrap();

    let stdin = io::stdin();
    let input = &mut String::new();

    input.clear();
    stdin.read_line(input).unwrap();

    println!("Processing...");
    do_scrap(&input);
}

fn do_scrap(url: &str) {
    let resp = reqwest::get(url).unwrap();
    assert!(resp.status().is_success());
    
    let document = Document::from_read(resp).unwrap();
    let mut table = Table::new();

    let mut count = 0;

    for node in document.find(Name("a")) {
        let url = node.attr("href").unwrap();
        let url_trim = url.trim_start_matches('/');
        
        let url_final = format!(" | {} | {}", count, url_trim);
        count += 1;

        table.add_row(row![FdBybl->url_final]);
    }

    table.printstd();
}