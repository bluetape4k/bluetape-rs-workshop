# Milestone 0.2.0 예제 교훈

날짜: 2026-06-13
브랜치: `feat/milestone-0.2.0-examples`

## 변경 사항

- 결정적인 그룹화, 청크화, `Page` 메타데이터를 다루는
  `batched-order-windowing`을 추가했다.
- 필수 provider 실패, 선택 provider 경고, timeout 제어를 포함한 제한된 비동기
  provider 작업을 다루는 `catalog-enrichment-fanout`을 추가했다.
- 타입이 지정된 timeout 및 shutdown 취소 오류와 함께 그룹화된 워커 처리를
  다루는 `shutdown-aware-worker`를 추가했다.
- 기존 `0.3.1` workshop dependency lane을 업그레이드하지 않고
  `bluetape-rs-collections = "0.3.1"`과 `bluetape-rs-async = "0.3.1"`을
  등록했다.

## 수정한 내용

첫 구현 계획 초안은 각각의 새 예제가 대부분 `src/lib.rs` 안에 들어갈 수
있다고 암시했다. 구현 전에 이를 수정했다. 최종 crate는 `lib.rs`를 짧은
Rustdoc/export 표면으로 유지하고 코드를 domain, error, workflow, test
모듈로 분리한다.

## 검증 증거

- RED 대상 테스트가 의도한 `todo!()` 구현 지점에서 실패했다.
- `cargo test -p batched-order-windowing`: PASS, 4개 테스트.
- `cargo test -p catalog-enrichment-fanout`: PASS, 5개 테스트.
- `cargo test -p shutdown-aware-worker`: PASS, 4개 테스트.
- `cargo fmt --all --check`: PASS.
- `cargo test --workspace --all-features`: PASS.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`: PASS.
- `make ci`: PASS.
- `git diff --check`: PASS.
- Step 6-R 코드 검토: PASS, `P0=0 P1=0`.

## 후속 적용 사항

- 이후 Rust workshop 예제는 테스트를 작성하기 전에 모듈 경계를 계획에
  먼저 명시한다.
- `HashMap` 기반 그룹화는 명시적으로 정렬하기 전까지 비결정적으로 취급한다.
- 이슈에서 통합 테스트와 service fixture를 명시적으로 요청하지 않는 한 외부
  provider/network 동작을 workshop 예제에서 제외한다.
