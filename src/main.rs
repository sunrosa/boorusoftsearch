use std::collections::HashMap;

use gelbooru_api as gb;

#[tokio::main]
async fn main() {
    let mut scores = HashMap::<String, i32>::new();
    scores = serde_json::from_str(std::fs::read_to_string("score.json").unwrap().as_str()).unwrap();

    let client = gb::Client::public();
    let posts = gb::posts()
    .limit(1)
    .tags(&["sort:random"])
    .send(&client)
    .await
    .unwrap();

    println!("{:?}", evaluate(&posts.posts[0].tags(), &mut scores));
    println!("{:?}", posts.posts[0].tags());
}

fn evaluate(tags: &Vec<&str>, scores: &mut HashMap<String, i32>) -> i32 {
    let mut evaluation: i32 = 0;
    for tag in tags {
        evaluation += *scores.entry(tag.to_string()).or_default(); // No clue in hell why it requires a mutable reference for something that I DO NOT mutate. But it does.
    }
    return evaluation;
}