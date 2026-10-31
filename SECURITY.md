# Security policy

Rless is experimental and contains substantial unsafe translated code. No
memory-safety milestone is verified. There is currently no stable release series
or promised security-support period; fixes are developed against the current
candidate. The [safety migration status](docs/current-state.md) and
[findings register](docs/safety/findings.json) document known work.

For a suspected vulnerability, use the repository's
[private vulnerability reporting page](https://github.com/JLuethe/rless/security/advisories/new)
if GitHub makes it available. If private reporting is unavailable, open a public
issue asking the maintainer for a private reporting channel, without attaching
exploit details, sensitive input or crash dumps. No response-time commitment is
currently offered.

Include the commit or release hash, Linux/glibc version, build flags, minimal
sanitized reproducer and expected versus observed behavior. Explain whether the
input can cause memory corruption, data loss, a hang or terminal-state damage.
Retain diagnostic logs for the private report and remove credentials and private
file contents. Ordinary compatibility bugs can be filed as public issues.
