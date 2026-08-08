# Milestone 0.2.0 예제 구현 계획

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**목표:** collections, 제한된 async fan-out, shutdown-aware worker를 위한 실행 가능한 Rust 예제 crate 세 개로 `0.2.0` workshop 경로를 구축한다.

**아키텍처:** 예제마다 crate 하나를 유지한다. 각 crate는 `src/lib.rs`에서
짧은 Rustdoc과 public re-export만 노출하고, domain type, 타입이 지정된
오류, workflow 로직, 테스트는 목적별 모듈에 둔다. 각 crate는
`bluetape-rs-core`로 caller 입력을 검증하고 `thiserror`로 타입이 지정된
오류를 사용하며, 자세한 학습 설명은 이중 언어 README 파일에 둔다. 루트
문서는 마일스톤 경로를 설명하고 예제에 링크한다.

**기술 스택:** Rust 2024, `bluetape-rs-core 0.3.1`, `bluetape-rs-logging 0.3.1`, `bluetape-rs-collections 0.3.1`, `bluetape-rs-async 0.3.1`, `tokio`, `thiserror`, `tracing`.

---

## 파일 구조

- `Cargo.toml` 수정: workspace member 세 개와 `bluetape-rs-collections`,
  `bluetape-rs-async`용 workspace dependency를 추가한다. async 테스트에
  필요하면 누락된 Tokio feature `rt-multi-thread`, `sync`도 추가한다.
- `README.md`, `README.ko.md` 수정: `0.2.0` 마일스톤 예제, 학습 경로,
  아키텍처/sequence 설명, 레이아웃 항목을 추가한다.
- `WIP.md` 수정: `0.2.0 Delivery Scope`를 추가한다.
- `examples/batched-order-windowing/{Cargo.toml,README.md,README.ko.md,src/{lib.rs,domain.rs,error.rs,windowing.rs,tests.rs}}` 생성.
- `examples/catalog-enrichment-fanout/{Cargo.toml,README.md,README.ko.md,src/{lib.rs,domain.rs,error.rs,enrichment.rs,tests.rs}}` 생성.
- `examples/shutdown-aware-worker/{Cargo.toml,README.md,README.ko.md,src/{lib.rs,domain.rs,error.rs,worker.rs,tests.rs}}` 생성.
- `docs/lessons/2026-06-13-milestone-0-2-0-examples.md` 생성.
- `docs/review/2026-06-13-milestone-0-2-0-code-review.md` 생성.

## Task 1: Workspace 등록

complexity: low

**파일:**
- 수정: `Cargo.toml`

- [ ] **Step 1: workspace member와 dependency 추가**

다음 workspace member를 추가한다.

```toml
    "examples/batched-order-windowing",
    "examples/catalog-enrichment-fanout",
    "examples/shutdown-aware-worker",
```

다음 workspace dependency를 추가한다.

```toml
bluetape-rs-async = "0.3.1"
bluetape-rs-collections = "0.3.1"
```

컴파일에 필요하면 Tokio feature를 다음과 같이 변경한다.

```toml
tokio = { version = "1.48.0", default-features = false, features = ["macros", "rt", "rt-multi-thread", "sync", "time", "test-util"] }
```

- [ ] **Step 2: 기존 예제가 계속 resolve되는지 검증**

실행:

```bash
cargo metadata --no-deps --format-version 1
```

예상: Task 2-4 crate가 생성된 뒤 여섯 package name이 모두 나타난다. 그
전에는 crate skeleton을 추가한 뒤 실행한다.

## Task 2: `batched-order-windowing`

complexity: medium

`$bluetape-rs-patterns`를 적용한다. Rust-native 타입 오류, `HashMap`
그룹화 후 결정적인 순서, caller 입력에 대한 패닉 없음, public API의 영어
Rustdoc을 사용한다.

**파일:**
- 생성: `examples/batched-order-windowing/Cargo.toml`
- 생성: `examples/batched-order-windowing/README.md`
- 생성: `examples/batched-order-windowing/README.ko.md`
- 생성: `examples/batched-order-windowing/src/lib.rs`
- 생성: `examples/batched-order-windowing/src/domain.rs`
- 생성: `examples/batched-order-windowing/src/error.rs`
- 생성: `examples/batched-order-windowing/src/windowing.rs`
- 생성: `examples/batched-order-windowing/src/tests.rs`

- [ ] **Step 1: 테스트를 먼저 작성**

짧은 crate Rustdoc과 public export만 포함하는 `src/lib.rs`를 생성한다.
public type은 `domain.rs`, public error는 `error.rs`, workflow 로직은
`windowing.rs`, 테스트는 `tests.rs`에 둔다. 아직 구현이 없으므로 첫 번째
테스트는 실패해야 한다.

```rust
#[test]
fn groups_chunks_and_pages_partner_events() {
    let page = build_order_batches(BatchRun {
        correlation_id: "corr-batch-001".to_owned(),
        page_number: 0,
        page_size: 2,
        events: vec![
            OrderEvent::new("north", "web", "ord-1", "sku-1", 2),
            OrderEvent::new("north", "web", "ord-2", "sku-2", 1),
            OrderEvent::new("north", "store", "ord-3", "sku-3", 4),
        ],
    })
    .expect("batch page should build");

    assert_eq!(page.total_items(), 2);
    assert_eq!(page.items()[0].tenant, "north");
    assert_eq!(page.items()[0].channel, "store");
    assert_eq!(page.items()[0].orders.len(), 1);
    assert_eq!(page.items()[1].channel, "web");
    assert_eq!(page.items()[1].orders.len(), 2);
}
```

빈 입력, 잘못된 page size, 빈 tenant에 대한 테스트도 추가한다.

- [ ] **Step 2: RED 테스트 실행**

실행:

```bash
cargo test -p batched-order-windowing
```

예상: crate 또는 구현이 불완전하므로 실패한다.

- [ ] **Step 3: 최소 동작 구현**

사용:

- `require_not_blank`
- `require_positive`
- `CorrelationId`
- `bluetape_rs_collections::iter::group_by`
- `bluetape_rs_collections::iter::chunks`
- `bluetape_rs_collections::Page`
- `tracing::info!`

구현은 페이징 전에 그룹화된 batch를 `(tenant, channel)`로 정렬해야 한다.

- [ ] **Step 4: GREEN 테스트 실행**

실행:

```bash
cargo test -p batched-order-windowing
```

예상: 모든 `batched-order-windowing` 테스트가 통과한다.

- [ ] **Step 5: 이중 언어 README 추가**

README 파일에는 시나리오, 대표 코드, 주의할 점과 다음 명령을 포함해야 한다.

```bash
cargo test -p batched-order-windowing
```

## Task 3: `catalog-enrichment-fanout`

complexity: high

`$bluetape-rs-patterns`를 적용한다. 제한된 Tokio task, 타입이 지정된
provider 실패, 취소/timeout 증거를 사용하며 필수 provider 실패를 숨기지
않는다.

**파일:**
- 생성: `examples/catalog-enrichment-fanout/Cargo.toml`
- 생성: `examples/catalog-enrichment-fanout/README.md`
- 생성: `examples/catalog-enrichment-fanout/README.ko.md`
- 생성: `examples/catalog-enrichment-fanout/src/lib.rs`
- 생성: `examples/catalog-enrichment-fanout/src/domain.rs`
- 생성: `examples/catalog-enrichment-fanout/src/error.rs`
- 생성: `examples/catalog-enrichment-fanout/src/enrichment.rs`
- 생성: `examples/catalog-enrichment-fanout/src/tests.rs`

- [ ] **Step 1: 테스트를 먼저 작성**

테스트는 다음을 다뤄야 한다.

- 그룹화·페이징·보강된 출력의 성공
- 비어 있는 요청 메타데이터
- 필수 provider 실패
- 경고로 기록되는 선택 provider 실패
- timeout 또는 취소

timeout 범위를 검증할 때는 짧고 결정적인 sleep을 사용한다. workspace Tokio
feature가 필요하게 만드는 경우가 아니라면 paused-time 테스트는 피한다.
`lib.rs`에는 crate Rustdoc, 모듈 선언, re-export만 둔다.

- [ ] **Step 2: RED 테스트 실행**

실행:

```bash
cargo test -p catalog-enrichment-fanout
```

예상: 구현이 완료되기 전에는 실패한다.

- [ ] **Step 3: 제한된 fan-out 구현**

사용:

- `require_not_blank`, `require_positive`
- `CorrelationId`
- `bluetape_rs_collections::{iter, Page}`
- `bluetape_rs_async::{map_bounded_collect, with_timeout_or_cancel, CancellationSource}`
- `tracing::info!`와 `tracing::warn!`

Provider fixture는 메모리에 유지한다. 필수 provider 오류는 요청을
실패시킨다. 선택 provider 오류는 반환할 view에 warning 문자열을 추가한다.

- [ ] **Step 4: GREEN 테스트 실행**

실행:

```bash
cargo test -p catalog-enrichment-fanout
```

예상: 모든 테스트가 통과한다.

- [ ] **Step 5: 이중 언어 README 추가**

README 파일은 필수 provider와 선택 provider의 계약을 설명하고 다음 명령을
포함해야 한다.

```bash
cargo test -p catalog-enrichment-fanout
```

## Task 4: `shutdown-aware-worker`

complexity: high

`$bluetape-rs-patterns`를 적용한다. 명시적인 async lifecycle, timeout 및
shutdown 테스트를 사용하고 async task에서 blocking 작업을 실행하지 않는다.

**파일:**
- 생성: `examples/shutdown-aware-worker/Cargo.toml`
- 생성: `examples/shutdown-aware-worker/README.md`
- 생성: `examples/shutdown-aware-worker/README.ko.md`
- 생성: `examples/shutdown-aware-worker/src/lib.rs`
- 생성: `examples/shutdown-aware-worker/src/domain.rs`
- 생성: `examples/shutdown-aware-worker/src/error.rs`
- 생성: `examples/shutdown-aware-worker/src/worker.rs`
- 생성: `examples/shutdown-aware-worker/src/tests.rs`

- [ ] **Step 1: 테스트를 먼저 작성**

테스트는 다음을 다뤄야 한다.

- worker가 정상 작업을 완료한다.
- 잘못된 설정은 실패한다.
- timeout은 타입이 지정된 timeout을 반환한다.
- shutdown/취소는 타입이 지정된 cancelled 상태를 반환한다.

`lib.rs`에는 crate Rustdoc, 모듈 선언, re-export만 둔다.

- [ ] **Step 2: RED 테스트 실행**

실행:

```bash
cargo test -p shutdown-aware-worker
```

예상: 구현이 완료되기 전에는 실패한다.

- [ ] **Step 3: worker 구현**

사용:

- `require_not_blank`, `require_positive`
- `CorrelationId`
- `bluetape_rs_collections::iter::group_by`
- `bluetape_rs_async::{shutdown_signal, with_timeout, AsyncControlError}`
- 결정적인 simulated work를 위한 `tokio::time::sleep`
- `tracing::info!`

처리 개수와 최종 상태를 포함한 `WorkerReport`를 반환한다.

- [ ] **Step 4: GREEN 테스트 실행**

실행:

```bash
cargo test -p shutdown-aware-worker
```

예상: 모든 테스트가 통과한다.

- [ ] **Step 5: 이중 언어 README 추가**

README 파일은 취소 계약을 설명하고 다음 명령을 포함해야 한다.

```bash
cargo test -p shutdown-aware-worker
```

## Task 5: 루트 문서

complexity: medium

**파일:**
- 수정: `README.md`
- 수정: `README.ko.md`
- 수정: `WIP.md`

- [ ] **Step 1: 루트 README 예제 표 업데이트**

세 새 예제와 정확한 package 테스트 명령을 행으로 포함하는
`Milestone 0.2.0: Collections and Async Examples` 섹션을 추가한다.

- [ ] **Step 2: Korean README 업데이트**

영어 내용을 한국어 설명과 동일한 명령으로 미러링한다.

- [ ] **Step 3: `WIP.md` 업데이트**

다음 `0.2.0 Delivery Scope` 표를 추가한다.

| Issue | Example | bluetape-rs APIs | Done When |
|---|---|---|---|
| #6 | `catalog-enrichment-fanout` | `core`, `logging`, `collections`, `async` | bounded provider fan-out with required/optional failure behavior |
| #7 | `batched-order-windowing` | `core`, `logging`, `collections` | deterministic grouping, chunking, paging |
| #8 | `shutdown-aware-worker` | `core`, `logging`, `collections`, `async` | bounded shutdown, timeout, cancellation |

- [ ] **Step 4: 문서를 source와 대조해 검증**

실행:

```bash
rg -n "batched-order-windowing|catalog-enrichment-fanout|shutdown-aware-worker" README.md README.ko.md WIP.md Cargo.toml examples
```

예상: package name과 명령이 일관되게 나타난다.

## Task 6: 검증

complexity: medium

**파일:** 변경한 모든 파일.

- [ ] **Step 1: 포맷 검사 실행**

실행:

```bash
cargo fmt --all --check
```

예상: exit 0.

- [ ] **Step 2: 대상 테스트 실행**

실행:

```bash
cargo test -p batched-order-windowing
cargo test -p catalog-enrichment-fanout
cargo test -p shutdown-aware-worker
```

예상: 모두 exit 0.

- [ ] **Step 3: workspace 테스트와 clippy 실행**

실행:

```bash
cargo test --workspace --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

예상: 두 명령 모두 exit 0.

- [ ] **Step 4: repository 게이트 실행**

실행:

```bash
make ci
git diff --check
```

예상: 두 명령 모두 exit 0.

## Task 7: 검토, lessons, PR

complexity: medium

**파일:**
- 생성: `docs/review/2026-06-13-milestone-0-2-0-code-review.md`
- 생성: `docs/lessons/2026-06-13-milestone-0-2-0-examples.md`

- [ ] **Step 1: Step 6-R 코드 검토**

최종 diff에 대해 local/native six-lane 검토를 실행한다. P0/P1/P2/P3을
`docs/review/2026-06-13-milestone-0-2-0-code-review.md`에 기록한다.

- [ ] **Step 2: PR 전에 lessons 커밋**

변경 사항, 예상 밖의 결과, 검증 증거를 포함한
`docs/lessons/2026-06-13-milestone-0-2-0-examples.md`를 생성한다. PR 생성
전에 커밋한다.

- [ ] **Step 3: PR 생성**

브랜치를 push하고 다음 제목으로 PR을 생성한다.

```text
feat: add milestone 0.2.0 collections and async examples
```

PR 본문에는 `Closes #6`, `Closes #7`, `Closes #8`을 포함하고 #5를
참조하며 `## DoD Status`로 끝내야 한다.

## 자체 검토

- Spec coverage: 모든 승인 기준이 Task 1-7에 매핑된다.
- Open-item scan: 보류되거나 명시되지 않은 구현 작업이 없다.
- Type consistency: package name은 kebab-case이고, Rust module crate는
  Cargo에 의해 snake_case로 resolve된다. `lib.rs` 파일은 export 표면으로
  유지하며 모든 구현 코드를 담지 않는다.
- Risk coverage: async timeout/취소, provider 실패 분리, 결정적 순서,
  README source-drift 검사를 할당했다.
