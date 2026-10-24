# Startup, configuration and option contract

Parent requirements: CLI-01/02/03/04, CONFIG-02/04, SEC-01/02, LEGACY-01.
The complete OPTIONS and ENVIRONMENT clauses in [less.txt](../reference/less.txt)
are normative, expanded by the tables and corrections below. Every option and
variable has its own catalogue entry and child requirement.

## Startup sequence

Perform externally visible initialization in this order:

1. Read `LESSNOCONFIG`, then initialize message translation from `LESSMSG`.
2. Determine executable identity and the initial secure-feature policy.
3. Determine whether stdout is a TTY; initialize marks, default commands,
   system/local lesskey files and system/local inline content.
4. Initialize polling, charset, layout, history, option defaults and search state.
5. Determine `more` identity/mode, initialize prompts and unsupported-option
   policy; scan `MORE` or `LESS`, then scan argv.
6. Discover terminal capabilities for TTY output; expand capability-dependent key
   bindings; choose editor and editing template; register operands.
7. Enter redirected-copy mode or select/open the first interactive file, apply
   initial positioning/commands, initialize the terminal and dispatch commands.

Do not merge all configuration first. Early lookups cannot be overridden
retroactively by a later lesskey `#env`. Runtime `--lesskey-src`/inline content can
add tables but does not re-run security or locale initialization.

## Lookup precedence and paths

For a lookup after tables exist: local lesskey variables, nonempty process
environment, system lesskey variables, then a feature's built-in default.
Local inline content is installed after local source files; newer tables are
searched before older tables. `LESSKEYIN_SYSTEM`/`LESSKEYIN` select source files;
otherwise Unix local lookup tries `$XDG_CONFIG_HOME/lesskey`,
`$HOME/.config/lesskey`, `$HOME/.lesskey`, stopping at the first usable file.
The default system source is `/usr/local/etc/syslesskey` for this build.
`LESSKEY_CONTENT_SYSTEM` and `LESSKEY_CONTENT` add inline sections.

`LESSNOCONFIG` empty/unset leaves ordinary lookup intact. Any nonempty value
disables lesskey files/content and filters environment lookups to names in its
comma-separated list; `-` keeps no listed names. TERM is still obtained through
the terminal path. Raw `POSIXLY_CORRECT` presence is checked separately, including
an empty value. Do not treat these variables as conventional boolean parsers.

## Secure policy

The runtime feature names are `edit`, `examine`, `glob`, `history`, `lesskey`,
`lessopen`, `logfile`, `osc8`, `pipe`, `shell`, `stop`, `tags`.

| Operation | Mask result |
| --- | --- |
| `LESSSECURE` absent/empty | Start with all features enabled. |
| `LESSSECURE` any nonempty string, including `0` | Start with no features enabled. |
| Nonempty `LESSSECURE_ALLOW` | Replace the starting mask with the listed bits, even if LESSSECURE is absent. |
| Nonempty `LESSSECURE_DISALLOW` | Remove listed bits from the resulting mask. |

Feature names accept unique prefixes; spaces around comma-separated names are
trimmed. Invalid/ambiguous names produce diagnostics. Do not infer a fatal exit
from a diagnostic alone. Every entry path checks its required feature mask before
opening an editor, new file, log, preprocessor, tags provider, shell/pipe or URI
handler. Initial explicit argv files remain allowed when examine is disabled.
Compile-time permanent secure mode is outside this profile.

## CLI grammar and transitions

Options normally remain recognizable after operands; `POSIXLY_CORRECT` presence
stops option scanning at the first operand. A lone `-` is an input operand.
`--` ends scanning. A pending string/number option consumes the following argv
element according to upstream pending-option rules, even if it resembles an
option. Unknown options print a diagnostic and may continue; see the error table.

Short options may cluster where their argument grammars permit. Long names accept
unique prefixes; exact matches win over longer matching names. For triple-valued
options, capitalization of the first letter selects the upper variant; subsequent
letters may have either case. Boolean/numeric/string and triple-valued options
must not be reduced to one generic toggle mechanism.

`LESS` is scanned before argv. In option strings `$` terminates a string-valued
argument. Earlier `--use-backslash` makes backslash quote the next character;
otherwise it is literal. Numeric/string options consume only their documented
grammar, allowing a remaining option suffix to be scanned. Missing, negative and
overflow values have distinct diagnostics. For numeric options the unsigned magnitude must fit 0 through 2147483647; a leading minus is accepted only on entries marked `O_NEGOK`. Fractions use a leading decimal point;
the supported fixed-point precision is six decimal digits.

| Context/type | Transition |
| --- | --- |
| Startup boolean | Set inverse of its table default; repetition does not toggle current state. |
| Startup triple, lowercase | Set 0 if table default is 1, otherwise 1; do not toggle current state. |
| Startup triple, uppercase | Set 0 if table default is 2, otherwise 2; do not toggle current state. |
| Startup `-+` / `--+` | Restore table default. |
| Interactive boolean toggle | Invert current state. |
| Interactive triple lowercase | Select state 1 unless already 1, then state 0. |
| Interactive triple uppercase | Select state 2 unless already 2, then state 0. |
| Interactive numeric/string without a value | Query rather than change. |
| Interactive query (`_` / `__`) | Report current setting; respect no-query flags. |
| Interactive reset | Restore table default for boolean/triple/numeric; string options reject set/reset forms. |
| Interactive force-set | Apply the startup inverse-of-table-default rule for boolean/triple; numeric and string force-set forms are rejected. |

The no-toggle flag rejects ordinary interactive toggling; do not extend that check to other reset/query paths unless their own rule/handler rejects them. No-query flags reject queries. Honor feature handlers separately. A default integer in the
catalogue is initialization metadata; the user meaning is its option clause and
observed default query. The boolean `-L` disables LESSOPEN although its stored
`use_lessopen` default is true.

### 712 raw-control correction

Startup initially selects restricted ANSI passthrough (`-R`) after setting table
defaults. The table's control-display default remains 0. Thus:

| Trigger | Effective display mode |
| --- | --- |
| No raw-control option | Restricted ANSI; other controls as caret notation. |
| argv `-r` | Unrestricted raw controls. |
| `LESS=-r` or `LESS=-R` | Restricted ANSI. |
| argv `-R` | Restricted ANSI. |
| `-+R` | Caret notation, including escapes. |
| repeated argv `-r -r` | Unrestricted raw controls. |

## Startup commands and obsolete options

`+cmd` queues a command for the first file; `++cmd` installs a command for each
file. Numeric startup commands position by line; incomplete eligible commands
are completed as documented. `-p` selects an initial pattern except in more mode.
`--cmd` queues its string before the first prompt. Apply ordering and per-file
state from the command clauses rather than translating all forms to one search.

`-k`/`--lesskey-file` consumes its argument, warns that binary lesskey files are
unsupported, and continues; it does not load that binary. `--old-bot` is a real
production option: when clearing the bottom prompt line, move to the physical bottom-left instead of the current line-left before erasing; preserve/restablish attributes around the erase. It is initially off and can be toggled at runtime. `--tty` belongs only to the instrumented upstream test build and
must not become a required production option.
