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
This is the recommended approach.
Alternatively you can run the project as a javascript module.
More details can be found [here]().
Please note this feature is experimental.

To use with Zensical add the following lines to your zensical.toml

```toml
[project.markdown_extensions.zensical.extensions.macros]
module_name = "dag_viewer"
```

You also need to create a config file for Dag Viewer.
This should be called `dag-viewer.yml` and live in your projects route directory.
Here you can define the paths to dotfiles you want Dag Viewer to be aware of.

```yml
dot-files:
  - path/to/my/file.dot
  - path/to/other.dot
```

Other config options are available [link](usage#Config).

You can now use Dag Viewer with jinja style templating directly in your zensical markdown like so.

```md
# My Docs

Check out this cool graph!

{{ dag_viewer("900px", "900px", "file.dot") }}

```

The function takes 3 arguments.
The first and second are the width and the height of the dag viewer canvas.
CSS units are supported here.
The third argument is the filename of the dotfile to be rendered.
Do not provide a full path e.g `path/to/my/file.dot` should be shortened to `file.dot`

## Config
The following options are available.

| yaml file | enviromental variable | description |
| - | - | - |
| `dot-files` | `DAG_VIEWER_DOT_FILES` | A list of paths to dotfiles to be rendered |
| `site-dir` | `DAG_VIEWER_SITE_DIR` | The path to zensicals output site directory |
