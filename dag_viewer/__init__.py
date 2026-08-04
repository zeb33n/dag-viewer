from pathlib import Path
import shutil
import subprocess
from typing import Any
import yaml
import os
import atexit

CONFIG_FILE = "dag-viewer.yml"

if Path(CONFIG_FILE).is_file():
    config = yaml.safe_load(Path(CONFIG_FILE).open())
else:
    config = {}


def get_config_value(key: str) -> Any | None:
    return os.environ.get(
        f"DAG_VIEWER_{key.upper().replace('-', '_')}", config.get(key)
    )


SITE_DIR = get_config_value("site-dir") or "site"
DOT_ARGS = get_config_value("dot-args") or ["-Grankdir=LR"]

ASSET_DIR = f"{SITE_DIR}/dag_viewer_assets"
JS_FILE = "dag_viewer.js"
WASM_FILE = "dag_viewer.wasm"
GRAPHS = []

def copy_assets():
    # copy required package files to ASSET_DIR
    for file in [JS_FILE, WASM_FILE]:
        dst = Path(ASSET_DIR).joinpath(file)
        out = Path(__file__).parent.joinpath(f"assets/{file}")
        dst.parent.mkdir(parents=True, exist_ok=True)
        print(f"DAG VIEWER: copying {out} to {dst}", flush=True)
        shutil.copy2(out, dst)

    # use graphvis to process the dotfiles
    for f in GRAPHS:
        if not (Path(f).exists() and Path(f).is_file()):
            print(f"DAG VIEWER: Warning: cant find file {f}")
            continue

        dotsrc = subprocess.run(
            ["dot", "-Tdot", "-Gsplines=polyline", *DOT_ARGS, f],
            check=True,
            capture_output=True,
        ).stdout

        pf = f"processed_{Path(f).name}"
        processed_path = Path(ASSET_DIR) / pf
        print(f"DAG VIEWER: Processed {f} writing output to {processed_path}")
        processed_path.write_bytes(dotsrc)


def define_env(env):
    # Counter
    viewer_count = [0]

    @env.macro
    def dag_viewer(w, h, graph):
        GRAPHS.append(graph)
        graph = Path(graph).name
        viewer_count[0] += 1
        return f"""
<script type="module">
  const base = new URL(".", window.location.href);
  const module = await import(`${{base}}/dag_viewer_assets/dag_viewer.js`);
  const {{ dag_viewer_init}} = module;
  dag_viewer_init(`${{base}}/dag_viewer_assets/processed_{graph}`, "dag_viewer_{viewer_count[0]}");
</script>
<canvas id="dag_viewer_{viewer_count[0]}" style="height: {h}; width: {w};"></canvas>
"""


atexit.register(copy_assets)
