# Nextflow on Zed

This is my attempt to get a Nextflow plugin for Zed working.
A few of us at Sanger use Zed rather than VSCode and it would be nice to see everything lit up as in VSCode!

Treesitter: https://github.com/matthuska/tree-sitter-nextflow
LSP: https://github.com/nextflow-io/language-server/releases/tag/v25.04.1

## Main Issues:
- src/nextflow.rs
    ```
    const PACKAGE_NAME: &str = "@nextflow/language-server"; // Used to build a search path on https://registry.npmjs.org/@nextflow/language-server
    ```
    A secondary issue to this is that @nextflow on npm is already taken by a 13 year old project which looks pretty abandoned: `https://www.npmjs.com/package/nextflow`

- the treesitter being based on groovy syntax is not great for nextflow. You get odd highlights and it can be annoying, perhaps a new treesitter is needed?
