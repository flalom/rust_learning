
extern crate scraper;
extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::{Predicate, Attr, Class, Name};

pub fn link_grabber(url: &str){

    let resp = reqwest::get(url).unwrap();
    assert!(resp.status().is_success());

    let docu = Document::from_read(resp);
       
    for job in docu.find(Class("job-list-item external")){
        let job_title = job.find(Class("job-title")).next().unwrap().text();
        let job_link = job.find(Class("job-link-detail")).next().unwrap().text();
        /*let answers = page.find(Class("status").descendant(Name("strong")))
            .next()
            .unwrap()
            .text();
            */
            println!("This is the job title: {}", job_title);
            println!("This is the link where to check: {}", job_link);
    }
        //.unwrap()
        //.find(Name("a"))
        //.filter_map(|n| n.attr("href"))
        //.for_each(|x| println!("{}", x));

}