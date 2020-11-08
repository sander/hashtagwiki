# #TypeReferencesInNaturalLanguage

We want the ability to cross over between #wiki and #KnowledgeCrunchingInCode, since this is essential for #DeepModeling with #DomainExpert-s.

## Examples

#IntegratedDevelopmentEnvironment support for #wiki that brings the benefit of #TypeChecking is still limited. Some instances that do seem to work:

### #Scaladoc

Enables referring to types in #docstring-s, which are rendered to #hyperlink-s in #HTML. #IntelliJ includes them in some #refactor-ings such as renaming.

```scala
/**
 * Identifier of a [[AccessControlDomain.SecurityPrincipal]].
 */
case class SecurityPrincipalId(value: UUID)
```

The only known compiler renders #HTML code documentation, not #book-s, #wiki-s or #BlogPost-s.

### #Rustdoc

In [RFC 1946 intra_rustdoc_links](https://github.com/rust-lang/rfcs/blob/master/text/1946-intra-rustdoc-links.md), Rust introduces the “implied shortcut reference link”, which are rendered to #hyperlink-s in #HTML:

```rust
/// Identifier of a [access_control::domain::SecurityPrincipal].
struct SecurityPrincipalId(UUID);
```

At the time of writing, it is not clear yet if any #IntegratedDevelopmentEnvironment supports refactoring in these docs.

The only known compiler renders #HTML code documentation, not #book-s, #wiki-s or #BlogPost-s.

### #SoyWiki

[SoyWiki](http://danielchoi.com/software/soywiki.html) seems to provide the required features with “namespaced wiki words” and “automated global renaming of wiki words”.

## Related

- #DenotationalSemantics
