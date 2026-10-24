# Decisions and remaining verification boundaries

There are no intentionally omitted production features in the Linux/GNU 712
specification. Source-inferred rules and observed cases are distinguished; the
case count is coverage evidence, not a proof that every possible sequence has
been exercised. The following decisions govern implementation and later claims.

| Topic | Decision | Consequence |
| --- | --- | --- |
| Regex semantics | GNU `RE_SYNTAX_POSIX_EXTENDED` on the frozen glibc profile; narrow target-library FFI allowed. | Rust ABI feasibility probes pass; a different backend needs conformance evidence. |
| Text | Byte-preserving input/offsets plus the pinned less classification/width tables. | Host Unicode defaults cannot silently replace the specified tables. |
| Terminal | Capability-driven behavior with frozen terminfo data; semantic cells, modes, bells and hyperlink associations in the observer. | Further physical-terminal extensions require observer/assertion work, not feature deletion. |
| Configuration | Ordered initialization and source-sensitive transitions, not a global merged settings map. | Security/early lookups precede later lesskey overrides. |
| Unusual errors | Preserve recorded continuation/status/stream behavior. | Unknown options and later missing files do not gain conventional fatal handling. |
| Platform expansion | macOS/BSD, Windows and historical systems are separate future profiles. | No claim of tested parity on those systems. |
| Performance | Semantic blocking/cancellation requirements are binding; throughput/RSS claims need their own measurements. | The specification makes no numerical speed or memory-parity claim. |
| Candidate identity | Exact exception in scope; strict reference replay currently keeps original less messages. | Add an explicit field-level identity adapter before comparing rless identity banners. |

Implementation verification still has work: run the suite against rless, expand
interaction/state-sequence coverage as discrepancies are found, port additional
upstream replay scenarios to the normal PTY interface where useful, fuzz malformed
input and test real pager callers. These are implementation/release checks, not
permission to reinterpret the written contracts.

The observer covers the captured terminal operations. It is not a general proof
of every physical terminal's behavior; a passing screenshot/trace must not be
used to dismiss an observed unsupported-mode discrepancy. File snapshots cover
fixture effects; external effects require the explicit controlled helpers used
by integration cases. Full parity claims await an actual Rust implementation.
