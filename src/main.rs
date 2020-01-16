use reqwest;
use select::document::Document;
use select::predicate::{Attr, Name, Predicate};

fn main() {

    println!("The scores for the Blue Devils Drum and Bugle Corps from the DCI Finals, from latest to the oldest: ");
    // Use the reqwest library to get the html from the url
    let mut res = reqwest::get("https://bluedevils.org/scores/?programID=1").unwrap();

    // Make sure the request doesn't fail
    assert!(res.status().is_success());

    let html = res.text().unwrap();

    let document = Document::from(html.as_ref());

    // Find the score in the html
    for element in document.find(Attr("class", "cell_score").descendant(Name("span"))) {
        println!("{}", element.text());
    }
}
