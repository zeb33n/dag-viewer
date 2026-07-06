from pathlib import Path
import shutil
import subprocess
import yaml

# parse config
CONFIG_FILE = "dag-viewer.yml"

config = yaml.safe_load(Path(CONFIG_FILE).open())

DOCS_DIR = config.get("docs-dir", "docs")
ASSET_DIR = f"{DOCS_DIR}/dag_viewer_assets"
DOT_FILES = config.get("dot-files", [f"{DOCS_DIR}/graph.dot"])
JS_FILE = "dag_viewer.js"
WASM_FILE = "dag_viewer.wasm"

# copy required package files to ASSET_DIR
for file in [JS_FILE, WASM_FILE]:
    dst = Path(ASSET_DIR).joinpath(file)
    out = Path(__file__).parent.joinpath(f"assets/{file}")
    dst.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(str(out), dst)

# use graphvis to process the dotfiles
for f in DOT_FILES:
    dotsrc = subprocess.run(
        ["dot", "-Tdot", "-Gsplines=polyline", "-Grankdir=LR", f],
        check=True,
        capture_output=True,
    ).stdout

    pf = f"processed_{Path(f).name}"
    (Path(ASSET_DIR) / pf).write_bytes(dotsrc)


# write a .gitignore so user doesnt accidentally commit our generated files
(Path(ASSET_DIR) / ".gitignore").write_text("*")


def define_env(env):

    @env.macro
    def dag_viewer(w, h, graph):
        return f"""
<script type="module">
  import {{dag_viewer_init}} from "/dag_viewer_assets/dag_viewer.js"
  dag_viewer_init("/dag_viewer_assets/processed_{graph}");
</script>
<canvas id="dag_viewer" height="{h}" width="{w}"></canvas>
"""
