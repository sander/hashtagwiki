# #mdoc

[mdoc](https://scalameta.org/mdoc/) provides “typechecked #Markdown documentation for #Scala” (#StronglyTyped). It is useful for documenting software projects with some examples and variables, such as:

````markdown
# MyLibrary Readme

Add this project to your `build.sbt`:

```scala
libraryDependencies += "org.example" % "my-library" % "@VERSION@"
```

To use:

```scala mdoc
myLibrary.foo(42)
```
````

The tool would replace `@VERSION` with the project version, type-check the `myLibrary.foo` call and render the execution result back into the #Markdown file.

It would be even cooler if it allowed for inline references to #Scala types: #TypeReferencesInNaturalLanguage. This could enable a combination between #wiki and #KnowledgeCrunchingInCode. Compare to #Laika’s support for [Linking to API Documentation](https://planet42.github.io/Laika/0.17/03-preparing-content/02-navigation.html#linking-to-api-documentation).
