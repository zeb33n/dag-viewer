from setuptools import setup
from setuptools.command.build_py import build_py
from pathlib import Path
import subprocess
import shutil


def build_wasm():
    subprocess.run(
        [
            "cargo",
            "build",
            "--release",
            "--target=wasm32-unknown-unknown",
        ],
        check=True,
    )

    out = Path("target/wasm32-unknown-unknown/release/dag_viewer.wasm")

    dst = Path("dag_viewer/assets/dag_viewer.wasm")
    dst.parent.mkdir(parents=True, exist_ok=True)

    shutil.copy2(out, dst)


class BuildPy(build_py):
    def run(self):
        build_wasm()
        super().run()


setup(cmdclass={"build_py": BuildPy})
