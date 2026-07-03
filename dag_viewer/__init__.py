from pathlib import Path
import shutil

ASSET_DIR = "docs/dag_viewer_assets"

# This is a hack we shouldn't really be writing to docs without a way to cleanup.
# It will do for now.
# maybe ok if we tell user to put graph.dot here.
for file in ["dag_viewer.js", "dag_viewer.wasm"]:
    dst = Path(ASSET_DIR).joinpath(file)
    out = Path(__file__).parent.joinpath(f"assets/{file}")
    dst.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(str(out), dst)


def define_env(env):

    @env.macro
    def dag_viewer(w, h):
        return f"""
<script type="module" src="dag_viewer_assets/dag_viewer.js"></script>
<canvas id="dag_viewer" height="{h}" width="{w}"></canvas>
"""
