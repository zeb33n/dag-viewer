from pathlib import Path
import shutil
import subprocess

ASSET_DIR = "docs/dag_viewer_assets"
JS_FILE = "dag_viewer.js"
WASM_FILE = "dag_viewer.wasm"
DOT_FILE = "graph.dot"
PROCESSED_DOT_FILE = f"processed_{DOT_FILE}"

# use graphvis to process the dotfile
dotsrc = subprocess.run(
    ["dot", "-Tdot", "-Gsplines=polyline", "-Grankdir=LR", f"{ASSET_DIR}/{DOT_FILE}"],
    check=True,
    capture_output=True,
).stdout

(Path(ASSET_DIR) / PROCESSED_DOT_FILE).write_bytes(dotsrc)

# copy required package files to ASSET_DIR
for file in [JS_FILE, WASM_FILE]:
    dst = Path(ASSET_DIR).joinpath(file)
    out = Path(__file__).parent.joinpath(f"assets/{file}")
    dst.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(str(out), dst)

# write a .gitignore so user doesnt accidentally commit our generated files
(Path(ASSET_DIR) / ".gitignore").write_text(f"""
{JS_FILE}
{WASM_FILE}
{PROCESSED_DOT_FILE}
""")


def define_env(env):

    @env.macro
    def dag_viewer(w, h):
        return f"""
<script type="module" src="dag_viewer_assets/dag_viewer.js"></script>
<canvas id="dag_viewer" height="{h}" width="{w}"></canvas>
"""
