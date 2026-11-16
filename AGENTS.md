# AGENTS.md

Rules for coding agents working in this repository. `qaros` is public: everything in it ships to
customers or is published at https://docs.quaternar.com.

## Compatibility gate (mandatory)

- Before committing a change to anything sent or stored (a message, a stored file format, a public
  struct or enum that crosses a process or reaches disk): classify it (additive optional field,
  enum value added, rename with dual write, migration, breaking), add its fixture and test, show
  the classification to the user and commit only after their explicit approval. A breaking change
  only in a release the user declares breaking. The rules it checks: Compatibility (hard rule)
  below.

## No internal decision records

- Never mention an internal architecture decision record (an ADR of the private parent repository,
  its `docs/adr/` folder, any `ADR-NNNN` or `ADR <area>/NNNN` reference) anywhere in this repository: docs, website pages, code
  comments, the C header and its source, C# XML docs and comments, tests, examples, changelogs,
  feature notes.
- Readers cannot open those records. State the rule, the reason or the behaviour itself instead.
- This repository's own public decision records (`docs/adr/`, `ADR-0001` and on) are not internal:
  link them freely, here and from the other QAROS repositories.
- The public C header (`qar-streaming-c/include/qar_streaming.h`) and the generated API reference
  come from the parent repository's `qar-streaming/libs/qar-streaming-c/include/qar_streaming_c/v0/`:
  the rule applies to those sources too. Never edit the generated header or the generated API pages;
  fix the source and regenerate.
- `CHANGELOG.md` here is generated from the parent repository's changelogs, except its `Docs`
  entries: the rule applies to the entries there that it copies.
- Versioned docs (`docs/website/*versioned_docs/`) are frozen snapshots: leave them as they are.

## Compatibility (hard rule)

Any QAROS header works with any QAROS runtime, older or newer, and from 0.10.0 every QAROS build
talks to every other and reads every file another wrote. This holds until a release is declared
breaking in the changelog (`Breaking`). Public page: `docs/website/docs/developer-guide/compatibility.mdx`.

- **C API is append-only.** Never rename a function, change a signature or grow a struct under its
  structure type. A function appended after a release is `OPTIONAL_SYMBOL` and returns
  `QAR_STATUS_NOT_IMPLEMENTED` on an older runtime; callers detect features by calling and handling
  that status.
- **Never check versions for equality.** No header-hash pinning, no comparison of the full version
  string or its `rc`/`dev` suffix, no bundled runtime: load the installed one. `qar_runtime_info`
  is for logs and messages only.
- **Every network or disk payload is versioned per message** (the schema envelope); there are no
  global or handshake protocol versions.
- **Fields are only ever added.** Never rename, remove, retype or change the unit, frame or meaning
  of a sent or stored field; a changed meaning is a new field.
- **Every schema change ships with a fixture and a test** that reads the previous release's data.
