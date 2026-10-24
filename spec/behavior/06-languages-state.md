# Languages, editing and persistent state

Parents: PROMPT-01, EDIT-01, CONFIG-01/03/04. The complete local
[lesskey grammar](../reference/lesskey.txt), plus LINE EDITING, PROMPTS,
KEY BINDINGS and ENVIRONMENT in [less.txt](../reference/less.txt), are normative.
This chapter supplies initialization/format details and resolves abbreviated
references in the parent requirements.

## Input-line editing

Input state includes byte buffer, cursor, insert/overwrite mode, prompt context,
history selection, completion list and literal-next status. Movement/deletion
works at the applicable character/word boundaries, preserves valid byte indexing
and honors terminal erase/kill/word-erase characters. Ctrl-G aborts; command-specific
erase-empty behavior can abort too. TAB/back-TAB cycle filename completion;
Ctrl-L expands matches. History is context-specific. Invalid keys and completion
failures preserve the buffer except where an explicit action changes it.

The full default bindings and special-key byte expansions are in the catalogue.
Their semantic action names are the corresponding lesskey LINE EDITING entries.
Key aliases share a transition but every active encoding must be accepted.

## Prompt language

Parse literal text, percent expansion, `?condition then :else .`, nested
conditionals, backslash-quoted metacharacters and prompt-selector variants using
the PROMPTS clauses. Each expansion has a defined value or an unknown-information
fallback. Preserve distinctions among displayed-line positions, logical lines,
bytes, percentage of bytes/lines, current/next file and unknown stream size.
Do not eagerly scan a pipe solely to manufacture a percentage omitted by the
reference. `%%`, `%G`, `%y` are part of 712's language.

The default short/medium/long, status, help, waiting and end prompts are separate
templates. Prompt selection and `-P` selectors do not rewrite the help prompt
as an ordinary file prompt. Output formatting, punctuation, escaping and column
truncation belong to compatibility.

## lesskey grammar

A source begins in command context unless a section says otherwise. Sections are
`#command`, `#line-edit`, `#env`; `#stop` ends fallback lookup for its context.
Command lines are key-string, whitespace, named action, optional extra key string.
The vocabulary includes all 77 command and 23 editing action names/aliases in the
catalogue. Key strings accept caret controls, octal and documented backslash/
special-key escapes. Extra bytes are queued as if typed after the selected action.
Unescaped comments/blank lines and maximum key lengths follow the grammar annex.

`#version OP N remainder` conditionally parses the remainder, with the six
documented relational operators. Evaluate the compatibility version as **712**,
independently of the rless package version. Unknown operators/action names and
malformed key strings generate location-qualified parse errors.

Environment definitions use `NAME = value`; whitespace after `=` is skipped,
trailing content is retained according to the annex. `+=` appends literally to
an immediately preceding definition of the same variable; it cannot append a
process variable or skip intervening other definitions. `${NAME}` expands the
available earlier environment. `${NAME/from/to/...}` performs literal replacement,
not regex replacement. Escaping `/` and `}` requires the documented double
backslashes. Inline content uses newline or unescaped semicolon as separators.
Do not add an include directive absent from the pinned grammar.

## Modelines

With `--modelines=N`, inspect only the first N file lines before ordinary display;
zero disables the scan. Recognize `less:`, `vim:`, `vi:`, `ex:` at start or preceded
by a space. `less:` requires `set `; the others can omit it. With `set`, settings
end at a colon; otherwise colon or space separates them. Only `ts=` and
`tabstop=` change tab stops in this implementation profile. Unknown settings are
ignored; there is no general shell/config execution facility in a modeline.
Tab-stop values use the same parser as `-x`.

## History and marks

`LESSHISTFILE=-` disables persistence. Otherwise explicit LESSHISTFILE wins; Unix
discovery checks XDG_STATE_HOME, `$HOME/.local/state`, XDG_DATA_HOME, then HOME,
using `lesshst` in the XDG/state directories and `.lesshst` in HOME. Reading prefers an existing history file; writing follows the selected history location. The first line is
`.less-history-file:`. Sections `.search`, `.shell`, `.mark` distinguish contents.
Search/shell entries start with a literal double quote and contain the rest of
the line as the entry; CR/newline terminates it. Unrecognized headers/content
follow the reference's tolerant reading rules; an invalid first header prevents
loading that file.

History updates merge existing permitted history with newly modified entries,
respect duplicate policy, and limit each command list by LESSHISTSIZE (default
100 when unset/nonpositive). A history temporary path replaces the last filename
character with Q, or Z when it was Q; regular temp files are made owner-readable/
writable only (0600), then committed according to the reference update sequence.
Marks are written only when enabled and permitted; each mark record is `m LETTER SCREEN_ROW BYTE_OFFSET FILENAME` followed by newline. Screen row and offset are nonnegative decimal; the remaining filename can contain spaces. Stdin marks are not serialized. Invalid letter/negative row/negative offset records are ignored. Both active and persistent mark tables receive a valid restored record. Preserve failure behavior
without corrupting an existing file. Autosave action selectors trigger the same
update mechanism at their documented events; secure history disable suppresses it.

## Message translations

LESSMSG is read before lesskey initialization. Translation lines use a known
message symbol, a separating space and replacement content. Leading whitespace,
blank lines and comments are accepted as specified; unknown symbols and missing
separators diagnose file/line errors. Escapes include newline/return/tab, literal
escaped characters, octal, hex and continued lines. Byte numeric escapes must be
0–255. Replacement strings retain required formatting arguments. The special HELP
symbol replaces help data. The pinned upstream message symbol catalogue is
included with the specification reference data; translated messages do not change
option semantics or the allowed identity exception.
