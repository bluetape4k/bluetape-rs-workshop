# invitation-codecs

[English](README.md) | [한국어](README.ko.md)

요청 필드를 검증한 뒤 invitation token, callback state, support-facing
reference를 만드는 예제입니다.

## 시나리오

Invitation 흐름에는 서로 다른 세 가지 artifact가 필요합니다.

- 비밀값이 아닌 routing claim을 담는 URL-safe invitation token
- redirect 경계를 통과하는 URL-safe callback state
- raw recipient 식별자를 직접 드러내지 않는 support reference

예제는 `bluetape-rs-core`로 입력을 검증하고, `bluetape-rs-codec`으로
URL-safe Base64, Base58, UTF-8 text 경계를 처리합니다. 잘못된 encoded input은
panic이나 silent fallback 대신 타입이 있는 오류로 반환합니다.

## 대표 코드

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

## 볼 점

- `require_not_blank`로 encoding 전에 호출자 입력 오류를 걸러냅니다.
- URL-safe Base64 helper는 redirect artifact에 `+`, `/`, `=`가 들어가지
  않게 합니다.
- 외부 reference는 `INV-` prefix와 Base58을 사용하므로, support가 raw
  recipient 식별자를 보지 않고도 인용할 수 있습니다.
- Decode helper는 malformed input과 invalid shape을 타입 오류로 반환합니다.

## 실행

```bash
cargo test -p invitation-codecs
```
