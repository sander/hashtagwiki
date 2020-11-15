use std::{fs, io};
use std::path::Path;

use crate::document;

const INPUT_DIRECTORY: &str = "wiki";
const OUTPUT_DIRECTORY: &str = "out";

fn prepare_output_directory() -> io::Result<()> {
    fs::remove_dir_all(OUTPUT_DIRECTORY).ok();
    fs::create_dir_all(Path::new(OUTPUT_DIRECTORY).join("wiki"))
}

fn publish_output() -> io::Result<()> {
    fs::read_dir(INPUT_DIRECTORY)?
        .map(|res| -> Result<_, io::Error> {
            let path = res?.path();
            let content = fs::read_to_string(&path)?;
            let (transformed, hashtags) = document::transform(&content);
            let output_path = Path::new(OUTPUT_DIRECTORY).join(&path).with_extension("html");
            println!("Writing to {:?}", &output_path);
            fs::write(&output_path, transformed)?;
            Ok((output_path, hashtags))
        })
        .collect::<Result<Vec<_>, io::Error>>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use crate::publishing::{INPUT_DIRECTORY, OUTPUT_DIRECTORY, prepare_output_directory, publish_output};

    #[test]
    fn publishes_all_wiki_files() {
        let count_output = || fs::read_dir(Path::new(OUTPUT_DIRECTORY).join("wiki")).unwrap().count();
        let count_input = || fs::read_dir(INPUT_DIRECTORY).unwrap().count();

        prepare_output_directory().unwrap();

        assert_eq!(count_output(), 0);

        publish_output().unwrap();

        assert_eq!(count_output(), count_input());
    }
}