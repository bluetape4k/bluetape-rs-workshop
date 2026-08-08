# Milestone 0.2.0 예제 설계

## 배경

`bluetape-rs-workshop`는 실행 가능한 Rust 예제로 `bluetape-rs`를 학습한다.
이 저장소는 `main`을 유일한 장기 유지 브랜치로 사용하며, 기능 작업은 pull
request를 통해 `main`을 대상으로 한다.

`0.1.0` 마일스톤은 다음 세 개의 독립적인 기반 예제를 제공했다.

- `foundation-order-cleanup`: 검증, 정규화, 타입이 지정된 오류, 수집한 로그.
- `request-tracing-log-capture`: correlation-aware 요청 로그.
- `temp-resource-test-harness`: 격리된 임시 filesystem 테스트.

`0.2.0` 마일스톤은 `bluetape4k/bluetape-rs-workshop`의 epic #5와 하위 이슈
#6, #7, #8로 추적한다. 이 spec은 workshop repository의 이슈를 기준 데이터
원본으로 사용한다. 이름이 비슷한 이전 `bluetape-rs` repository 이슈는 이
구현의 대상이 아니다.

## 목표

`0.2.0` collections 및 async 경로를 세 개의 실행 가능한 예제 crate로
추가한다. 이 예제는 `0.1.0`의 검증, 로깅, 테스트 기준선을 작은 백엔드
workflow로 조합한다.

## 현재 증거

- 현재 브랜치 기준선: 구현 전에 `feat/milestone-0.2.0-examples`에서
  `make ci`가 통과했다.
- 열린 마일스톤 이슈:
  - #5: `[Epic] 0.2.0 collections and async examples`
  - #6: `feat: add catalog enrichment fanout workshop example`
  - #7: `feat: add batched order windowing workshop example`
  - #8: `feat: add shutdown aware worker workshop example`
- 현재 workspace dependency에는 `bluetape-rs-core`, `bluetape-rs-logging`,
  `bluetape-rs-test`, `tokio`, `thiserror`, `tracing`이 포함되어 있다.
- 로컬 Cargo 소스에서 `bluetape-rs-collections 0.3.1`이
  `iter::{chunks, group_by, partition_results}`, `Page`, `PageError`를
  export하는 것을 확인했다.
- 로컬 Cargo 소스에서 `bluetape-rs-async 0.3.1`이 `try_map_bounded`,
  `map_bounded_collect`, `with_timeout`, `with_timeout_or_cancel`,
  `run_until_cancelled`, `CancellationSource`, `shutdown_signal`을
  export하는 것을 확인했다.

## 선택한 접근

하나의 PR에서 세 개의 독립적인 예제 crate를 사용한다.

1. `examples/batched-order-windowing`
   - collections-first workflow.
   - 파트너 주문 이벤트를 검증하고 tenant/channel별로 그룹화한 뒤 결정적인
     batch로 청크화하고 page 메타데이터를 반환한다.
2. `examples/catalog-enrichment-fanout`
   - collections와 제한된 async fan-out을 결합한 workflow.
   - catalog 행과 요청 메타데이터를 검증하고 행을 그룹화하며 출력을
     페이징한 뒤 필수 및 선택 provider로 catalog item을 보강한다.
3. `examples/shutdown-aware-worker`
   - async lifecycle workflow.
   - worker 설정을 검증하고 작은 collection 변환을 수행하며 진행 상황을
     로깅하고 shutdown, timeout, 취소를 준수한다.

루트 README와 Korean README에는 실행 명령과 학습 경로를 포함한 `0.2.0`
섹션을 추가한다. `WIP.md`에는 상세한 `0.2.0 Delivery Scope` 표를 추가한다.
기존 다이어그램 generator는 현재 Graphviz 및 geometry 증거 패턴을 대규모
재작성 없이 보존할 수 있는 경우에만 확장할 수 있다.

## 검토한 대안

### 대안 A: 통합 `collections-async-workflow` crate 하나

이 방식은 통합을 잘 보여주지만 학습 표면을 넓히고 독자가 collection 동작과
async lifecycle 동작을 분리해 이해하기 어렵게 만든다. workshop 패턴이
예제별 crate이므로 거부했다.

### 대안 B: 독립적인 PR 세 개

이 방식은 PR별 검토 범위를 줄이지만 루트 README와 마일스톤 다이어그램
작업을 반복하게 하고 `0.2.0` 경로를 하나의 일관된 단계로 검증하기
어렵게 만든다. 사용자가 마일스톤 단위 예제 작업을 요청했으므로 거부했다.

### 대안 C: 하나의 PR에 독립적인 crate 세 개

선택했다. 작은 예제를 유지하면서 마일스톤 단위의 README, 검토, CI, DoD
게이트를 하나로 제공한다.

## 예제 계약

### `batched-order-windowing`

입력:

- `correlation_id`, `page_number`, `page_size`, raw `OrderEvent` 행을 포함한
  `BatchRun`.
- `OrderEvent`에는 `tenant`, `channel`, `order_id`, `sku`, `quantity`가
  포함된다.

동작:

- 필수 필드가 비어 있거나 quantity가 양수가 아니면 거부한다.
- `bluetape-rs-collections`를 사용해 검증된 행을 `(tenant, channel)`별로
  그룹화한다.
- 요청한 page size에 따라 각 그룹을 청크화한다.
- tenant, channel, chunk index로 정렬한 결정적인 `OrderBatch` 값 목록을
  반환한다.
- 제공된 page 메타데이터와 함께 `Page<OrderBatch>`를 반환한다.
- correlation-aware 요약 로그를 출력한다.

테스트:

- 정상 그룹화/청크화/페이징.
- 빈 입력은 빈 page를 반환한다.
- 잘못된 page size는 실패한다.
- 잘못된 행은 타입이 지정된 검증 오류와 함께 실패한다.

### `catalog-enrichment-fanout`

입력:

- `correlation_id`, `page_number`, `page_size`, `max_concurrency`,
  `CatalogRow` item을 포함한 `CatalogRequest`.
- provider fixture는 network call이 아닌 메모리 내 provider response로
  표현한다.

동작:

- 요청 메타데이터와 product 행을 검증한다.
- product를 category별로 그룹화하고 product view를 페이징한다.
- 명시적인 동시성 제한을 둔 provider fan-out에는 `try_map_bounded` 또는
  `map_bounded_collect`를 사용한다.
- 필수 provider 실패는 요청을 실패시킨다.
- 선택 provider 실패는 경고로 수집하며 요청을 실패시키지 않는다.
- timeout 또는 취소는 호출자가 확인할 수 있는 타입이 지정된 오류를
  반환한다.
- enrichment 실행에 대한 correlation-aware 로그를 출력한다.

테스트:

- 그룹화·페이징·보강된 출력의 성공.
- 비어 있는 요청 메타데이터 또는 product 필드는 실패한다.
- 필수 provider 실패는 실패한다.
- 선택 provider 실패는 기록하지만 출력은 계속 사용할 수 있다.
- timeout 또는 취소 동작은 명시적이고 결정적이다.

### `shutdown-aware-worker`

입력:

- `correlation_id`, `max_concurrency`, `deadline`, work item을 포함한
  `WorkerConfig`.
- queue, key, cost 필드를 포함한 `WorkerItem`.

동작:

- 설정과 work item을 검증한다.
- dispatch 전에 작업을 그룹화하거나 청크화한다.
- shutdown-aware lifecycle을 모델링하기 위해 `shutdown_signal`,
  `with_timeout_or_cancel`, `run_until_cancelled` 중 하나를 사용한다.
- 작업을 async 및 non-blocking 상태로 유지한다. core async task에서는
  blocking 작업을 실행하지 않는다.
- 처리 개수와 shutdown/timeout 상태를 포함한 `WorkerReport`를 반환한다.
- correlation-aware 진행 로그를 출력한다.

테스트:

- worker가 정상 작업을 완료한다.
- 잘못된 설정은 실패한다.
- timeout은 타입이 지정된 timeout 상태로 실패한다.
- shutdown/취소는 타입이 지정된 cancelled 상태를 반환한다.

## 위험 및 실패 모드

1. Async task가 누수되거나 caller shutdown을 무시할 수 있다.
   - 완화: `bluetape-rs-async` helper와 timeout/취소를 위한 결정적인 Tokio
     테스트를 사용한다.
2. Provider fan-out이 필수 provider 실패를 숨길 수 있다.
   - 완화: 필수 provider와 선택 provider를 분리해 모델링하고 두 실패
     모드를 모두 검증한다.
3. Collection helper가 `HashMap`을 통해 비결정적인 순서를 만들 수 있다.
   - 완화: 그룹화된 출력을 반환하기 전에 정렬한다.
4. 예제가 백엔드 workflow가 아니라 일반적인 snippet이 될 수 있다.
   - 완화: 모든 예제는 검증으로 시작하고 현실적인 order 또는 catalog
     데이터를 사용하며 호출자가 확인할 수 있는 report로 끝낸다.
5. README 주장이 source name과 어긋날 수 있다.
   - 완화: 루트와 예제 README 명령을 `Cargo.toml` package name과 대조하고
     대상 테스트를 실행해 검증한다.

## 승인 기준

- `Cargo.toml`이 새 예제 crate 세 개를 등록한다.
- 루트 `README.md`와 `README.ko.md`가 모든 `0.2.0` 예제에 링크하고 각
  예제의 명령을 포함한다.
- `WIP.md`에 `0.2.0 Delivery Scope` 표가 있다.
- 각 예제에 `README.md`, `README.ko.md`, `Cargo.toml`, `src/lib.rs`가 있다.
- 각 예제가 `thiserror`를 사용한 타입이 지정된 Rust 오류를 사용한다.
- 호출자 대상 API에 해당하는 public struct/function에 영어 Rustdoc이 있다.
- `batched-order-windowing`이 검증, 그룹화, 청크화, 페이징, 경계 테스트를
  다룬다.
- `catalog-enrichment-fanout`이 검증, 그룹화, 페이징, 제한된 fan-out,
  필수 실패, 선택 실패, timeout 또는 취소를 다룬다.
- `shutdown-aware-worker`가 성공, 입력 검증, timeout,
  shutdown/취소를 다룬다.
- 로컬 검증이 통과한다.
  - `cargo fmt --all --check`
  - `cargo test -p batched-order-windowing`
  - `cargo test -p catalog-enrichment-fanout`
  - `cargo test -p shutdown-aware-worker`
  - `cargo test --workspace --all-features`
  - `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  - `git diff --check`

## 범위 제외

- `bluetape-rs-*` dependency를 `0.3.1`에서 `0.4.0`으로 업그레이드.
- 기존 test helper를 넘어서는 실제 network, database, filesystem 통합.
- 마일스톤 `0.2.0`의 release 또는 종료. 이 PR은 예제만 제공한다.
- benchmark harness 추가.
- 새 workspace member가 기존 `make ci` 동작으로 커버되지 않는 경우를
  제외한 GitHub Actions 변경.

## DoD

- 구현 전에 spec과 구현 계획을 커밋한다.
- 새 동작에 대한 테스트를 먼저 작성하고 해당 구현을 추가하기 전에
  실패하는 것을 확인한다.
- Step 2-R, Step 3-R, Step 6-R, PR 검토 게이트가 P0=0 및 P1=0으로
  수렴한다.
- PR 생성 전에 lessons를 커밋한다.
- PR 본문은 `## DoD Status`로 끝난다.
