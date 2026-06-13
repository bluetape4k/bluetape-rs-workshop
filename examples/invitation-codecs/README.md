# invitation-codecs

[English](README.md) | [한국어](README.ko.md)

This example builds invitation tokens, callback state, and support-facing
references after validating request fields.

## Scenario

An invitation flow needs three different artifacts:

- a URL-safe invitation token that carries non-secret routing claims
- URL-safe callback state for the redirect boundary
- a support reference that avoids exposing the raw recipient identifier

The example uses `bluetape-rs-core` for input validation and
`bluetape-rs-codec` for URL-safe Base64, Base58, and UTF-8 text boundaries.
Malformed encoded input returns typed errors instead of panicking or silently
falling back.

## Representative Code

```rust
let package = build_invitation_codes(InvitationRequest::new(
    "north",
    "invite-42",
    "recipient-7",
    "/accept",
    "nonce-001",
    "SUP-1001",
))?;

assert!(!package.invitation_token.contains(['+', '/', '=']));
assert!(package.external_reference.starts_with("INV-"));
```

## What To Notice

- `require_not_blank` rejects caller input before encoding starts.
- URL-safe Base64 helpers keep redirect artifacts free of `+`, `/`, and `=`.
- The external reference is Base58 with an `INV-` prefix, so support can quote
  it without seeing the raw recipient identifier.
- Decode helpers return typed malformed-input or invalid-shape errors.

## Run

```bash
cargo test -p invitation-codecs
```
