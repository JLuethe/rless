# Layout and navigation contract

Parents: DISPLAY-01/02/03/04 and NAV-01/02/03. The COMMANDS, OPTIONS and NATIONAL
CHARACTER SETS clauses in [less.txt](../reference/less.txt) are the complete
normative definitions of each named action and rendering option. The catalogue
lists every default binding, including aliases and appended command bytes.

## Coordinate model

Maintain source byte offset, logical file line, display segment/row, horizontal
column, target row and viewport top independently. A wrapped file line occupies
several display rows. Screen-line motion, newline motion and numeric line jumps
therefore have different results. Counts belong to the pending command and are
consumed according to its action; half-window/window counts can persist as new
defaults while ordinary motion counts do not.

## Text layout

Character classification follows LESSCHARSET/LESSCHARDEF, locale fallback,
UTF/binary formatting and the pinned Unicode interval tables. The hashes of
`compose.uni`, `fmt.uni`, `omit.uni`, `ubin.uni`, `wide.uni` are frozen in the
profile. An implementation may generate equivalent Rust interval data; host
wcwidth or a contemporary Unicode crate must not silently replace those rules.
Preserve invalid bytes, partial multibyte sequences, combining characters and
double-width cells without losing their source offsets.

Tabs advance to the selected absolute/repeating stops. Backspace, CR, overstrike
bold/underline and the `--proc-*` triple modes have separate transformations,
including their interactions with search highlighting. Raw display mode is not
a global UTF-8 validation switch. Restricted ANSI mode passes recognized SGR and
allowed OSC sequences while accounting for their zero display width; unrestricted
raw mode can have terminal-dependent effects. Incomplete/malformed escapes and
the LESSANSI* allow/terminator variables follow their grammar in the annex.

Wrap at available display width unless chopping/headers require horizontal layout.
Line numbers/status columns reduce usable width. Headers freeze the selected
rows/columns, and search/header numbering exclusions apply separately. `-s`
squeezes repeated blank lines; `-~` changes EOF tildes. Status, attention, search,
OSC link and target attributes must compose and reset without leaking into later
ordinary text. Horizontal scrolling uses columns, not UTF-8 byte counts.

## Motion action rules

| Family | Default/count behavior |
| --- | --- |
| Forward/back one line | One display row, or count rows; stop at ordinary endpoints. |
| Forced line/page | Permit documented movement past endpoint; do not conflate with ordinary motion. |
| ESC-j / ESC-k | Move by logical file lines, independently of wrapping. |
| Forward/back page | One configured window, or count lines. |
| z / w | Move by a window and update its count when supplied. |
| d / u | Move by remembered half-window; supplied count changes that remembered value. |
| Horizontal left/right | Half display width or explicit count; preserve logical vertical position. |
| Right-limit scroll | Like right shift but stop at the longest visible line's limit. |
| First/last-column commands | Reset or reach the relevant visible-line limit. |
| g / G | First/last line without count; numbered line when supplied. |
| ESC-G | Last already-buffered input, without requiring complete pipe EOF. |
| p / % | Beginning without count; fractional/whole percentage positioning with count. |
| P | Byte-position jump, distinct from percentage and logical line number. |

Apply the configured target row, including negative/fractional settings, after
jumps/searches. Recompute fractional targets on resize. Repaint (`r`) preserves
buffered input; flush-repaint (`R`) invalidates it before rereading.

## Marks, brackets and file state

`mX` marks the top line, `MX` the bottom line, or the supplied line. User marks
are letters in either case. Jumping to a mark restores its file and recorded
screen row unless an explicit positioning count overrides it. `''` and Ctrl-X
Ctrl-X return to the previous large-movement position. ESC-m clears a mark.
Special marks include beginning `^`, end `$`, top `.`/`:`, bottom `;`, middle `,`,
target `_`, and last search match `@`. An unset/cross-file-invalid mark produces
the applicable message, not an invented origin position.

Bracket actions search for a matching nested bracket starting with the selected
occurrence on the top/bottom line, with the documented forward/reverse pairing.
Arbitrary paired characters use the ESC-control commands. User marks persist
across file changes, and with `--save-marks` across permitted history saves.
Removing a file list entry invalidates associated state as documented.
