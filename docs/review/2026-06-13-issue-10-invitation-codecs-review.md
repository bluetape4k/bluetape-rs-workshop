# Issue 10 Invitation Codecs Review

## Scope

- Issue: https://github.com/bluetape4k/bluetape-rs-workshop/issues/10
- Branch: `feat/issue-10-invitation-codecs`
- Files: new `examples/invitation-codecs` crate, workspace registration, root README/WIP updates.

## Review Result

- P0: 0
- P1: 0
- Verdict: PASS

## Findings

No P0/P1 findings.

## Evidence

- TDD RED: `cargo test -p invitation-codecs` failed on missing API before implementation.
- TDD RED for delimiter boundary: `cargo test -p invitation-codecs rejects_reserved_delimiter_before_encoding` failed on missing `ReservedDelimiter`.
- Targeted verification: `cargo test -p invitation-codecs` passed with 6 tests.
- Full local gate: `make ci` passed.
- Whitespace gate: `git diff --check` passed.

## Notes

The implementation rejects blank fields and reserved `|` delimiters before
encoding so generated artifacts can round-trip through the strict decode
helpers. The external reference intentionally omits the raw recipient id and
uses an `INV-` prefixed Base58 payload.
