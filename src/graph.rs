use std::io::BufRead;

use sophia::graph::Graph;
use sophia::graph::inmem::FastGraph;
use sophia::parser::rio_common::StrictRioSource;
use sophia::parser::TripleParser;
use sophia::term::{Term, TermData};
use sophia::triple::stream::TripleSource;
use sophia::triple::Triple;

#[derive(Serialize, Debug, PartialEq, Eq)]
struct Claim {
    name: String,
    value: String,
}

// fn term_to_string<TD>(term: &Term<TD>) -> String where TD: TermData {
//     "foo".to_string()
// }

fn claims(source: impl Graph, term: Term<&str>) -> Vec<Claim> {
    let mut x: Vec<Claim> = Vec::new();
    for result in source.triples_with_s(&term).map_triples(|t|
        Claim {
            name: "foo".to_string(),
            value: "bar".to_string()
        }
    ) {
        match result {
            Ok(claim) => x.push(claim),
            Err(_) => {}
        }
    }
    x
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io;

    use handlebars::Handlebars;
    use serde_json::Map;
    use sophia::graph::{*, inmem::FastGraph};
    use sophia::ns::Namespace;
    use sophia::parser::turtle;
    use sophia::serializer::*;
    use sophia::serializer::nt::NtSerializer;
    use sophia::term::Term;
    use sophia::term::Term::Iri;
    use sophia::triple::stream::TripleSource;
    use sophia::triple::Triple;

    use crate::graph::{Claim, claims};

    #[test]
    fn can_parse_rdf() {
        let example = r#"
        @base <urn:uuid:69a04cbe-5ba6-42d3-90e9-f54b8b0f2c93> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix dc: <http://purl.org/dc/elements/1.1/> .

<#wiki>
  rdfs:label "wiki" ;
  rdfs:comment "a collaboratively edited publication" .

<#Markdown>
  rdfs:label "Markdown" ;
  dc:references <https://daringfireball.net/projects/markdown/> .
"#;

        let source = sophia::parser::turtle::parse_str(example);
        let iri: Term<&str> = Term::new_iri("urn:uuid:69a04cbe-5ba6-42d3-90e9-f54b8b0f2c93#wiki").expect("Could not create IRI");
        let mut filtered: FastGraph = source.filter_triples(|t| t.s() == &iri).collect_triples().expect("Could not filter");
        let mut stringifier = NtSerializer::new_stringifier();
        let out = stringifier.serialize_graph(&mut filtered).map(|x| x.as_str());
        println!("filtered: {:?}", out);
    }

    #[test]
    fn can_list_claims_v2() {
        let file = File::open("graph/hashtagwiki.ttl").expect("Could not open file");
        let reader = io::BufReader::new(file);
        let source = turtle::parse_bufread(reader);
        let iri: Term<&str> = Term::new_iri("urn:uuid:69a04cbe-5ba6-42d3-90e9-f54b8b0f2c93#wiki").expect("Could not create IRI");
        // let mut filtered: FastGraph = source.filter_triples(|t| t.s() == &iri).collect_triples().expect("Could not filter");
        // let mut stringifier = NtSerializer::new_stringifier();
        // let out = stringifier.serialize_graph(&mut filtered).map(|x| x.as_str());
        // println!("filtered: {:?}", out);
        let expected = vec![
            Claim {
                name: "http://www.w3.org/2000/01/rdf-schema#label".to_string(),
                value: "wiki".to_string(),
            },
            Claim {
                name: "http://www.w3.org/2000/01/rdf-schema#comment".to_string(),
                value: "a collaboratively edited publication".to_string(),
            }
        ];
        let graph: FastGraph = source.collect_triples().expect("Could not collect triples");
        let result = claims(graph, iri);//.expect("Could not find claims");
        assert_eq!(result, expected);
        // println!("Result: {:?}", result);
    }

    #[test]
    fn can_list_claims() {
        let file = File::open("graph/hashtagwiki.ttl").expect("Could not open file");
        let reader = io::BufReader::new(file);
        let source = turtle::parse_bufread(reader);
        let iri: Term<&str> = Term::new_iri("urn:uuid:69a04cbe-5ba6-42d3-90e9-f54b8b0f2c93#wiki").expect("Could not create IRI");
        let mut filtered: FastGraph = source.filter_triples(|t| t.s() == &iri).collect_triples().expect("Could not filter");
        let mut stringifier = NtSerializer::new_stringifier();
        let out = stringifier.serialize_graph(&mut filtered).map(|x| x.as_str());
        println!("filtered: {:?}", out);
    }

    #[test]
    fn can_render() {
        let mut handlebars = Handlebars::new();
        handlebars.register_template_file("node", "./templates/node.hbs").expect("Could not register template file");
        let mut data = Map::new();
        let claims = vec![
            Claim {
                name: "foo".to_string(),
                value: "bar".to_string(),
            },
            Claim {
                name: "baz".to_string(),
                value: "qux".to_string(),
            }
        ];
        data.insert("claims".to_string(), handlebars::to_json(&claims));
        println!("{}", handlebars.render("node", &data).expect("Could not render"));
    }
}
