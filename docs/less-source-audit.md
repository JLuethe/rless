# less 712 source audit

## Baseline and work performed

- Project checkout: `JLuethe/rless`, initial commit `f04c2fa`, originally only
  README and GPLv3 LICENSE. No application, Cargo manifest, or tests existed.
- Reference: `https://github.com/gwsw/less`, commit
  `7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e`; commit subject `v712`.
- User explicitly selected this upstream version as the compatibility target.
- Inspected a temporary read-only-in-practice source checkout outside rless,
  including the manual, help, source tables, startup, build files and tests.
  The temporary checkout is not a required persistent artifact; links below
  identify the exact source for future reproduction.
- Extracted source interface tables into the companion inventory and checked
  extraction counts against the source table entries.
- The host's installed executable reports **less 668 (GNU regular expressions)**.
  It is not the 712 oracle and was not used to establish 712 runtime behavior.
- No upstream build, upstream test suite, differential tests or rless tests were
  run in this planning task. Runtime observation remains an explicit plan phase.

## Immutable source map

All links use the selected commit, not a moving branch.

| Key | Source | Purpose |
| --- | --- | --- |
| S1 | [NEWS](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/NEWS), [version.c](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/version.c) | Version changes and identity. |
| S2 | [less.nro.VER](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.nro.VER), [less.hlp](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.hlp) | User contract, syntax, commands, variables and security. |
| S3 | [opttbl.c](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/opttbl.c), [option.c](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/option.c), [optfunc.c](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/optfunc.c) | Option grammar, aliases, defaults, handlers and runtime changes. |
| S4 | [main.c](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/main.c), [less.h](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less.h) | Initialization order, modes, feature gates, exit handling. |
| S5 | [decode.c](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c), [command.c](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/command.c), [cmdbuf.c](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/cmdbuf.c) | Command tables, dispatch, line editing and history. |
| S6 | [ch.c](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/ch.c), [edit.c](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/edit.c), [filename.c](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/filename.c), [ifile.c](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/ifile.c) | Buffering, file lifecycle, preprocessors and filename handling. |
| S7 | [forwback.c](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/forwback.c), [jump.c](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/jump.c), [mark.c](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/mark.c), [brac.c](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/brac.c), [position.c](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/position.c) | Navigation, positions, marks and bracket matching. |
| S8 | [search.c](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/search.c), [pattern.c](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/pattern.c) | Search state, filtering, highlighting and regex backend differences. |
| S9 | [line.c](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/line.c), [charset.c](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/charset.c), [linenum.c](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/linenum.c), `*.uni` | Layout, bytes/characters, Unicode widths and line numbers. |
| S10 | [lsystem.c](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lsystem.c), [tags.c](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/tags.c) | Subprocesses, editor/pipe integration, tags and feature restrictions. |
| S11 | [screen.c](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/screen.c), [ttyin.c](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/ttyin.c), [signal.c](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/signal.c), [os.c](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/os.c) | Terminal and platform abstraction, signals, input polling. |
| S12 | [prompt.c](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/prompt.c) | Prompt expansion and conditional grammar. |
| S13 | [lesskey.nro.VER](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey.nro.VER), [lesskey_parse.c](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesskey_parse.c), [evar.c](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/evar.c), `decode.c` | Configuration grammar, actions, discovery and precedence. |
| S14 | [lmsg.c](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lmsg.c), [lessmsg](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lessmsg), `lessmsg_int` | Translation identifiers, formatting and messages. |
| S15 | [lessecho.c](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lessecho.c), [lessecho.nro.VER](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lessecho.nro.VER), [less-osc8-open.sh](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/less-osc8-open.sh) | Companion helper interfaces and default hyperlink handlers. |
| S16 | [lesstest/README](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesstest/README), [lesstest/runtest](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/lesstest/runtest), [Makefile.in](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/Makefile.in), [.github/workflows/main.yml](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/.github/workflows/main.yml) | Existing replay protocol, build/test procedure and CI. |

## Findings that affect the design

1. **712 changes startup rendering.** NEWS says `-R` is now enabled by default.
   `opttbl.c:init_option` sets `ctldisp = OPT_ONPLUS` after initializing table
   defaults, deliberately preserving the older `-R`/`-+R` option semantics.
   Copying the table defaults alone would implement the wrong behavior.
2. **Removed functionality still has parser surface.** NEWS removes compiled
   binary lesskey support; `optfunc.c:opt_k` still handles `-k` by reporting it
   unsupported. Both rejection behavior and source-lesskey support matter.
3. **The help page is not the whole interface.** The option table contains
   undocumented/internal entries such as `--old-bot`, and `--tty` is gated by
   `LESSTEST`. Source-table inclusion alone does not prove production availability.
4. **Configuration order is unusual.** Local lesskey environment values override
   the process environment, which overrides system lesskey values. CLI option
   scanning then overrides the selected environment option string. `LESSNOCONFIG`
   changes the lookup behavior and requires its own tests.
5. **Regex is a build property.** README/configure support GNU, POSIX, PCRE/PCRE2,
   other historical backends and no regex. A pure-Rust regex crate cannot be
   presumed equivalent to all of them; the build profile must name a backend.
6. **A pager is a terminal state machine.** Separate paths handle file vs screen
   lines, stdin vs keyboard input, output redirection, signals, child programs,
   alternate screens, paste, mouse and terminal overrides.
7. **Existing tests are useful but coupled.** There are 21 `.lt` fixtures. They
   replay keystrokes and compare terminal screens through `lesstest`/`lt_screen`.
   The runner checks for the `LESSTEST` version marker and uses special I/O;
   compatibility testing of an ordinary rless binary needs a PTY harness or adapter.
8. **Documentation can lag implementation.** README still mentions generating a
   `lesskey` program while NEWS announces its removal. Resolve this through current
   Makefile targets/parser behavior and runtime tests, not README alone.

## Evidence limits

The extracted inventory is a coverage seed, not a fully resolved build manifest.
Preprocessor guards, runtime initialization, dynamic key expansion, dynamically
formed variable names and helper-specific behavior require further analysis.
Counts include conditional entries; they are not a claim that all are enabled
in every binary. Source readings establish requirements, not passing behavior.
