# Plan: analyze less and specify rless

For the next specification-refinement stage, use the
[specification completion plan](spec-completion-plan.md). It defines how to turn
the current draft into explicit behavior contracts and reference-backed
acceptance cases before pager implementation.

## Outcome and baseline

Build a Rust pager named `rless` that can replace `less` in existing commands,
scripts, pager integrations, and interactive use. Compatibility includes the
unusual commands, configuration rules, errors, and terminal lifecycle.

The user selected the linked upstream version, **712**, rather than the locally
installed 668. Freeze the reference at
`7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e`. Moving upstream HEAD must never silently
change the contract. See the [specification](rless-compatibility-spec.md).

Initial source inspection and inventory extraction are complete. Building the
pinned reference, observing its runtime behavior, and implementing rless are
future work. The specification is a source-grounded draft; its test cases are
planned, not passing tests.

Follow-up status: the [readiness review](less-spec-review.md) has now built the
pinned reference, passed 21 upstream tests and captured 17 focused observations.
The broad behavior extraction, executable compatibility harness and full case
ledger remain outstanding. The historical procedure below describes the original
plan; the review evidence records what has since been executed.

## Work sequence and completion gates

| Phase | Work | Deliverable | Gate to proceed |
| --- | --- | --- | --- |
| A. Source reconnaissance | Read manuals, option/command tables, startup, configuration, security, build choices, NEWS, and upstream tests. | Pinned source audit, interface inventory, initial requirement IDs. | Every discovered interface is inventoried or explicitly identified as dynamic, internal, obsolete, or platform dependent. Initial pass complete; classification needs runtime confirmation. |
| B. Reference environment | Build 712 with explicit regex and terminal support. Record compiler, OS, libc, locale, terminfo, feature macros, and executable hash. Run upstream checks. | Reproducible reference manifest and test results. | Reference binary reports 712; its own tests run and unexplained failures are resolved. Installed 668 is never substituted. |
| C. Behavioral extraction | Exercise every option family, command action, environment variable, and integration under controlled conditions. Resolve documentation/source disagreements. | Small fixtures and expected traces linked to requirements. | Every inventory entry has a profile, effective default, argument grammar, state transition, error behavior, and test mapping. |
| D. Spec refinement | Replace unresolved observations with measured behavior; define platform profiles and explicit exceptions. | Versioned executable compatibility spec and issue ledger. | No unknown behavior remains for the first implementation milestone. Remaining unknowns have owners and block their associated milestones. |
| E. Rust design spikes | Test byte-oriented input, terminal control, regex compatibility, Unicode widths, key decoding, and cancellation. | Architecture decisions and focused prototypes. | Candidate dependencies can express upstream behavior; dependencies that cannot are replaced or isolated behind adapters. |
| F. Incremental implementation | Implement the milestones below using the reference fixtures as acceptance tests. | Working Rust binary and regression suite. | Each milestone passes its cases and earlier regression cases. Partial milestones are never described as full equivalence. |
| G. Compatibility release | Close the complete inventory, exercise platform profiles and real pager integrations, measure performance, and review packaging. | Compatibility report, manuals, binaries, supported-profile list. | All requirements for each claimed profile pass; no silent no-op options or unexplained differences remain. |

Phases B–D form the requirements investigation. Phase E should settle the risky
design decisions before bulk implementation. Work can progress by subsystem
after its requirements are resolved; a single unresolved platform detail need
not block unrelated investigation.

## Repository and evidence layout

Current documents:

- `docs/less-source-audit.md`: baseline, evidence, differences, source map.
- `docs/less-interface-inventory.md`: extracted options, bindings, lesskey actions,
  and documented environment variables.
- `docs/rless-compatibility-spec.md`: requirements, acceptance scenarios, decisions.

Planned implementation artifacts, not created by this planning task:

- `compat/reference.json`: commit, executable digest, configure flags, tool and
  library versions, feature macros, terminal database and locale identities.
- `compat/cases/`: argv, environment, file bytes, terminal size, input events,
  synchronization checkpoints, expected outputs and side effects.
- `compat/requirements.json`: one row per requirement and interface entry, with
  sources, profile, case IDs, implementation status, and latest observed result.
- `tests/compat/`: subprocess and PTY comparison harness; platform-specific runner.
- `docs/decisions/`: regex, terminal, Unicode, platform, and packaging decisions.

Keep reference sources/build products outside the rless checkout. Do not vendor
the entire C project into the Rust source tree just to inspect it. Keep notices
and provenance when importing upstream fixtures or material under the existing
GPLv3 project license and upstream's applicable license terms.

## Reference build procedure to validate in phase B

On Linux, install a C compiler, make, autoconf/autoheader, m4, Perl, and terminal
development headers/libraries. Manual generation additionally needs nroff/groff.
Use the existing isolated cloud checkout; no extra Git worktree is needed.

```sh
git clone https://github.com/gwsw/less.git /tmp/rless-less-712
cd /tmp/rless-less-712
git checkout --detach 7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e
git rev-parse HEAD
make -f Makefile.aut all
./configure --with-regex=gnu
make -j2
./less --version
./less README > /tmp/rless-less-712-readme.out
cmp README /tmp/rless-less-712-readme.out
make check
```

These are source-derived instructions, **not yet executed**. Start with two build
jobs and adjust to actual resources. Record `defines.h` and `config.log` as build
evidence. Use the normal `less` executable for production behavior comparisons;
`make check` builds a separate `less_t` with test instrumentation. Preserve
individual command exit statuses and the reported number of tests.

The upstream `lesstest/runtest` requires the `LESSTEST` protocol and explicitly
rejects an ordinary binary. First run it against upstream. For rless, translate
its fixture inputs into the independent PTY harness, or implement a separate
test adapter after inspecting the protocol. Merely pointing `-l` at rless is not
a valid reuse strategy. Do not require test instrumentation in the shipped pager.

## Behavioral investigation method

For each requirement:

1. Read the manual and implementing functions; record immutable source links.
2. Create the smallest input demonstrating the behavior and a boundary/failure
   input. Include both startup and runtime changes where applicable.
3. Run the pinned reference in a controlled profile. Capture output streams,
   exit status, terminal state, screen checkpoints, spawned commands and files.
4. Repeat to establish determinism. Fix nondeterministic fixtures before using
   them as an oracle. Document disagreements between documentation and execution.
5. Add a stable case ID and expected observation to the requirement record.
6. Run the same case against rless once that subsystem exists. Classify every
   difference; do not refresh reference outputs merely to make rless pass.

Required coverage dimensions include seekable files vs pipes/FIFOs, stdout/stderr
TTY combinations, empty and truncated input, long and invalid byte sequences,
terminal sizes and capabilities, locale, regex backend, configuration precedence,
signals, process exit, filesystem failures, and option interactions.

Use pairwise combinations for broad option coverage plus explicit higher-order
cases where the source shares state: `-F/-X/-E`, `-S/-N/--header`, search flags and
filters, `LESS` plus runtime resets, and secure-mode exceptions plus helpers.

## Implementation milestones

| Milestone | Required behavior | Acceptance families |
| --- | --- | --- |
| M0: Oracle and harness | Pinned reference, reproducible tests, terminal emulator/checkpoint protocol, requirement ledger. | Harness self-checks against two reference runs; deliberately changed input must produce a detected difference. |
| M1: Invocation and byte transport | CLI grammar, environment options, operands, stdin, redirected stdout, errors, basic exit lifecycle. | T-CLI, T-IO, T-CONFIG, relevant T-EXIT cases. |
| M2: Interactive paging | Terminal lifecycle, wrapping/chopping, byte/line/display positions, motion, jumps, prompts, marks, resize and suspend. | T-NAV, T-DISPLAY, T-TERM, T-PROMPT. |
| M3: Search and editing | Compatible regex backend, search/filter modifiers, highlights, history, command-line editing and lesskey. | T-SEARCH, T-EDIT, T-CONFIG. |
| M4: Complete Linux feature surface | Multi-file operations, streaming/follow, headers, tags, mouse, OSC 8, external commands, preprocessors, logs, secure and more modes, uncommon options. | T-FILES, T-FOLLOW, T-INTEGRATION, T-SECURE, T-MORE and all remaining inventory cases. |
| M5: Release profiles | Performance, robustness, documentation and packaging; macOS/BSD and Windows parity where claimed. | T-PERF, T-ROBUST, T-PLATFORM, all earlier suites. |

These milestones order the work; M1–M3 are useful development checkpoints but
do not satisfy the user's functionality-equivalence goal.

## Decisions to settle with evidence

- **Regex:** GNU, POSIX, and PCRE builds do not share one dialect. Start with the
  explicitly built Linux/GNU profile, then define other profiles separately.
  Test backreferences, match selection, locale and invalid bytes before picking
  a Rust regex dependency or narrow FFI adapter.
- **Terminal:** determine whether the backend supports terminfo overrides,
  alternate-screen policy, control keys, paste, mouse, and terminal restoration.
- **Text:** preserve raw bytes and file offsets; do not require valid UTF-8.
  Compare width behavior against the pinned Unicode tables.
- **Platforms:** Linux is the first implementation profile. macOS/BSD and Windows
  require their own reference builds; DOS/OS2/OS9 remain inventoried and need an
  explicit scope decision before any universal replacement claim.
- **Identity:** use the name rless in identity output while retaining upstream
  option names, `LESS*` variables and lesskey/history discovery. Document the
  precise normalization allowed in comparisons.

Do not estimate delivery dates until phase C has measured the case count and
phase E has resolved the regex/terminal risks. Size milestones from verified
behavior, not C line count.
