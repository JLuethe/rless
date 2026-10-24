# less OSC 8 helper

Derived from `less-osc8-open.sh` at upstream less commit
`7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e`. Distributed under the project's
`LICENSE-LESS` (upstream Less License).

URI parsing, local-host validation, error messages and man arguments are retained.
Local file paging uses rless; a sibling `../bin` directory is preferred on PATH,
and the default MANPAGER is rless. An existing MANPAGER is preserved. This follows
the compatible helper packaging provision in `spec/behavior/07-integrations.md`.
