# Wiki server

A wiki engine that:

- uses Markdown for markup
- uses hashtags for linking concepts, resulting in “natural bi-directional links”

## To do

- [ ] Collect into a graph, listing dc:isReferencedBy for each hashtag
- [ ] Persist the graph into a JSON-LD file per hashtag
- [ ] Have wiki.js create a modal rendering the JSON-LD content
- [ ] Add an edit button running `open` on macOS
- [ ] Render to static output files for serving from object storage

## Credits

- Hashtag popups inspired by De Correspondent [info cards by Momkai](https://www.momkai.com/cases/decorrespondent_platform)