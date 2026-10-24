# Review: is the specification sufficient for an equivalent rless?

**Verdict: sufficient to start the reference harness and architecture work;
insufficient as a standalone implementation contract for functional equivalence.**
The feature coverage is a useful foundation. However, 39 family-level requirements
and 18 scenario families still delegate most exact behavior to the C source.
Implementers following only those descriptions would have to guess observable
rules. The existing document correctly calls itself a draft.

Reviewed revision: `f6837e960e374fb1f4d24aa75cbb6374fba038a0`.
Reference: less 712, `7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e`.
The correction and reference observations linked below are local review additions;
they do not close the remaining specification work.

## Evidence gathered

- Built the exact reference commit on Linux x86_64 with GCC 14.2.0, glibc 2.41,
  GNU regex, terminfo, and a non-permanently-secure configuration.
- Confirmed the normal executable reports `less 712 (GNU regular expressions)`.
- Ran the upstream redirected-README check: byte-for-byte comparison passed.
- Ran `make check`: **21 replay tests, 0 errors**. The initial test-tool build
  lacked the locally extracted terminal include path; supplying the include and
  library paths to the recursive build corrected that setup failure.
- Executed **17 focused observations** against the normal reference executable:
  10 non-TTY invocations, five PTY option queries, and two live pipe/PTY sessions.
  All sessions completed. These observations are not a full differential suite
  and do not establish rless conformance.
- Examined option transitions, initialization order, secure-mode parsing, file
  opening/copying and size scans in the pinned source.

Build tools and headers were extracted locally from packages selected by a signed
Debian package index; no signature/TLS verification was disabled. Generated source
and executables remain outside rless, under `/tmp/rless-less-reference`.
The [evidence record](verification/less-712-review.json) includes executable and
configuration hashes, relevant build macros, and normalized observations.

## Findings, in priority order

### R1 — P1: the streaming rule contradicts an enabled less option

Location: spec **IO-03**, formerly an unconditional prohibition on waiting for
EOF before showing the first page.

Upstream `--file-size` explicitly scans the input on opening. With a pipe,
`--file-size` and its default zero time limit can wait for the writer to close.
The live-pipe observations showed the default pager displaying the first page
before close, while `--file-size` displayed it after close. Both were given the
same 100 complete lines while the writer remained open for a bounded observation.
Source confirms the distinction: [edit.c:695](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/edit.c#L695)
calls [linenum.c:486](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/linenum.c#L486).

**Disposition:** corrected IO-03 to distinguish ordinary incremental paging from
explicit whole-input operations. Added file-size/time-limit investigation cases.
Full cancellation, timeout and EOF behavior still needs executable fixtures.

### R2 — P1: the acceptance families have no executable expected results

Location: spec sections **2, 4 and 5**; plan's deferred case and requirement ledger.

Every option can be mentioned in an inventory while its actual contract remains
undefined. A `T-CLI` label does not specify argv bytes, starting state, expected
status or stderr; `T-NAV` does not specify the resulting top byte or cursor cell.
The repository has no complete requirements-to-case ledger, PTY comparator or
rless executable. Passing upstream's own tests only validates the reference build.

**Required closure:** assign stable cases to every production option, each binding
and configuration action, each variable/family, and additional dynamic grammars.
Each case needs preconditions, input bytes/events, profile, exact expected state
or bytes/effects/status, observation provenance, and a requirement ID. Produce
cases before implementing the associated behavior. Check coverage mechanically.

### R3 — P1: initialization order is missing from configuration precedence

Location: spec **CONFIG-02**, **SEC-01**, **CLI-02**, **DISPLAY-03**.

The local-lesskey/process/system precedence description is valid for lookups once
those tables are loaded; it is not a rule that all variables are merged before
initialization. In `main`, `LESSNOCONFIG` and `LESSMSG` are read first;
`init_secure()` runs before `init_cmds()` loads lesskey content. A later local
lesskey `#env` value cannot retroactively change that security initialization.
Charset, option, search and prompt initialization also have a defined order.
See [main.c:217](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/main.c#L217)
and [decode.c:534](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/decode.c#L534).

Observed and source-backed edge rules also need explicit entries:

- `LESS=-r` selects the restricted ANSI mode; argv `-r` selects unrestricted raw
  controls. They are not interchangeable sources of the same option value.
- `-+R` restores caret display, although 712 starts in restricted ANSI mode.
- Repeating startup `-r` twice does not toggle it off. Interactive toggles follow
  different transitions; the table's type/default does not fully specify them.
- `LESSSECURE=0` is nonempty and enables secure mode. Empty/unset is different.
  `LESSSECURE_ALLOW` replaces the starting feature mask even without LESSSECURE;
  DISALLOW then removes bits. This mask behavior was source-inspected; the two
  secure-value diagnostic observations do not test every feature gate.

**Required closure:** a startup sequence and per-option transition matrix,
including string/numeric parsing, init handlers, set/reset/query, and environment
source differences. Add configuration tests at each initialization boundary.

### R4 — P1: error outcomes remain too general to implement correctly

Location: spec **CLI-01**, **IO-01/02**, **EXIT-01** and **LEGACY-01**.

The reference observations demonstrate that conventional error handling would
break compatibility:

| Invocation with stdout piped | Observed result |
| --- | --- |
| Unknown long option followed by a valid file | Diagnostic on stderr; file copied; status 0. |
| `-P` without its argument | Required-value diagnostic on stderr; status 1. |
| `-k obsolete-file` followed by a valid file | Unsupported-format warning; file copied; status 0. |
| Missing file alone | Diagnostic on stderr; status 1. |
| Valid file followed by a missing file | Valid bytes followed by the missing-file diagnostic on stdout; status 0. |
| Missing file followed by a valid file | Diagnostic on stderr, valid bytes on stdout; status 0. |

These are observed upstream outcomes, not recommended new error semantics.
The blanket phrase "match errors" gives no implementable decision table. See
[main.c:379](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/main.c#L379)
and [option.c:235](https://github.com/gwsw/less/blob/7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e/option.c#L235).

**Required closure:** record per-path diagnostics, continuation, selected output
stream and final status for all argument/open/read/write/child failures. Preserve
the unusual reference behavior unless an explicit exception changes the target.

### R5 — P1: platform/regex/terminal profiles and text rules are not finalized

Location: spec sections **1 and 6**, **SEARCH-02**, **DISPLAY-02**, **TERM-01/03**.

Linux/GNU is an initial profile, but the spec still leaves the exact enabled
features, terminal database, effective defaults, Unicode rules, and supported
platform list to future analysis. This review records one working reference build;
it does not establish Windows or macOS parity, or even every Linux TERM/locale.

For GNU regex, `pattern.c` selects `RE_SYNTAX_POSIX_EXTENDED`; compatibility needs
the actual match selection, backreference, invalid-byte and case-conversion rules,
not just a backend label. Terminal libraries and Unicode-width crates also need
measured compatibility before architectural decisions depend on them.

**Required closure:** freeze the first release profile, record all relevant
versions/capabilities, resolve regex/terminal/text prototypes, and add exact
boundary cases. Scope later OS profiles explicitly before claiming portability.

### R6 — P2: some interfaces have no dedicated semantic contract

Location: spec **CONFIG-01/03**, **PROMPT-01** and the interface inventory.

Modelines appear in the inventory but lack an explicit requirement/case for
their grammar and timing. For example, `less: set ts=4:` differs from
`vim: ts=4`; accepted prefixes, delimiters and only-the-first-N-lines behavior
must be specified. The prompt language, lesskey expansion/directives, history and
translated-message formats likewise need grammar/format sections, limits,
malformed-input behavior and concrete examples. Inventory presence is not enough.

**Required closure:** add those contracts and fixtures. Classify production
`--old-bot` separately from test-only `--tty`; neither is resolved by calling it
"internal." Freeze the allowed identity substitutions so message comparisons
cannot hide substantive differences.

### R7 — P2: the implementation milestones need prerequisite edges

Location: plan **M1–M4**.

M1 requires configuration-sensitive invocation, but lesskey is assigned to M3.
It also needs secure policy before file-opening helpers, while secure mode and
preprocessors are grouped in M4. A basic M1 can be explicitly partial, but must
not be signed off against the complete T-CONFIG/T-CLI/T-IO families while those
dependencies are absent.

**Required closure:** make early configuration lookup/`#env` parsing and security
initialization shared prerequisites; separate them from later custom key editing
and external command features. Define milestone-specific case subsets without
weakening the eventual equivalence gate.

## Implementation-readiness gate

Proceed now with the reference harness, evidence capture and architecture spikes.
Before signing off a production subsystem, require:

1. A frozen, reproducible target profile and initialized state.
2. Explicit transition/grammar/error rules for the subsystem.
3. Executable normal, boundary and failure cases with reference observations.
4. A complete inventory-to-requirement-to-case mapping for that subsystem.
5. Resolved regex/terminal/text dependencies where the subsystem relies on them.

Full equivalence requires closing these gates across the complete production
surface and testing rless against the reference. The 21 passing upstream tests
and 17 review observations are evidence for this review, not proof that the
current spec or a future implementation already satisfies that goal.
