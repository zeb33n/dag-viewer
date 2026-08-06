# Attributes

Dag Viewer supports a number of specialised node attributes that can modify the behaviour of the macro.
For a brief overview please see the following table.

## `dv_label`

`dv_label` allow us to modify the dislay label of the node it is attributed to.
We can use this to create two nodes on out graph with the same name.

```
digraph {
  a [dv_label="Gemini"]
  b [dv_label="Gemini"]
  a -> b
}
```

{{ dag_viewer("100%", "300px", "docs/gemini.dot") }}

## `dv_link`

We can use `dv_link` to embed links which are followed when we click the node on the graph.
This can be used alongside some inline html to powerful effect.

### `link.dot`

```
digraph {
  node1 [dv_link="link.html#node1"]
  node2 [dv_link="link.html#node2"]
  node3 [dv_link="link.html#node3"]
  node1 -> node2
  node1 -> node3
}
```

### `link.md`

```
<style>
  .md-grid {
    max-width: none; 
  }

  .graph-info {
    width: 30%;
    height: 400px;
    overflow-y: auto;
    min-height: 0;
  }

  .graph-container {
    display:flex;
    align-items:flex-start;
    gap:16px;   
  }

</style>

<div class="graph-container" markdown>
  # Insert dag viewer macro here

  <div class="graph-info" markdown>

    ## Node1

    here is some info about the 1st node

    ## Node2

    here is some info about the 2nd node

    ## Node3

    here is some info about the 3rd node

  </div>
</div>
```
<style>
  .md-grid {
    max-width: none; 
  }

  .graph-info {
    width: 30%;
    height: 400px;
    overflow-y: auto;
    min-height: 0;
  }

  .graph-container {
    display:flex;
    align-items:flex-start;
    gap:16px;   
  }
</style>
<div class="graph-container" markdown>

  {{ dag_viewer("60%", "400px", "docs/link.dot") }}

  <div class="graph-info" markdown>

#### Node1

Here is some info about the 1st node:

- I bet you didnt know it was the 1st node!
- This nodes children are
  - Node2
  - Node3

#### Node2

Here is some info about the 2nd node:

- The Buzz Aldrin of all nodes
- This nodes parent is Node1

#### Node3

Here is some info about the 3rd node:

- Nodes must be good since good things come in threes
- This nodes parent is Node1


  </div>
</div>
