# Milestone 0.2.0 구현 계획 검토

날짜: 2026-06-13
범위: `docs/superpowers/plans/2026-06-13-milestone-0-2-0-examples-plan.md`
기준 spec: `docs/superpowers/specs/2026-06-13-milestone-0-2-0-examples-design.md`

## 검토 방법

계획을 승인된 spec 및 현재 crate API 증거와 대조해 로컬에서 검토했다.
사용 가능한 subagent 도구 메타데이터가 명시적인 사용자 요청이 있는
경우로 생성을 제한하므로 native subagent 생성은 사용하지 않았다. 이
아티팩트는 필요한 독립 검토 경로를 로컬 동등 방식으로 유지한다.

형제 crate 비교:

- `examples/foundation-order-cleanup/src/lib.rs`는 규모가 더 작은 0.1.0
  crate에 맞는 간결한 예제 표면을 유지한다.
- `examples/request-tracing-log-capture/src/lib.rs`와
  `examples/temp-resource-test-harness/src/lib.rs`는 README 수준의 산문을
  코드에서 분리한다.
- 0.2.0 예제가 0.1.0 예제보다 크므로, 이제 계획은 모든 구현을 `lib.rs`에
  넣는 대신 `domain.rs`, `error.rs`, workflow 모듈, `tests.rs`를 요구한다.

## 발견 사항

| 경로 | 결과 | 증거 |
|---|---:|---|
| API 적합성 | P0=0, P1=0 | 계획은 로컬 Cargo 소스에서 이미 검사한 `bluetape-rs-core`, `bluetape-rs-logging`, `bluetape-rs-collections`, `bluetape-rs-async` 0.3.1 API를 사용한다. |
| TDD 형태 | P0=0, P1=0 | 각 crate에 명시적인 RED 후 GREEN 단계와 workspace 게이트 전 대상 패키지 테스트가 있다. |
| 비동기 수명 주기 | P0=0, P1=0 | fan-out은 비동기 제어 helper를 통한 timeout을 다루고, worker는 shutdown signal과 timeout을 타입이 지정된 상태로 다룬다. |
| 결정성 | P0=0, P1=0 | collection 그룹화는 `HashMap` 반복 순서의 변동을 피하기 위해 페이징 전에 정렬해야 한다. |
| 모듈 경계 | P0=0, P1=0 | 계획은 `lib.rs`를 짧은 Rustdoc/export 표면으로 유지하고 crate마다 domain, error, workflow, test 모듈을 분리한다. |
| 문서화 | P0=0, P1=0 | 루트와 crate README는 정확한 테스트 명령과 함께 영어와 한국어로 업데이트해야 한다. |
| 릴리스 workflow | P0=0, P1=0 | 계획은 dependency 업그레이드, 마일스톤 종료, merge를 범위에서 제외하며, PR 본문은 `## DoD Status`로 끝나야 한다. |

## 게이트 판정

PASS. 차단 항목: P0=0, P1=0.

이 계획을 그대로 사용해 Step 4 구현으로 진행한다.
