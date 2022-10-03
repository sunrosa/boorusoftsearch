use std::collections::HashMap;

use gelbooru_api as gb;

#[tokio::main]
async fn main() {
    let mut scores = HashMap::<String, i32>::new();
    scores = serde_json::from_str(std::fs::read_to_string("score.json").unwrap().as_str()).unwrap();

    let mut results = std::vec::Vec::<(i32, gb::api::Post)>::new();

    let client = gb::Client::public();
    let post_query = gb::posts().random(true).send(&client).await.unwrap();

    for post in post_query.posts {
        results.push((evaluate(&post, &mut scores), post))
    }

    results.sort_by(|a, b| b.0.cmp(&a.0));

    for result in results {
        println!(
            "{}: https://gelbooru.com/index.php?page=post&s=view&id={}",
            result.0,
            result.1.id()
        );
    }
}

fn evaluate(post: &gb::api::Post, scores: &mut HashMap<String, i32>) -> i32 {
    let mut evaluation: i32 = 0;
    for tag in post.tags() {
        evaluation += *scores.entry(tag.to_string()).or_default(); // No clue in hell why it requires a mutable reference for something that I DO NOT mutate. But it does.
    }

    evaluation += *scores.entry(format!("rating:{}", post.rating)).or_default();

    return evaluation;
}
