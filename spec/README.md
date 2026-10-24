# rless behavioral specification

This package specifies the less 712 compatibility target. Start with
[scope](scope.md), [contract format](format.md) and the seven behavior chapters
below. The [parent requirements](../docs/rless-compatibility-spec.md) retain their
original IDs. Detailed requirements and interfaces are indexed in
[requirements.json](requirements.json) and [capabilities.json](capabilities.json).
The [observed defaults](defaults.md) record concrete option-query results;
[coverage.json](coverage.json) records measured interface and case coverage.

1. [Startup, configuration and options](behavior/01-startup-options.md)
2. [Input, files and errors](behavior/02-input-files-errors.md)
3. [Terminal and events](behavior/03-terminal-events.md)
4. [Text layout and navigation](behavior/04-layout-navigation.md)
5. [Search and filters](behavior/05-search-filters.md)
6. [Languages and persistent state](behavior/06-languages-state.md)
7. [Integrations and compatibility modes](behavior/07-integrations.md)

The pinned [less user contract](reference/less.txt),
[lesskey grammar](reference/lesskey.txt) and [lessecho contract](reference/lessecho.txt)
are incorporated normative annexes: they supply the complete command, option,
environment and grammar descriptions without requiring access to the C source.
They were rendered from the selected commit, with formatting removed and version
placeholders filled. Their upstream notices and [license](reference/LESS-LICENSE)
are retained. Platform-specific paragraphs outside the selected profile do not
expand its platform scope.

Precedence between specification components:

1. Explicit profile scope and enumerated differences.
2. Explicit corrections and behavior tables in these chapters.
3. Recorded reference expectations for the identical case/profile.
4. Incorporated manual clauses.

A disagreement is a specification defect to investigate, not permission to choose
whichever behavior is convenient. The case suite is evidence for the prose;
passing a subset of cases never removes an untested normative requirement.

[Readiness](readiness.md) records actual coverage and outstanding questions.
The [implementation notes](../docs/implementation.md) describe the translated Rust
baseline. The separate [safety migration specification](safety/README.md) defines
signal, ownership, state, FFI and verification requirements; its planned scenarios
do not change the behavioral requirements or count as implemented safety evidence.
