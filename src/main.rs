use std::collections::HashMap;

use gelbooru_api as gb;

#[derive(serde::Deserialize)]
struct UserTags {
    search: Vec<String>,
    scores: HashMap<String, i64>
}

#[tokio::main]
async fn main() {
    let mut user_tags: UserTags = serde_json::from_str(std::fs::read_to_string("tags.json").unwrap().as_str()).unwrap();

    let mut results = std::vec::Vec::<(i64, gb::api::Post)>::new();

    let client = gb::Client::public();
    let post_query = gb::posts().random(true).tags(&user_tags.search).send(&client).await.unwrap();

    for post in post_query.posts {
        results.push((evaluate(&post, &mut user_tags.scores), post))
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

fn evaluate(post: &gb::api::Post, scores: &mut HashMap<String, i64>) -> i64 {
    let mut evaluation: i64 = 0;
    for tag in post.tags() {
        evaluation += *scores.entry(tag.to_string()).or_default(); // No clue in hell why it requires a mutable reference for something that I DO NOT mutate. But it does.
    }

    evaluation += *scores.entry(format!("rating:{}", post.rating)).or_default();

    return evaluation;
}
