#!/usr/bin/env python3
"""qaros-adaptation: create, check, test, package, pin and verify QAROS web app adaptations.

    python -I qaros-adaptation.py --help

Python 3 standard library only. Runs from an installed kit (`<install>/share/qaros/adaptation-kit/
tools/`) and from the `qaros` repository (`adaptation-kit/tools/`).
"""
import sys
from pathlib import Path

# `python -I` leaves the script's folder off sys.path: add it for the qaros_adaptation package.
sys.path.insert(0, str(Path(__file__).resolve().parent))

from qaros_adaptation.cli import main  # noqa: E402

if __name__ == "__main__":
    sys.exit(main())
