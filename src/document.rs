use pulldown_cmark::{CowStr, Event, html, Options, Parser, Tag};
use regex::Regex;
use std::ops::Add;

#[derive(Debug, PartialEq)]
pub struct HashTag(String);

fn parse_hash_tag(mut callback: impl FnMut(HashTag)) -> impl FnMut(Event) -> Vec<Event> {
    let mut in_a_link = false;
    move |event| match (event, in_a_link) {
        (Event::Text(s), false) => {
            extract_hashtags(&s).into_iter().map(|t| match t {
                Parsed::ParsedText(s) => Event::Text(CowStr::from(s.to_string())),
                Parsed::ParsedHashTag(h) => {
                    callback(HashTag(h.to_string()));
                    Event::Html(CowStr::from(format!("<span property=\"dc:references\">{}</span>", h.to_string())))
                }
            }).collect()
        }
        (e @ Event::Start(Tag::Link(_, _, _)), false) => {
            in_a_link = true;
            vec![e]
        }
        (e @ Event::End(Tag::Link(_, _, _)), true) => {
            in_a_link = false;
            vec![e]
        }
        (e, _) => vec![e],
    }
}

fn parser(input: &str, callback: impl FnMut(HashTag)) -> impl Iterator<Item=Event> {
    let options = Options::empty();
    Parser::new_ext(input, options).flat_map(parse_hash_tag(callback))
}

pub(crate) fn transform(input: &str) -> (String, Vec<HashTag>) {
    let mut out = String::from(
        "<html prefix=\"dc: http://purl.org/dc/elements/1.1/\">
<link rel=\"stylesheet\" href=\"/static/wiki.css\">
");
    let mut hashtags = Vec::new();
    html::push_html(&mut out, parser(input, |t| hashtags.push(t)));
    out.push_str("\n<script src=\"/static/wiki.js\"></script>");
    (out, hashtags)
}

#[derive(Debug, PartialEq)]
enum Parsed<'a> {
    ParsedText(CowStr<'a>),
    ParsedHashTag(CowStr<'a>),
}

fn extract_hashtags(s: &str) -> Vec<Parsed> {
    let regex = Regex::new(r"#[a-zA-Z][0-9a-zA-Z_]*").unwrap();
    let mut result = Vec::new();
    let mut last_index_in_result = 0;

    for mat in regex.find_iter(s) {
        if last_index_in_result != mat.start() {
            result.push(Parsed::ParsedText(CowStr::from(&s[last_index_in_result..mat.start()])));
        }
        result.push(Parsed::ParsedHashTag(CowStr::from(mat.as_str())));
        last_index_in_result = mat.end();
    }
    if last_index_in_result < s.len() {
        result.push(Parsed::ParsedText(CowStr::from(&s[last_index_in_result..])));
    }
    result
}

#[cfg(test)]
mod tests {
    use pulldown_cmark::CowStr;

    use crate::document::{extract_hashtags, Parsed, transform, HashTag};

    #[test]
    fn can_extract_hashtags() {
        let s = "foo #bar #baz qux";
        let tags = extract_hashtags(s);
        assert_eq!(tags.get(0), Some(&Parsed::ParsedText(CowStr::from("foo "))));
        assert_eq!(tags.get(1), Some(&Parsed::ParsedHashTag(CowStr::from("#bar"))));
        assert_eq!(tags.get(2), Some(&Parsed::ParsedText(CowStr::from(" "))));
        assert_eq!(tags.get(3), Some(&Parsed::ParsedHashTag(CowStr::from("#baz"))));
        assert_eq!(tags.get(4), Some(&Parsed::ParsedText(CowStr::from(" qux"))));
        assert_eq!(tags.len(), 5);
    }

    #[test]
    fn transforms_markdown() {
        let doc = "# #foo\n\nA #link [and](foo) [#link](#bar).";
        let (transformed, hashtags) = transform(doc);
        assert_eq!(transformed, "<h1><span property=\"dc:references\">#foo</span></h1>\n<p>A <span property=\"dc:references\">#link</span> <a href=\"foo\">and</a> <a href=\"#bar\">#link</a>.</p>\n".to_string());
        assert_eq!(hashtags, vec![HashTag("#foo".to_string()), HashTag("#link".to_string())]);
    }
}