"""The QAROS adaptation kit's command line tool (`qaros-adaptation`), standard library only.

Modules:

- `kit`: the kit's libraries, their headers and `lib/index.json`;
- `profile`: reading, validating and rewriting an adaptation's `profile.json`;
- `lints`: the known-pitfall checks of `check`;
- `packaging`: `package` (and the functions a repository's own build script calls);
- `qarapp`: the `.qarapp` format: manifest, signature, ZIP, verification;
- `verify`: `verify` against a running QAROS web app source;
- `cli`: the command line.
"""

TOOL_NAME = "qaros-adaptation"
