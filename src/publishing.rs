use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::{fs, io};

use crate::document;

const INPUT_DIRECTORY: &str = "wiki";
const OUTPUT_DIRECTORY: &str = "out";
const STATIC_DIRECTORY: &str = "static";

fn prepare_output_directory() -> io::Result<()> {
    fs::remove_dir_all(OUTPUT_DIRECTORY).ok();
    fs::create_dir_all(Path::new(OUTPUT_DIRECTORY).join("wiki"))?;
    fs::create_dir_all(Path::new(OUTPUT_DIRECTORY).join("hashtag"))?;
    fs::create_dir_all(Path::new(OUTPUT_DIRECTORY).join("static"))?;
    let out = Path::new(OUTPUT_DIRECTORY);
    for entry in fs::read_dir(STATIC_DIRECTORY)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            let target = out.join(&path);
            fs::copy(&path, &target)?;
        }
    }
    Ok(())
}

fn publish_output() -> io::Result<()> {
    let mut map = HashMap::new();
    fs::read_dir(INPUT_DIRECTORY)?
        .map(|res| -> Result<_, io::Error> {
            let path = res?.path();
            let content = fs::read_to_string(&path)?;
            let id = document::PageId(String::from(path.file_stem().unwrap().to_string_lossy()));
            let (transformed, hashtags) = document::transform(&content, id);
            let title = document::title(&content);
            let output_path = Path::new(OUTPUT_DIRECTORY)
                .join(&path)
                .with_extension("html");
            fs::write(&output_path, transformed)?;
            for tag in hashtags.into_iter() {
                let entry = map.entry(tag.clone()).or_insert(HashSet::new());
                entry.insert((
                    document::PageId(path.file_stem().unwrap().to_string_lossy().to_string()),
                    title.clone(),
                ));
            }
            Ok(())
        })
        .collect::<Result<Vec<_>, io::Error>>()?;
    map.into_iter()
        .map(|(tag, paths)| {
            let json = serde_json::json!({
                "wiki": paths.into_iter().map(|s| {
                    let mut entry = HashMap::new();
                    entry.insert("id", s.0.0);
                    entry.insert("title", s.1.0);
                    entry
                }).collect::<Vec<_>>()
            });
            let output_path = Path::new(OUTPUT_DIRECTORY)
                .join("hashtag")
                .join(&tag.0[1..])
                .with_extension("json");
            fs::write(&output_path, json.to_string())?;
            Ok(())
        })
        .collect::<Result<Vec<_>, io::Error>>()?;
    Ok(())
}

pub(crate) fn run() {
    prepare_output_directory().expect("Error preparing output directory");
    publish_output().expect("Error publishing output");
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use crate::publishing::{
        prepare_output_directory, publish_output, INPUT_DIRECTORY, OUTPUT_DIRECTORY,
    };

    #[test]
    fn publishes_all_wiki_files() {
        let count_output = || {
            fs::read_dir(Path::new(OUTPUT_DIRECTORY).join("wiki"))
                .unwrap()
                .count()
        };
        let count_input = || fs::read_dir(INPUT_DIRECTORY).unwrap().count();

        prepare_output_directory().unwrap();

        assert_eq!(count_output(), 0);

        publish_output().unwrap();

        assert_eq!(count_output(), count_input());
    }
}
