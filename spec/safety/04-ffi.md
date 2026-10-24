# FFI boundaries and unsafe review

Workstream 4. Compatibility parents: SEARCH-02, TERM-01/02/03, DISPLAY-02,
IO-01/03 and EXT-01/02/03/04. The supported platform remains fixed by the scope.

## SAFE-FFI-001 Boundary allowlist

Native calls MUST be concentrated in a private platform/native layer exposing safe,
owned APIs to the application core. Initial allowed categories are terminal/I/O
and signal primitives, GNU regex, terminfo/capabilities and unavoidable process
startup support. Allocation adapters may exist only as an explicitly tracked
migration boundary until their application callers have migrated.

An unsafe layer that contains pager parsing, navigation, rendering or search
control flow is not an acceptable final boundary. Native GNU regex implementation
code remains external; matching control/state and offset mapping belong in safe
Rust. Compiling or launching the old C pager to provide rless behavior is forbidden.

Raw modules and native handles MUST be private. Public application-facing APIs
accept slices, typed configuration, owned/scoped resource wrappers and Results;
they must not require a caller to uphold undocumented raw-pointer preconditions.

## SAFE-FFI-002 Canonical ABI definitions

Use canonical target definitions from maintained bindings or an opaque ABI bridge
compiled against the selected system headers. Hand-copied glibc FILE, regex,
signal, termios and stat layouts duplicated through generated pager modules MUST
be removed. Pointer-only native objects should remain opaque where possible.

Where layout crosses the boundary, verify size, alignment, field offsets,
integer widths, signedness, constants and ownership with native ABI probes.
Generated bindings need pinned generator/header identities and regeneration rules.
A small C ABI shim is permitted for layout/macros/opaque allocation; it must not
contain pager application logic. All such native code joins the reviewed trust
boundary and receives sanitizer coverage appropriate to its compilation.

The existing Linux x86_64/glibc target remains the only claimed platform.
Unsupported targets must fail clearly; a successful build on an unverified ABI
does not authorize a compatibility/safety claim.

## SAFE-FFI-003 GNU regex ownership and semantics

An owning compiled-pattern wrapper MUST release its native object exactly once,
keep compile-error lifetimes valid, and expose validated match/submatch ranges
into the input slice. Empty and zero-length matches require progress rules.
Lengths/offsets that do not fit native regoff_t must be handled explicitly without
truncation, changed matches or unchecked pointer construction.

Compilation/matching must preserve the existing GNU syntax, backreferences,
leftmost-longest selection, case rules and source-byte mappings. Any process-global
native syntax setting must be serialized/restored or otherwise scoped with a
documented proof. A different regex engine is not an allowed shortcut.

Native execution and uninstrumented library code remain an explicit trust limit.
No safety test may claim to have checked native internals merely because the Rust
wrapper passed Miri or an instrumented caller build.

## SAFE-FFI-004 Terminal and capability lifetimes

Terminfo capability strings and expansion results MUST be copied or borrowed only
for their documented native lifetime. Static scratch storage, overwritten tparm
results, null/error sentinels and terminal reinitialization must not yield dangling
or mutable aliases. Geometry conversion and row/column arithmetic are checked.

Read/write and terminal wrappers must state valid descriptor ownership, buffer
initialization, short-I/O semantics and native error behavior. A C read result
defines the initialized slice length; bytes beyond it must remain inaccessible.
Callbacks used for terminal output cannot retain a temporary buffer/context or
alias an already-borrowed output state.

## SAFE-FFI-005 Sentinels callbacks and errors

Model native sentinels as values with explicit checked interpretation, not callable
function pointers or valid Rust references. This includes signal dispositions,
null/-1 capability results, error pointers and partially initialized native objects.

Every wrapper must document whether native code retains pointers, invokes callbacks,
changes global state or errno, and can reenter Rust. Callback context lives until
the final possible invocation, is inaccessible through competing mutable borrows,
and is cleared on all paths. Preserve errno where the wrapper promises to preserve
it, and return the primary failure when cleanup also fails.

## SAFE-FFI-006 Unsafe proof records and compiler enforcement

For every remaining unsafe item, maintain an inventory record containing:

- stable item ID, source location/hash, native symbols and category;
- allocation/resource owner and release operation;
- pointer validity, initialized length, alignment, aliasing and lifetime invariants;
- thread, signal and callback assumptions;
- caller preconditions and why the safe wrapper establishes them;
- failure/unwind behavior, focused tests and reviewer/sign-off evidence.

Each unsafe block needs a local SAFETY explanation referring to these invariants.
Enable `unsafe_op_in_unsafe_fn` as an error for reviewed code; a large unsafe
function body wrapped in one block is not a completed proof. Missing safety
documentation and undocumented unsafe blocks must fail the safety lane.

Converted core modules MUST forbid unsafe code, direct FFI declarations and bypass
attributes. Compiler/AST checks must cover macro-expanded/generated application
code and unsafe trait implementations. Lower textual counts alone do not satisfy
the gate, and moving the same raw operations into a conveniently named module
does not establish a safe abstraction.

## SAFE-FFI-007 Native and vendored trust inventory

Record locked Rust dependencies, vendored sources/patches, native library and
header versions, loaded library identities and compiler/linker flags. Inventory
unsafe code in dependencies separately from the application's allowlist.

Review ABI assumptions, public soundness contracts, known advisories and local
patches. An advisory finding needs impact analysis and resolution; an empty
advisory report does not substitute for unsafe review. The C2Rust bitfield support
must be removed when its generated application types disappear, or remain an
explicitly reviewed dependency of the narrow ABI layer.

The final report MUST state the remaining trusted native boundary and tested host
configuration. It must not describe a GNU-regex/terminfo FFI build as entirely
pure safe Rust or independently verified native-library memory safety.
