"""Import the safety lane API without colliding with tools/compat/run.py."""
import importlib.util
from pathlib import Path
import sys

NAME = '_rless_safety_lanes'
if NAME not in sys.modules:
    spec = importlib.util.spec_from_file_location(NAME, Path(__file__).with_name('run.py'))
    implementation = importlib.util.module_from_spec(spec)
    sys.modules[NAME] = implementation
    spec.loader.exec_module(implementation)


def __getattr__(name):
    return getattr(sys.modules[NAME], name)
