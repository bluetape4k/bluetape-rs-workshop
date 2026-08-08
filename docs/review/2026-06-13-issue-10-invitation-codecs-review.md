# Issue 10 Invitation Codecs 검토

## 범위

- 이슈: https://github.com/bluetape4k/bluetape-rs-workshop/issues/10
- 브랜치: `feat/issue-10-invitation-codecs`
- 파일: 새 `examples/invitation-codecs` crate, workspace 등록, 루트 README/WIP 업데이트.

## 검토 결과

- P0: 0
- P1: 0
- 판정: PASS

## 발견 사항

P0/P1 발견 사항 없음.

## 증거

- TDD RED: 구현 전에 API가 없어 `cargo test -p invitation-codecs`가 실패했다.
- 구분자 경계에 대한 TDD RED: `ReservedDelimiter`가 없어
  `cargo test -p invitation-codecs rejects_reserved_delimiter_before_encoding`가 실패했다.
- 대상 검증: `cargo test -p invitation-codecs`가 6개 테스트와 함께 통과했다.
- 전체 로컬 게이트: `make ci`가 통과했다.
- 공백 게이트: `git diff --check`가 통과했다.

## 참고

구현은 인코딩 전에 빈 필드와 예약된 `|` 구분자를 거부하므로, 생성된
산출물이 엄격한 decode helper를 통해 왕복 변환될 수 있다. 외부 참조는
원시 recipient id를 의도적으로 생략하고 `INV-` 접두사가 붙은 Base58
payload를 사용한다.
