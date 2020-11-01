use std::fs;

use warp::{Filter, http::Response};
use std::path::Path;

mod document;

#[tokio::main]
async fn main() {
    let wiki = warp::path!("wiki" / String)
        .map(|name| {
            let path = Path::new("wiki/name").with_file_name(name).with_extension("md");
            println!("Path: {:?}", path);
            match fs::read_to_string(path) {
                Ok(doc) => {
                    let (transformed, hashtags) = document::transform(&doc);
                    Response::builder().header("Content-Type", "text/html").body(transformed)
                }
                Err(_) => Response::builder().status(warp::http::StatusCode::NOT_FOUND).body("Not found".to_string())
            }
        });

    warp::serve(wiki)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
