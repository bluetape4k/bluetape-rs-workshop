# README 다이어그램 검토

날짜: 2026-06-13
브랜치: `feat/milestone-0.2.0-examples`
범위: 루트 README 파일과 0.2.0 마일스톤 예제 README 파일.

## 범위

다음 README 다이어그램 자산을 추가하거나 갱신했다.

- `workshop-collections-async-architecture`
- `workshop-collections-async-sequence`
- `example-batched-order-windowing`
- `example-catalog-enrichment-fanout`
- `example-shutdown-aware-worker`

generator frame과 화살표 marker 규칙을 강화한 뒤 동일한 스크립트로 기존
0.1.0 자산도 재생성했다.

## 증거

- 편집 전에 GNO guidance를 확인했다.
  - `bluetape4k-docs`: README 다이어그램 Graphviz 증거와 batch 교훈.
  - `bluetape4k-github`: geometry 게이트에 대한 README 다이어그램 PR/이슈 guidance.
- 재사용한 generator: `scripts/generate-foundation-diagrams.py`.
- 다이어그램별 생성 증거: `.dot`, `.plain`, `*-graphviz.svg`,
  `*-graphviz.png`, 최종 `.svg`, 최종 `.png`.
- geometry 요약을 다음 경로에 보존했다.
  `docs/images/readme-diagrams/geometry-summary.txt`.
- 렌더링된 PNG를 다음과 같이 개별 검사했다.
  - `workshop-collections-async-architecture.png`
  - `workshop-collections-async-sequence.png`
  - `example-batched-order-windowing.png`
  - `example-catalog-enrichment-fanout.png`
  - `example-shutdown-aware-worker.png`
- 검토를 마치기 전에 시각적 수정 사항을 적용했다.
  - 대각선 architecture connector를 제거했다.
  - grouping과 node label로 이미 설명되는 교차 장거리 connector를 제거했다.
  - footer box가 최종 카드를 가리지 않도록 흐름 캔버스 높이를 늘렸다.

## 검증

- `python3 scripts/generate-foundation-diagrams.py`: PASS.
- 모든 `docs/images/readme-diagrams/*.svg`에 대한 SVG XML parse: PASS.
- SVG/PNG 쌍 검사: PASS.
- README SVG 삽입 검사: PASS, 로컬 SVG 삽입 없음.
- 금지된 UI font 및 stale marker 검사: PASS, 최종 SVG 파일에 `Inter`, `Arial`,
  `Helvetica`, `8x8`, `13x13` marker 사용 없음.
- README 이미지 링크 검사: PASS, `missing=0`.
- `git diff --check`: PASS.
- `cargo fmt --all --check`: PASS.

## 발견 사항

P0: 없음.

P1: 없음.

P2: 없음.

P3: 없음.

## 게이트 판정

PASS.

P0=0 P1=0
