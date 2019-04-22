extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::{Class, Name, Predicate};

fn main() {
    hacker_news("https://www.jobscout24.ch/de/jobs/life%20science/?regidl=1-2-3");
}

fn hacker_news(url: &str) {

    let resp = reqwest::get(url).unwrap(); //Unwraps a result, yielding the content of an Ok.
    //Panics if the value is an Err, with a panic message provided by the Err's value.
    
    assert!(resp.status().is_success()); //Ensure that a boolean expression is true at runtime. 
    //This will invoke the panic! macro if the provided expression cannot be evaluated to true at runtime.

    let document = Document::from_read(resp).unwrap();

    // finding all instances of our class of interest. In this case this is an element that belong to a table of content
    for node in document.find(Class("job-list-item")) {
        // grabbing the story rank, that is the number of the story in the list
        // let job_place = node.find(Class("job-attributes")).next().unwrap();
        // finding class, then selecting article title
        /* let job_title = node.find(Class("title").descendant(Name("a")))
            .next()
            .unwrap()
            .text();    */
        // same as above
        let job_url = node.find(Class("job-title").descendant(Name("a"))).next().unwrap();
        // however, we don't grab text
         // instead find the "href" attribute, which gives us the url
        println!("\n | | {}\n", job_url.attr("href").unwrap());
        //println!("{:?}\n", job_place.text());

    }
}
