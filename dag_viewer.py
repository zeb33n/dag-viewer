import subprocess
from pathlib import Path

JS_PATH = Path(__file__).parent / "dag_viewer.js"
WASM_PATH = Path(__file__).parent / "dag_viewer.wasm"


def define_env(env):
    @env.macro
    def dag_viewer(w, h):
        subprocess.run(["cp", str(JS_PATH), "site/assets/"])
        subprocess.run(["cp", str(WASM_PATH), "site/assets/"])

        return f"""
<canvas id="dag_viewer" tabindex="1" height="600px" width="100%"></canvas>

<script type="module">
  console.log("waagwan");
  import {{ dag_viewer_init }} from "/assets/dag_viewer.js";

  dag_viewer_init();
</script>
"""
