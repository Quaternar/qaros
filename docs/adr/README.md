# Decision records

Decisions that apply across the public QAROS repositories. Each record states its context, the
decision and its consequences; a later record supersedes an earlier one rather than editing it.

| Record | Decision |
|---|---|
| [ADR-0001](0001-one-versioning-scheme.md) | One versioning scheme for every QAROS repository: SemVer, derived build versions that sort in build order, `release/X.Y` branches for patches of the previous minor |
| [ADR-0002](0002-app-release-on-merge.md) | QAROS app repositories release on every merge that changes behaviour: the bump comes from the pull request's label or Conventional Commit title, `VERSION` holds the last release, CI commits, tags and publishes |
| [ADR-0003](0003-an-untagged-build-is-x-y-z-commits-plus-sha.md) | A build that is neither a release nor a candidate is `X.Y.Z-<commits>+<sha>` (was `X.Y.Z-dev.M+<sha>`): no `dev` in a version users see, and a numeric pre-release still sorts below every `-rc.N` and the release |
