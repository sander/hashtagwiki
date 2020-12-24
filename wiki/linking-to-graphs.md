# Linking to #KnowledgeGraph-s

It would be useful to involve external and internal #KnowledgeGraph-s in the #InfoCard design for #ConceptLinking of #HashtagWiki. This enables weak linking to strongly consistent models, enhancing the quality of the #wiki.

## Use case

Given a #wiki with `graph/concepts.ttl` containing:

```turtle
@base <urn:uuid:07751be0-0730-4e58-a6bf-5a41a57309b1> .
@prefix togaf: <http://www.semanticweb.org/ontologies/2020/4/OntologyTOGAFContentMetamodel.owl#> .

<#HashtagWiki>
  a togaf:InformationSystemService ;
  rdfs:label "HashtagWiki" .
```

- `01` **User** clicks `#HashtagWiki` in an HTML rendering.
- `02` **User** sees an #InfoCard listing
  1. other pages referring to this concept,
  2. a rendering of
     - the closest #concept in local #KnowledgeGraph-s (which is in this case `urn:uuid:07751be0-0730-4e58-a6bf-5a41a57309b1#HashtagWiki`) and
     - its properties (in this case, its human-readable `rdfs:label` and it being a subclass of `togaf:InformationSystemService`).
- `03` **User** uses the #InfoCard to navigate further, e.g. by selecting the `togaf:InformationSystemService` entry to learn more about it and discover related #wiki pages.
