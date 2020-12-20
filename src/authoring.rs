use std::{fs, io};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::Path;

use warp::{Filter, http::Response};

use crate::document;
use crate::document::HashTag;

fn pages_containing(hashtag: &HashTag) -> Result<Vec<String>, io::Error> {
    let dir = fs::read_dir(Path::new("wiki"))?;
    dir.map(|entry| {
        let path = entry?.path();
        let id = document::PageId(String::from(path.file_stem().unwrap().to_string_lossy()));
        if document::transform(&fs::read_to_string(&path)?, id).1.contains(&hashtag) {
            Ok(path.file_stem().map(|s| s.to_string_lossy().to_string()))
        } else {
            Ok(None)
        }
    }).filter_map(|res| match res {
        Ok(Some(s)) => Some(Ok(s)),
        Ok(None) => None,
        Err(e) => Some(Err(e)),
    })
        .collect::<Result<Vec<_>, io::Error>>()
}

#[derive(Debug)]
struct IOError;

impl warp::reject::Reject for IOError {}

async fn get_hashtag_info(name: String) -> Result<impl warp::Reply, warp::Rejection> {
    match Path::new(&name).file_stem() {
        Some(s) => {
            let tag = HashTag(format!("#{}", s.to_string_lossy()));
            match pages_containing(&tag) {
                Ok(pages) => {
                    let mut result = HashMap::new();
                    result.insert("wiki", pages.into_iter().map(|s| {
                        let mut entry = HashMap::new();
                        entry.insert("id", s);
                        entry
                    }).collect::<Vec<_>>());
                    Ok(warp::reply::json(&result))
                }
                Err(_e) => Err(warp::reject::custom(IOError))
            }
        }
        None => Err(warp::reject::not_found())
    }
}

pub async fn serve(address: impl Into<SocketAddr>) {
    let wiki = warp::path!("wiki" / String)
        .map(|name| {
            let id = document::PageId(String::from(name));
            let path = Path::new("wiki/name").with_file_name(id.0.clone()).with_extension("md");
            match fs::read_to_string(path) {
                Ok(doc) => {
                    let (transformed, _hashtags) = document::transform(&doc, id);
                    Response::builder().header("Content-Type", "text/html").body(transformed)
                }
                Err(_) => Response::builder().status(warp::http::StatusCode::NOT_FOUND).body("Not found".to_string())
            }
        });
    let hashtag = warp::path!("hashtag" / String)
        .and_then(get_hashtag_info);
    let static_dir = warp::path("static").and(warp::fs::dir("./static/"));

    let routes = wiki.or(hashtag).or(static_dir);

    warp::serve(routes).run(address).await;
}