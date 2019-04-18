
pub fn link_grabber(url: &str) {
    let resp = reqwest::get(url).unwrap();
    println!(resp)
    assert!(resp.status().is_success());

    Document::from_read(resp)
        .unwrap()
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}{}", &url, x));
}