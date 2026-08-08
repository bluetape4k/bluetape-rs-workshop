# Milestone 0.1.0 검토 증거

## 범위

- 초기 `bluetape-rs-workshop` Rust workspace를 추가했다.
- 다음 0.1.0 마일스톤 예제를 구현했다.
  - `foundation-order-cleanup`
  - `request-tracing-log-capture`
  - `temp-resource-test-harness`
- `WIP.md`, 이중 언어 README 파일, README 다이어그램 자산을 추가했다.

## 7-Tier 검토

| 계층 | 결과 | 증거 |
|---|---|---|
| 기능 정확성 | Pass | 9개 단위 테스트가 세 예제의 성공, 실패, 경계 사례를 다룬다. |
| API 호환성 | Pass | 예제가 crates.io에 게시된 `bluetape-rs-*` `0.3.1` crate를 대상으로 컴파일된다. |
| Rust 관용구 | Pass | 타입이 지정된 오류와 `Result`, 소유 출력 모델을 사용하며, `unsafe`와 라이브러리 코드의 패닉이 없다. |
| 비동기/동시성 | Pass | 0.1.0 예제에는 비동기 또는 공유 가변 동시성 표면이 없다. |
| 테스트 규율 | Pass | doctest 하네스를 포함해 `cargo test --workspace --all-features`가 통과한다. |
| 문서화 | Pass | `README.md`, `README.ko.md`, 예제별 README 파일이 실행 명령과 시나리오 의도를 설명한다. |
| 다이어그램 게이트 | Pass | `scripts/generate-foundation-diagrams.py`가 최종 SVG/PNG 쌍, Graphviz `.dot/.plain/*-graphviz.svg/*-graphviz.png` 증거, `docs/images/readme-diagrams/geometry-summary.txt`를 재생성한다. |
| CI/릴리스 준비도 | Pass | GitHub CI가 `main` 대상 PR에 대해 fmt, clippy, all-features 테스트를 실행한다. |

## 검증

```text
make ci
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

결과: 2026-06-11에 로컬에서 통과했다.

다이어그램 검증:

```text
python3 scripts/generate-foundation-diagrams.py
xmllint --noout docs/images/readme-diagrams/*.svg
rg 'Inter|Arial|Helvetica' docs/images/readme-diagrams/*.svg
rg 'docs/images/readme-diagrams/.*\.svg' README.md README.ko.md examples/*/README*.md
```

결과: generator가 완료되었고, SVG 파싱이 통과했으며, 금지된 UI 글꼴이 없고 README 파일은 PNG만 삽입한다.

## 후속 결함

처음 생성한 예제 흐름 다이어그램은 카드 간격이 너무 좁아 README 크기에서
커넥터 줄기를 확인하기 어려웠다. 캔버스 높이와 카드 간격을 늘리는 데서
끝내지 않고, 이 결함을 `scripts/generate-foundation-diagrams.py`의 검증으로
승격했다.

- 수직 예제 흐름의 카드 간격에 더 큰 간격값을 사용한다.
- `geometry-summary.txt`에 `shortConnectors`와 `minConnectorStem`을 기록한다.
- 직접 연결되는 수직 커넥터 줄기가 28px보다 짧으면 generator가 실패한다.

## 위험

- P0: 0
- P1: 0
- 잔여 위험: 다이어그램 렌더링은 GitHub README 표시에 커밋된 PNG 자산을 사용한다. 재생성을 위해 SVG와 Graphviz 소스도 커밋되어 있다.
