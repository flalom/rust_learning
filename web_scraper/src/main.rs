mod main_2;

extern crate scraper;
extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::Name;

fn main() {
    main_2::link_grabber("https://www.swisstph.ch/");
}

