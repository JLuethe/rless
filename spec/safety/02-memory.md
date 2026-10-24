# Buffers parsing and allocation

Workstream 2. Compatibility parents: IO-01/02/03, DISPLAY-01/02/03, EDIT-01,
SEARCH-01/02/03, CONFIG-01/03, PROMPT-01 and NAV-02.

## SAFE-MEM-001 Owned byte storage

Application-owned dynamic byte buffers MUST use an owning Rust abstraction with
one allocation/deallocation authority. Input, filenames, search text and command
data remain byte sequences; invalid UTF-8, NUL, CRLF and missing final newline
retain the behavior specified for each interface.

The first implementation slice replaces xbuffer allocation/growth/pop/reset/set
semantics with an owned byte buffer and migrates its callers. It MUST define empty
buffers, self-copy/overlap, growth and reset behavior. C-string conversion is an
explicit boundary operation with a defined embedded-NUL policy. Arbitrary source
bytes MUST NOT be silently truncated by `CString`/strlen assumptions.

## SAFE-MEM-002 Size arithmetic and coordinate domains

All input-derived allocation sizes, range ends, counts and narrowing conversions
MUST be validated before use. Allocation calculations use checked arithmetic and
report the applicable error before mutating published state. Wrapping arithmetic
is permitted only where intentional behavior is documented and tested.

File byte offsets MUST retain at least the supported signed 64-bit domain and
must not be confused with usize indexes, logical lines or display columns.
Unavailable positions use typed alternatives rather than a negative value cast to
an unsigned index. Conversion to native regex/terminal integer domains MUST check
range. Files beyond 4 GiB and lines larger than an initial allocation remain valid.

## SAFE-MEM-003 Borrow lifetimes and growth

No pointer, slice or reference into a growable buffer may survive an operation
that can reallocate, replace, free or truncate that storage. Persist locations as
validated offsets/typed handles and reacquire borrows after mutation.

Multi-buffer conversions MUST state input/output overlap rules. Self-copy,
filter replacement, command history edits, highlighting and redraw cancellation
MUST not alias mutable storage. Native callbacks receive only scoped access, and
must not retain addresses after their owning call returns.

## SAFE-MEM-004 Safe parsers and text transformations

CLI numbers/options, lessecho quoting/metacharacters, lesskey, prompts/messages/history,
command editing, key/paste/mouse sequences, charset/UTF decoding and ANSI/OSC parsing MUST expose safe APIs
over bounded byte slices. Each parser defines consumed bytes, incomplete input,
success and error results without raw end-pointer arithmetic in the core.

Malformed/truncated input and empty or zero-width cases MUST make progress, request
more input, or produce the specified error. They must not panic, index outside a
slice or loop indefinitely. Preserve pinned Unicode tables and source-to-search/
display offset mappings; current host Unicode defaults are not substitutes.

## SAFE-MEM-005 Matching allocator ownership

Every native allocation MUST have one documented owner and matching release API.
Do not turn libc-owned data into a Vec/Box/String whose allocator or capacity was
not established by that Rust owner. Do not release Rust allocations with libc
free. Adoption/copy rules for environment pointers, library strings, regex buffers
and FILE handles MUST be explicit.

Partial native initialization, failed compile/open, ownership transfer and repeated
close/drop MUST be covered. Release occurs exactly once, after the last borrow.
Native lifetime management belongs behind the FFI boundary, not in application
parser/display/search code.

## SAFE-MEM-006 Streaming retention and stable views

The owned input model MUST distinguish source offsets, retained pipe ranges,
seekable rereads, incomplete last lines, temporary lack of bytes, EOF and errors.
Ordinary first-page display MUST remain independent of stream EOF. Explicit
size/end scans retain their specified blocking behavior.

Eviction, append, truncate, rotation and flush-repaint MUST invalidate or remap
views, line indexes, marks and highlight ranges according to the existing file
contract. A stale view MUST not point at reused allocation storage. Finite buffer
retention and unavailable backward data remain observable; copying entire streams
to simplify ownership is not an acceptable implementation.

## SAFE-MEM-007 Allocation failure and transaction boundaries

Input-sized growth while the terminal is active MUST use a fallible strategy,
including implicit formatting/conversion allocations. Reserve before publishing
a new buffer or state transition. A failed reservation leaves the previous state
valid and routes to specified recovery or controlled exit and cleanup.

Inventory infallible allocation sites, including dependency/native allocation,
and document their failure boundary. Catching a panic does not recover from
allocator abort or memory corruption. Inject allocation failure at each logical
growth/creation boundary; verify no double-release, partial filter installation,
incorrect history write or raw-terminal leak. Kernel OOM killing remains outside
the cleanup promise in SAFE-BASE-002.

## SAFE-MEM-008 Enforced safe application memory code

Completed application memory/parser modules MUST compile with
`#![forbid(unsafe_code)]` and without direct libc calls, raw ownership escape
hatches or generated C bitfield types. A helper that merely hides unchecked
pointer arithmetic behind a safe signature does not satisfy this requirement.

During migration, legacy adapters MUST be isolated, named in the unsafe inventory,
and bounded by documented borrowing/ownership contracts. They remain unfinished
safety work. A subsystem is not complete while untracked legacy aliases still
reference its storage; migration must remove those aliases in the same ownership
slice or retain an explicit failing final gate.
