# Search and filter contract

Parents: SEARCH-01/02/03. Search command descriptions and modifiers in
[less.txt](../reference/less.txt) are normative, including where searches start
relative to the target/displayed lines. Configuration must not change those
positions merely because a different internal search engine is convenient.

## Search state

Track pattern bytes, compiled expression, direction, persistent modifiers,
selected submatch, count, last-match file/byte position, incremental candidate,
highlight visibility and active filters. Starting a search, repeating it and
reversing it are distinct operations. Empty patterns reuse the applicable prior
pattern; absent prior patterns produce a diagnostic. Failure/cancellation restores
the prescribed position and does not fabricate a match.

| Prefix/modifier | Meaning |
| --- | --- |
| Ctrl-N / ! | Select nonmatching lines. |
| Ctrl-E / * | Continue across file boundaries. |
| Ctrl-F / @ | Start at first file for forward, last file for reverse search. |
| Ctrl-K | Highlight without moving. |
| Ctrl-R | Literal search, bypassing regex metacharacters. |
| Ctrl-S followed by a digit | Select a parenthesized subpattern. |
| Ctrl-W | Wrap when reaching the search boundary. |
| Ctrl-L | Quote the next input character literally. |

Modifiers are toggled/retained according to the COMMANDS clauses. Wrap and
cross-file behavior have explicit mutual interactions; do not independently loop
forever through both. `n` repeats prior direction, `N` reverses, ESC-n/ESC-N span
files. Initial-pattern options and more-mode command interpretation are separate.

## GNU regex

The frozen backend uses glibc GNU regex with `RE_SYNTAX_POSIX_EXTENDED`, including
its match/submatch selection, backreferences, ranges/classes, anchors, alternation,
repetition and error behavior. Matching is against the pager's transformed search
text with mappings back to source byte positions. It is not PCRE or the syntax
of Rust's common `regex` crate. Invalid compilation reports `Invalid pattern`.

Case modes: ordinary mode is sensitive; `-i` ignores case only when the pattern
contains no uppercase letters under the applicable conversion; `-I` ignores it
unconditionally. Use the same charset/locale transformations for literal and
regex paths as specified; preserve byte offsets when conversions change length.
Zero-length matches must make search progress without losing repeat/count rules.

A Rust implementation can use target glibc GNU-regex bindings through a narrow
opaque wrapper while implementing paging in Rust. Bindings must use target ABI
definitions, not guessed struct layouts. A pure-Rust replacement backend is
acceptable only after matching this contract, including leftmost-longest
selection and backreferences. Regex backend substitution is not an allowed
compatibility exception.

## Highlighting, incremental search and filtering

Preserve all/last/no-highlight modes, temporary undo vs complete clear, target
row and horizontal match-shift placement. Search over text with ANSI display
sequences must map highlights back to displayed spans without coloring terminal
control bytes as text. Header lines and columns have independent exclusions.

Incremental search updates the candidate viewport/highlight as editing changes
the pattern; cancel restores the prior state. Committing makes the candidate the
ordinary search history entry. Invalid partial patterns must not destroy the
last valid compiled search.

`&` filters which lines are displayed rather than changing source bytes or logical
line numbers. Multiple filters and their inversion/subpattern rules follow the
annex. Navigation, prompts, line numbers, search and clearing filters must remain
consistent with the original file. Hidden lines are not deleted or logged as
modified input.
