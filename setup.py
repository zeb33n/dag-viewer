from setuptools import setup
from setuptools.command.build_py import build_py
import subprocess


class BuildPy(build_py):
    def run(self):
        subprocess.run(["make", "rust"], check=True)
        super().run()


setup(cmdclass={"build_py": BuildPy})
