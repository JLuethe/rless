# Terminal and event contract

Parents: TERM-01/02/03 and EDIT-01. Terminfo/environment, line editing and mouse
clauses in [less.txt](../reference/less.txt) supply the complete feature syntax.

## File descriptors and capabilities

Determine interactive paging from stdout's TTY status. A pipe feeding stdin does
not become the keyboard: commands come from the controlling terminal through the
supported tty input path. Distinguish stdout/stderr redirection and absence of a
controlling terminal. Terminal capability discovery and key expansion occur after
argv scanning, with `TERM`, database data, `LESS_TERMINFO_*`, `LESS_TERMCAP_*`,
LINES/COLUMNS and LESS_LINES/LESS_COLUMNS precedence from the manual.

Size from the terminal ioctl supersedes ordinary LINES/COLUMNS. The LESS-specific
size settings can force or offset dimensions as documented. Resize updates geometry,
fractional jump/shift/match targets and layout; logical file position is retained
as closely as the new layout allows. Tiny dimensions must not cause invalid
arithmetic or unbounded repaint loops.

## Lifecycle state machine

`initial` → raw input/capability initialization → `paging` → temporary cooked
handoff/suspend or `exiting` → deinitialize modes, flush, close active input,
save permitted history, restore termios, release tty.

Alternate screen, keypad, paste and mouse are independently controlled. `-X`
suppresses terminal init/deinit strings; it does not turn paging into byte-copy
mode. `--no-keypad` and `--no-paste` affect their respective capabilities. `-F`
requires the initial one-screen decision before unnecessary alternate-screen
initialization; wrapped rows and status/header rows affect whether input fits.
`--redraw-on-quit` restores the visible last screen to the primary screen as
documented. A PID/open terminal alone is not an acceptance checkpoint.

## Keyboard, paste and mouse

Default byte-to-action mappings are in the catalogue. Special-key byte strings
for the frozen terminal are in [special-keys.json](../../compat/special-keys.json).
Unavailable capabilities do not create a binding. Match terminal erase, kill and
word-erase settings, ambiguous/incomplete escape sequences, literal-next input,
application keypad state and lesskey overrides. Escape bytes are inputs, not
Unicode key names invented by a UI library.

Bracketed paste has explicit start/end sequences; disable/ignore rules must not
execute pasted multiline commands as ordinary pager commands. Mouse reports
include old X11 and SGR encodings. Decode coordinates/buttons/modifiers and apply
the enabled vertical/horizontal scroll/drag/click features, reverse direction
and wheel-line setting. Invalid reports cannot corrupt command parsing.

## Signals and recovery

SIGINT normally aborts the active operation and repaints/returns to commands;
`-K` selects interrupt exit. SIGWINCH schedules geometry/layout recovery.
SIGTSTP restores terminal state before stopping, then reinitializes/redraws after
resume; secure policy can disable stopping. SIGQUIT is ignored in the normal
Unix handler set. SIGTERM/SIGHUP run cleanup and exit with status 15 in this reference, including SIGHUP; do not replace that value with a shell-style 128+signal code.
LESS_SIGUSR1 configures the documented action for SIGUSR1. Signal arrival must
not leave partially changed terminal modes or stale multi-character commands.

Before editor/shell/pipe/URI child execution, flush and hand the terminal to the
child in its expected mode. On return restore pager modes, dimensions and file
position/repaint behavior. Do not assume a child preserved terminal attributes.
Termios restoration and terminal escape-mode restoration are separate acceptance
observations; both are required where the operation supports cleanup.
