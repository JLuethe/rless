# Isolated M2 preparation review

Date: 2026-10-25. Reviewer: Codex implementation author. Independent review remains
pending. Scope: the isolated `rless-safe-core` component crate, its source files
and component-only callers/tests. No production caller migration or M2 closure.

## Ownership and reachability

`tools/safety/core/Cargo.toml` compiles `src/safe_core/mod.rs` separately. Neither
`src/lib.rs`, pager main nor lessecho imports it. The crate depends only on the
Rust standard library, forbids unsafe code and exposes no raw pointer/descriptor
or native entry point. Production still uses the legacy xbuffer and state paths.
The original M0/M1 prerequisites and required independent approvals are unchanged.

The owner/ABI decision for this preparation is Rust-owned allocations released
by their originating Vec, numeric value types and compile-time branded owners.
There is no libc allocation adoption, native layout, signal handler, native
callback registration, runtime identity registry or secondary pager-state owner.
Standard-library/compiler internals remain trusted; ASan uses prebuilt std, while
Miri interprets its pinned-source sysroot. This is not a native-library audit.

## ByteBuf and reserve authority

Creation uses an empty Vec or a fallible copy from a slice. ByteBuf is its only
allocation/deallocation authority and has no Clone or mutable/raw storage escape.
`as_bytes` lends an ordinary slice bounded by `&self`; mutation needs `&mut self`.
Rust's borrow checker rejects retaining the slice across mutation. Source aliases
for self-copy are represented by a validated original range, then reacquired after
reservation. Invalid/reversed/end-outside ranges fail before allocation or commit.

The reserve helper checks logical addition and element-byte multiplication,
including the isize allocation domain, before calling Vec::try_reserve. A checked
capacity guarantee precedes extend/push operations; their internal ordinary Vec
reserve cannot allocate because sufficient capacity already exists. Replacement
copies into a separate staged Vec before assigning the new owner. Failed reserve
leaves existing bytes, length and owner intact. Reset retains capacity; truncate
rejects increasing length; pop on empty returns None. All bytes remain arbitrary,
including NUL and invalid UTF-8.

NulTerminatedBytes is a separate owning copy, constructed only after checking for
interior NUL and reserving its terminator fallibly. Its private representation
always has at least the final zero byte. Borrowed access cannot mutate/remove the
terminator; destroying the original ByteBuf does not invalidate the copy. No
infallible String/CString conversion is hidden in the component API.

Component fault plans have checked positive ordinals and count only actual
reserves. They reject before native allocation and allocate nothing themselves.
A failure hit is distinct from a request satisfied by existing capacity. They
are compiled only for tests/the isolated fault-testing feature and do not modify
the pager allocator or its production-path hook contract.

Failure paths and valid counterparts are exercised by units, the bounded action
oracle, Miri and ASan. The action oracle uses a separate Vec model and checks
unchanged state on each actual injected allocation failure, then subsequent valid
operations. Large/integer limits have separate unit cases. Preparation fuzz caps
are harness bounds, not product limits or proof of complete parser behavior.

## Coordinates

SourceOffset stores only nonnegative signed-64-bit offsets. Construction rejects
sentinels/out-of-domain values, addition is checked, and distance is computed only
after ordering validation. Values beyond 4 GiB remain representable. Narrow GNU
regex integer conversion is an explicit checked operation; its rejection does not
cap the general file-offset domain. BufferIndex, LogicalLine, DisplayRow and
DisplayColumn remain distinct types. Logical lines are one-based; unavailable
positions belong in an outer state, not a fabricated index.

RetainedRange describes initialized source bytes and distinguishes a valid data
index from an end boundary. It does not own a file or certify freshness after an
input lifecycle change. Future input owners must revalidate it after rotation,
truncation or eviction. These value types alone do not solve legacy stale views.

## Owner-bound generational arena

`with_arena` is the only constructor. Its higher-ranked closure receives a fresh,
invariant lifetime brand; both Arena and Handle carry it. Handles/arenas cannot
escape the closure's result or be mixed with a separately created owner's brand.
An Rc marker makes them main-thread confined without allocating an identity token.
The safe counterpart and diagnostic-specific compiler controls check same-owner
lookup, cross-owner rejection, scope escape and thread transfer rejection.

Slots own Option<T>. Insertion reserves before publishing a new slot and returns
the original uncommitted T on allocation failure. Reuse/removal uses an intrusive
free list and allocates nothing. Get/get_mut return borrows tied to the current
arena borrow. Removing a value immediately advances its generation; old handles
cannot access a later slot occupant. At u64::MAX the slot is permanently retired,
never wrapped into a prior identity. Tests place a slot near that boundary rather
than claiming to execute 2^64 removals. Removed values transfer to the caller;
remaining values are released by normal Vec/Option destruction. Drop-count tests
check failure/retry/removal/destruction do not duplicate or prematurely drop T.

The safe Rust FnOnce supplied to with_arena and generic T's Drop/Debug bodies
remain caller code. They can panic; this component supplies no native frame,
terminal guard or promise to recover from arbitrary caller panics. Future pager
integration must review the actual closure/state lifetime and shutdown behavior.
No unsafe Send/Sync, leaked lifetime, global counter or native callback pointer
is used to establish identity or bypass borrowing.

## Integration work still required

- Complete the production xbuffer/caller review, including editing, parsers,
  command-table transfers, expansion, self-copy and retained native views.
- Preserve CLI/lesskey/environment precedence, exact byte semantics and explicit
  controlled failures while replacing an entire owner/alias slice.
- Design the actual PagerState event-loop scope around branded identities; the
  compile-only positive example is not a migrated application state graph.
- Bind coordinate conversions to the actual native/file/layout callers. Run real
  sparse-file and streaming/invalidation tests at integration.
- Preserve M0/M1 gates, native-boundary review, final variant/profile evidence and
  the original 15-target/15-CPU-hour minimum final campaign floor. This component
  work receives zero production fuzz-budget credit.

All reviewed source hashes and evidence references belong in the preparation
checkpoint/register. Earlier source-bound reviews remain historical; a new safe
component does not promote any unreviewed legacy owner or a whole acceptance row.
