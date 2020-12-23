use pulldown_cmark::{html, CowStr, Event, Options, Parser, Tag};
use regex::Regex;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) struct HashTag(pub String);

fn parse_hash_tag(mut callback: impl FnMut(HashTag)) -> impl FnMut(Event) -> Vec<Event> {
    let mut in_a_link = false;
    move |event| match (event, in_a_link) {
        (Event::Text(s), false) => extract_hashtags(&s)
            .into_iter()
            .map(|t| match t {
                Parsed::ParsedText(s) => Event::Text(CowStr::from(s.to_string())),
                Parsed::ParsedHashTag(h) => {
                    callback(HashTag(h.to_string()));
                    Event::Html(CowStr::from(format!(
                        "<span property=\"dc:references\">{}</span>",
                        h.to_string()
                    )))
                }
            })
            .collect(),
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

fn parser(input: &str, callback: impl FnMut(HashTag)) -> impl Iterator<Item = Event> {
    let options = Options::empty();
    Parser::new_ext(input, options).flat_map(parse_hash_tag(callback))
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) struct PageId(pub String);

#[derive(Clone, PartialEq, Hash, Eq)]
pub(crate) struct PageTitle(pub String);

pub(crate) fn transform(input: &str, page_id: PageId) -> (String, Vec<HashTag>) {
    let mut out = String::from(
        "<!doctype html>\
<html prefix=\"dc: http://purl.org/dc/elements/1.1/\">
<meta charset=\"utf-8\">
<link rel=\"stylesheet\" href=\"../static/wiki.css\">
<title>",
    );
    out.push_str(&title(input).0);
    out.push_str("</title>\n<nav class=\"toolbar\"><p><a href=\"https://github.com/sander/hashtagwiki/edit/main/wiki/");
    out.push_str(&page_id.0);
    out.push_str(".md\">Edit</a></p></nav>\n");
    let mut hashtags = Vec::new();
    html::push_html(&mut out, parser(input, |t| hashtags.push(t)));
    out.push_str("\n<script src=\"../static/wiki.js\"></script>");
    out.push_str("\n<script async src=\"https://platform.twitter.com/widgets.js\"></script>");
    (out, hashtags)
}

pub(crate) fn title(s: &str) -> PageTitle {
    let regex = Regex::new(r"^(# )?(?P<title>.*)").unwrap();
    PageTitle(match regex.captures(s) {
        Some(m) => match m.name("title").unwrap().as_str() {
            "" => String::from("Untitled"),
            s => String::from(s),
        },
        None => String::from("Untitled"),
    })
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
            result.push(Parsed::ParsedText(CowStr::from(
                &s[last_index_in_result..mat.start()],
            )));
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

    use crate::document::{extract_hashtags, title, transform, HashTag, PageId, Parsed};

    #[test]
    fn can_extract_hashtags() {
        let s = "foo #bar #baz qux";
        let tokens = extract_hashtags(s);
        assert_eq!(
            tokens.get(0),
            Some(&Parsed::ParsedText(CowStr::from("foo ")))
        );
        assert_eq!(
            tokens.get(1),
            Some(&Parsed::ParsedHashTag(CowStr::from("#bar")))
        );
        assert_eq!(tokens.get(2), Some(&Parsed::ParsedText(CowStr::from(" "))));
        assert_eq!(
            tokens.get(3),
            Some(&Parsed::ParsedHashTag(CowStr::from("#baz")))
        );
        assert_eq!(
            tokens.get(4),
            Some(&Parsed::ParsedText(CowStr::from(" qux")))
        );
        assert_eq!(tokens.len(), 5);
    }

    #[test]
    fn transforms_markdown() {
        let doc = "# #foo\n\nA #link [and](foo) [#link](#bar).";
        let (transformed, hashtags) = transform(doc, PageId(String::from("MyPage")));
        assert_eq!(transformed, "<!doctype html><html prefix=\"dc: http://purl.org/dc/elements/1.1/\">\n<meta charset=\"utf-8\">\n<link rel=\"stylesheet\" href=\"../static/wiki.css\">\n<title>#foo</title>\n<nav class=\"toolbar\"><p><a href=\"https://github.com/sander/hashtagwiki/edit/main/wiki/MyPage.md\">Edit</a></p></nav>\n<h1><span property=\"dc:references\">#foo</span></h1>\n<p>A <span property=\"dc:references\">#link</span> <a href=\"foo\">and</a> <a href=\"#bar\">#link</a>.</p>\n\n<script src=\"../static/wiki.js\"></script>\n<script async src=\"https://platform.twitter.com/widgets.js\"></script>".to_string());
        assert_eq!(
            hashtags,
            vec![HashTag("#foo".to_string()), HashTag("#link".to_string())]
        );
    }

    #[test]
    fn extracts_title() {
        assert_eq!(title("# #foo\n\nContent"), "#foo");
        assert_eq!(title("bar\n\nContent"), "bar");
        assert_eq!(title(""), "Untitled");
    }
}
