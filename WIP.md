# bluetape-rs-workshop WIP

이 저장소는 실행 가능한 예제로 `bluetape-rs`를 학습한다. 마일스톤은
독립적인 기반 예제에서 통합 백엔드 시나리오로 확장된다.

## 브랜치 정책

- `main`은 유일하게 장기간 유지하는 브랜치다.
- 기능 작업은 `main`을 대상으로 pull request를 연다.
- 각 PR은 관련 마일스톤 이슈를 참조하고, `make ci`로 예제를 실행할 수
  있는 상태를 유지해야 한다.

## 마일스톤 로드맵

| 마일스톤 | Epic | 예제 | 통합 수준 |
|---|---:|---|---|
| 0.1.0 | [#1](https://github.com/bluetape4k/bluetape-rs-workshop/issues/1) | [#2](https://github.com/bluetape4k/bluetape-rs-workshop/issues/2), [#3](https://github.com/bluetape4k/bluetape-rs-workshop/issues/3), [#4](https://github.com/bluetape4k/bluetape-rs-workshop/issues/4) | 검증, 로깅, 테스트 헬퍼를 위한 기반 예제 |
| 0.2.0 | [#5](https://github.com/bluetape4k/bluetape-rs-workshop/issues/5) | [#6](https://github.com/bluetape4k/bluetape-rs-workshop/issues/6), [#7](https://github.com/bluetape4k/bluetape-rs-workshop/issues/7), [#8](https://github.com/bluetape4k/bluetape-rs-workshop/issues/8) | 0.1.0 헬퍼를 요청 및 워커 흐름에 조합 |
| 0.3.0 | [#9](https://github.com/bluetape4k/bluetape-rs-workshop/issues/9) | [#10](https://github.com/bluetape4k/bluetape-rs-workshop/issues/10), [#11](https://github.com/bluetape4k/bluetape-rs-workshop/issues/11), [#12](https://github.com/bluetape4k/bluetape-rs-workshop/issues/12) | 토큰, 지원 참조, 텍스트 수집을 위한 codec 경계 시나리오 추가 |
| 0.4.0 | [#13](https://github.com/bluetape4k/bluetape-rs-workshop/issues/13) | [#14](https://github.com/bluetape4k/bluetape-rs-workshop/issues/14), [#15](https://github.com/bluetape4k/bluetape-rs-workshop/issues/15), [#16](https://github.com/bluetape4k/bluetape-rs-workshop/issues/16) | 이전 마일스톤을 조합하는 통합 서비스 스타일 walkthrough |

## 0.1.0 전달 범위

| 이슈 | 예제 | bluetape-rs API | 완료 조건 |
|---|---|---|---|
| [#2](https://github.com/bluetape4k/bluetape-rs-workshop/issues/2) | `foundation-order-cleanup` | `bluetape-rs-core`, `bluetape-rs-logging` | 파트너 주문 행을 검증하고 로그를 수집하면서 타입이 지정된 주문으로 정규화 |
| [#3](https://github.com/bluetape4k/bluetape-rs-workshop/issues/3) | `request-tracing-log-capture` | `bluetape-rs-core`, `bluetape-rs-logging` | 요청 처리가 correlation-aware 로그를 출력하고 테스트에서 이를 검증 |
| [#4](https://github.com/bluetape4k/bluetape-rs-workshop/issues/4) | `temp-resource-test-harness` | `bluetape-rs-core`, `bluetape-rs-test` | 임시 workspace가 파일 테스트를 격리하고 결정적으로 정리 |

## 0.2.0 전달 범위

| 이슈 | 예제 | bluetape-rs API | 완료 조건 |
|---|---|---|---|
| [#6](https://github.com/bluetape4k/bluetape-rs-workshop/issues/6) | `catalog-enrichment-fanout` | `bluetape-rs-core`, `bluetape-rs-logging`, `bluetape-rs-collections`, `bluetape-rs-async` | 제한된 provider fan-out이 필수 provider 실패, 선택 provider 경고, timeout 제어, 결정적 페이징을 처리 |
| [#7](https://github.com/bluetape4k/bluetape-rs-workshop/issues/7) | `batched-order-windowing` | `bluetape-rs-core`, `bluetape-rs-logging`, `bluetape-rs-collections` | 파트너 주문 이벤트를 검증·그룹화·청크화·정렬하고 명시적인 page 메타데이터와 함께 반환 |
| [#8](https://github.com/bluetape4k/bluetape-rs-workshop/issues/8) | `shutdown-aware-worker` | `bluetape-rs-core`, `bluetape-rs-logging`, `bluetape-rs-collections`, `bluetape-rs-async` | 워커가 그룹화된 작업을 타입이 지정된 timeout 및 shutdown 취소 동작과 함께 실행 |

## 0.3.0 전달 범위

| 이슈 | 예제 | bluetape-rs API | 완료 조건 |
|---|---|---|---|
| [#10](https://github.com/bluetape4k/bluetape-rs-workshop/issues/10) | `invitation-codecs` | `bluetape-rs-core`, `bluetape-rs-codec` | 초대 산출물이 입력을 검증하고 URL-safe token과 callback state를 왕복 변환하며, 잘못된 입력을 거부하고 Base58 지원 참조를 노출 |
| [#11](https://github.com/bluetape4k/bluetape-rs-workshop/issues/11) | `support-reference-encoding` | `bluetape-rs-core`, `bluetape-rs-codec`, `bluetape-rs-collections` | 지원 참조를 일괄 처리하고 명시적으로 잘못된 참조를 처리하면서 디코딩 |
| [#12](https://github.com/bluetape4k/bluetape-rs-workshop/issues/12) | `text-boundary-ingest` | `bluetape-rs-core`, `bluetape-rs-codec`, `bluetape-rs-async` | 텍스트 수집이 유효한 UTF-8, 손실이 있는 표시, downstream 거부 동작을 구분 |
