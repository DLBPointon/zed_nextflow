# Nextflow on Zed


This is my attempt to get a Nextflow plugin for Zed working.
A few of us at Sanger use Zed rather than VSCode and it would be nice to see everything lit up as in VSCode!


Treesitter: https://github.com/matthuska/tree-sitter-nextflow

LSP: https://github.com/nextflow-io/language-server/releases/tag/v25.04.1


`example.nf` is a workflow file taken from `sanger-tol/EAR` so you can see what the plugin does.


## Installation
```
git clone https://github.com/DLBPointon/zed_nextflow.git
```

Head to Zed --> Extensions --> Instal Dev Plugin --> Select zed_nextflow

On install the plugin will download a `grammars` folder, this is the treesitter repo, and then build a `extension.wasm` which is basically the plugin.

## Main Issues:
- src/nextflow.rs


    ```
    const PACKAGE_NAME: &str = "@nextflow/language-server"; // Used to build a search path on https://registry.npmjs.org/@nextflow/language-server
    ```


    A secondary issue to this is that @nextflow on npm is already taken by a 13 year old project which looks pretty abandoned: `https://www.npmjs.com/package/nextflow`


    So because it can't get the LSP it will print an error but the treesitter should still work.


- The treesitter being based on groovy syntax is not great for nextflow. You get odd highlights and it can be annoying, perhaps a new treesitter is needed?
