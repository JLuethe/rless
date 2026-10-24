# Compatibility scope

## Target profile

Profile ID: `linux-gnu-712`. Reference commit:
`7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e` from `gwsw/less`.
The exact build, effective preprocessor definitions, libc/compiler identities,
Unicode table hashes and terminal data are recorded in
[reference.json](../compat/reference.json).

The first implementation target is a Unix terminal on Linux x86_64, glibc 2.41,
GNU regex, terminfo, normal non-permanently-secure build with runtime secure
policy available. Search highlighting, user lesskey files, tags, shell/editor,
history, logging, preprocessor and OSC 8 features are enabled as recorded.
Locale fixtures use `C` and `C.UTF-8`; terminal fixtures use the pinned
`xterm-256color`, `vt100` and `dumb` definitions. Case applicability states the
terminal and locale actually exercised. Supporting a terminal definition is not
a claim that every hardware terminal implements its advertised capabilities.

macOS/BSD, Windows consoles, DOS, OS/2, OS9, alternate regex engines and
permanently-secure builds require separate future profiles. Their scope is
explicitly deferred; they are not silently counted as passing Linux cases.
An interface absent from a terminal's key capabilities has an inactive binding
in that terminal, while any other alias remains active.

## Observable equivalence

For identical profile, initial files/configuration, fd topology and events,
rless must preserve accepted syntax, state transitions, displayed text/styles,
cursor, terminal modes, bells, permitted hyperlinks, byte offsets, child inputs,
file contents/permissions, diagnostics, output streams and process status.
Equivalent repaint byte sequences are allowed if they have the same observable
terminal effects at every specified checkpoint and required intermediate event.
Non-TTY output and externally passed file/pipe bytes are compared exactly.

Inputs are byte sequences. Valid UTF-8 is not a precondition. Filename encoding,
shell quoting, pipe blocking and filesystem errors are part of the contract.
Arithmetic/resource behavior must be defined at the boundaries recorded by
cases and limits; C memory layout, allocation strategy and undefined behavior
are not implementation requirements.

## Enumerated differences

`DIFF-IDENTITY-01`: binary/package name and version/attribution may truthfully say
rless and declare compatibility with less 712. Identity substitutions are allowed
only in the version banner, help title, and explicit program-name advice in
diagnostics. They do not authorize replacing arbitrary input text, filenames,
prompts, localized messages or error wording. The current reference expectations
store original less text. A candidate comparator must use an explicit identity
field adapter; the supplied replay command currently performs strict comparisons.

Fixture directory paths are normalized to `{ROOT}` in captured outputs. No other
normalization, whitespace stripping, SGR removal or ignored status is permitted.
There are no intentional semantic feature omissions in this Linux target.

## Performance and robustness

Semantic equivalence and performance claims are separate. Ordinary startup must
page incrementally, with explicit full-input operations retaining their blocking
semantics. Use byte offsets capable of addressing files beyond 4 GiB, bounded
cancellation checks and recoverable allocation/I/O errors. Exact latency/RSS
budgets require measured workloads; no unmeasured speedup is claimed here.
Uncatchable termination cannot promise terminal restoration. Restoring state on
supported normal/error/catchable-signal exits is required.
