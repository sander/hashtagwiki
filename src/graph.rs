#[cfg(test)]
mod tests {
    use sophia::graph::{*, inmem::FastGraph};
    use sophia::ns::Namespace;
    use sophia::parser::turtle;
    use sophia::serializer::*;
    use sophia::serializer::nt::NtSerializer;
    use sophia::term::Term;
    use sophia::term::Term::Iri;
    use sophia::triple::stream::TripleSource;
    use sophia::triple::Triple;

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
}
