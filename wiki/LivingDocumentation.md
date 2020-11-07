# #LivingDocumentation

The #book [Living Documentation: Continuous Knowledge Sharing by Design](https://www.goodreads.com/book/show/34927405-living-documentation) introduces #DomainDrivenDesign-inspired tools and methods to build high-quality documentation throughout the #SoftwareDevelopmentLifecycle.

It features examples of #KnowledgeExtraction, #KnowledgeAugmentation, #KnowledgeCuration, automated creation of evolving documentation and diagrams, using #IntegratedDevelopmentEnvironment-s for #refactor-ing knowledge, and improving #SoftwareDesign.

The approach deals both with new and legacy environments.

## Example: annotations

The book suggests an interesting use of Java / Scala annotations to provide metadata to classes. This could benefit #KnowledgeCrunchingInCode, for example by creating a traversable #ontology of concepts or highlighting key concepts in a #GuidedTour. The advantage over Scala marker traits is that these annotations cannot change behavior of the classes themselves. Example:

```scala
@DomainObject
@GuidedTour("This key concept is a generalization of human and system users.")
case class SecurityPrincipal(id: UUID, domain: SecurityDomain)
```
