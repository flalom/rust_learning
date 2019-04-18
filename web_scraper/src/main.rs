mod main_2;

extern crate scraper;
extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::Name;

fn main() {
<<<<<<< HEAD
    main_2::link_grabber("https://www.swisstph.ch/");
}

=======
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
>>>>>>> ed96d656fc191c3351651bebef972dc2fff8197d
