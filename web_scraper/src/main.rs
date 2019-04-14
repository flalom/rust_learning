extern crate scraper;
extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::Name;

fn main() {
    link_grabber("https://www.swisstph.ch/");
}

fn link_grabber(url: &str) {
    let resp = reqwest::get(url).unwrap();
    //assert!(resp.status().is_success());

    Document::from_read(resp)
        .unwrap()
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}{}", &url, x));
}
