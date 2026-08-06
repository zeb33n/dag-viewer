---
render_macros: false
---
# Usage Instructions

## Install

We are on pip

```bash
pip install dag-viewer
```

Alternatively build and install from source

```bash
git clone https://github.com/zeb33n/dag-viewer.git
make install
```

Please note the sytem dependency [Graphvis](graphvis.org) is required.

## Usage

The project is designed to be run as a Zenscial macro.

To use with Zensical add the following lines to your zensical.toml

```toml
[project.markdown_extensions.zensical.extensions.macros]
module_name = "dag_viewer"
```

You can now use Dag Viewer with jinja style templating directly in your zensical markdown like so.

```md
# My Docs

Check out this cool graph!

{{ dag_viewer("900px", "900px", "file.dot") }}

```

The function takes 3 arguments.
The first and second are the width and the height of the dag viewer canvas.
CSS units are supported here.
The third argument is the filepath to the dotfile to be rendered.
This should be realtive to the directory you run `zenscial build` in.

You can then build your site as normal.

```
zensical build
```

It is important to note that the macro will not work with zensical serve.
This is because zensical's inbuilt http server disable wasm.
we recommend using a third party server e.g python's builtin.

```
python3 -m http.server -d site
```

## Config

You can configure Dag Viewer using enviromental variables or a config file.
This should be called `dag-viewer.yml` and live in your projects route directory.

The following options are available.

| yaml file | enviromental variable | description | type |
| - | - | - | - |
| `site-dir` | `DAG_VIEWER_SITE_DIR` | The path to zensicals output site directory | string |
| `dot-args` | `DAG_VIEWER_DOT_ARGS` | Additional formatting argument to pass to graphviz | list |

## Dot Attributes

Dag Viewer supports a number of specialised node attributes that can modify the behaviour of the macro.
For a brief overview please see the following table.

| attribute | effect | 
| - | - |
| [`dv_label`](attributes.md#dv_label) | Modifies the display label of the attributed node | 
| [`dv_link`](attributes.md#dv_link) | A link to follow when the atrributed node is clicked |

for more info please see [attributes](attributes.md)

