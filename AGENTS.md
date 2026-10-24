# AGENTS.md

Rules for coding agents working in this repository. `qaros` is public: everything in it ships to
customers or is published at https://docs.quaternar.com.

## No internal decision records

- Never mention an internal architecture decision record (an ADR of the private parent repository,
  its `docs/adr/` folder, any `ADR-NNNN` or `ADR <area>/NNNN` reference) anywhere in this repository: docs, website pages, code
  comments, the C header and its source, C# XML docs and comments, tests, examples, changelogs,
  feature notes.
- Readers cannot open those records. State the rule, the reason or the behaviour itself instead.
- The public C header (`qar-streaming-c/include/qar_streaming.h`) and the generated API reference
  come from the parent repository's `qar-streaming/libs/qar-streaming-c/include/qar_streaming_c/v0/`:
  the rule applies to those sources too. Never edit the generated header or the generated API pages;
  fix the source and regenerate.
- `CHANGELOG.md` here is generated from the parent repository's changelogs, except its `Docs`
  entries: the rule applies to the entries there that it copies.
- Versioned docs (`docs/website/*versioned_docs/`) are frozen snapshots: leave them as they are.
