# Nextflow on Zed


This is my attempt to get a Nextflow plugin for Zed working.
A few of us at Sanger use Zed rather than VSCode and it would be nice to see everything lit up as in VSCode!


Treesitter: https://github.com/nextflow-io/tree-sitter-nextflow

LSP: https://github.com/nextflow-io/language-server/releases/tag/v25.04.1


`example.nf` is a workflow file taken from `sanger-tol/EAR` so you can see what the plugin does.


## Installation
```
git clone https://github.com/DLBPointon/zed_nextflow.git
```

Head to Zed --> Extensions --> Instal Dev Plugin --> Select zed_nextflow

On install the plugin will download a `grammars` folder, this is the treesitter repo, and then build a `extension.wasm` which is basically the plugin.

## Main Issues:
- Removed *.scm files whilst testing new treesitter (I reverted to the older one for now).

- src/nextflow.rs
    - Using github instead based on comments from @bentsherman
    - Based on Zig extension
    - Doesn't seem to work though and I can't get any error logging, `zed::open logs` shows successful compilation and install.

- The treesitter being based on groovy syntax is not great for nextflow. You get odd highlights and it can be annoying, perhaps a new treesitter is needed?
    - Nextflow team are working on it, looks like it just needs the queries before it can be used now.
