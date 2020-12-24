use std::io::BufRead;

use rdf::uri;
use serde::{Serialize, Serializer};
use serde::ser::{SerializeMap};

#[derive(Debug, PartialEq, Eq, Clone)]
struct SerializableNode(rdf::node::Node);

#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
struct Claim {
    predicate: SerializableNode,
    object: SerializableNode,
}

impl Serialize for SerializableNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer {
        let mut map = serializer.serialize_map(Some(2))?;
        match &self.0 {
            rdf::node::Node::UriNode { uri } => {
                map.serialize_entry("type", "iri")?;
                map.serialize_entry("value", uri.to_string())?;
            },
            rdf::node::Node::BlankNode { id } => {
                map.serialize_entry("type", "blank")?;
                map.serialize_entry("value", id)?;
            },
            rdf::node::Node::LiteralNode { literal, data_type, language } => {
                map.serialize_entry("type", "literal")?;
                map.serialize_entry("value", literal)?;
            }
        }
        map.end()
    }
}

fn find_claims(source: rdf::graph::Graph, uri: rdf::uri::Uri) -> Vec<Claim> {
    println!("triples: {:?}", source.triples_iter().collect::<Vec<_>>());
    source.get_triples_with_subject(&rdf::node::Node::UriNode { uri }).into_iter().map(|t| Claim {
        predicate: SerializableNode(t.predicate().clone()),
        object: SerializableNode(t.object().clone()),
    }).collect()
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io;

    use handlebars::{Handlebars, to_json};
    use rdf::reader::rdf_parser::RdfParser;
    use serde_json::Map;

    use crate::graph::{Claim, find_claims};

    #[test]
    fn can_render_claims() {
        let file = File::open("graph/hashtagwiki.ttl").expect("Could not open file");
        let reader = io::BufReader::new(file);
        let mut reader = rdf::reader::turtle_parser::TurtleParser::from_reader(reader);
        let graph = reader.decode().expect("Could not parse");
        println!("Graph: {:?}", graph);
        let c = find_claims(graph, rdf::uri::Uri::new("#wiki".to_string()));
        println!("Claims: {:?}", c.clone());
        println!("Claims as JSON: {:?}", to_json(c.clone()).to_string());

        let mut handlebars = Handlebars::new();
        handlebars.register_template_file("node", "./templates/node.hbs").expect("Could not register template file");
        let mut data = Map::new();
        data.insert("claims".to_string(), handlebars::to_json(&c));
        println!("{}", handlebars.render("node", &data).expect("Could not render"));
    }
}
