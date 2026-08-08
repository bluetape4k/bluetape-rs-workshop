# Milestone 0.2.0 Spec 검토

## 범위

- Spec: `docs/superpowers/specs/2026-06-13-milestone-0-2-0-examples-design.md`
- 이슈: `bluetape4k/bluetape-rs-workshop`의 #5, #6, #7, #8
- 검토 모드: 로컬 동등 7-tier 검토.

Subagent 참고: native subagent 도구를 사용할 수 있지만, 노출된 도구
메타데이터는 사용자가 명시적으로 subagent를 요청한 턴에서만 생성을
허용한다. 따라서 이 Step 2-R은 동일한 여섯 관점 경로와 통합 관점에
대해 주 세션에서 수행한 로컬 동등 검토를 기록한다.

## 발견 사항

| 계층 | 관점 | P0 | P1 | P2 | P3 | 증거 |
|---|---|---:|---:|---:|---:|---|
| 1 | 성능 | 0 | 0 | 0 | 0 | Spec는 제한된 `try_map_bounded` / `map_bounded_collect`를 사용하며 benchmark 주장이 없다. |
| 2 | 안정성 | 0 | 0 | 0 | 0 | Spec는 비동기 예제에 timeout 및 취소 테스트를 요구한다. |
| 3 | 보안 | 0 | 0 | 0 | 0 | Spec는 모든 입력을 로컬/메모리 안에 유지하고 caller-controlled 필드의 검증을 요구한다. |
| 4 | 운영/Ops | 0 | 0 | 0 | 0 | Spec는 correlation-aware 로그와 README 취소 계약을 요구한다. |
| 5 | 개발자/API | 0 | 0 | 0 | 0 | Spec는 crate-per-example 패턴과 타입이 지정된 Rust 오류를 따른다. |
| 6 | 사용자/호출자 | 0 | 0 | 0 | 0 | Spec는 모든 예제에 이중 언어 README 파일과 실행 명령을 요구한다. |
| 7 | 통합/증거 | 0 | 0 | 0 | 0 | Spec는 현재 이슈, 기준 `make ci`, 로컬 Cargo 소스 증거를 인용한다. |

## 수렴

P0=0 P1=0. Step 2-R은 종료되었다.

## 참고

- `bluetape-rs-* 0.4.0`으로의 dependency 업그레이드는 명시적으로 범위에서 제외한다.
- GitHub 마일스톤 종료는 이 PR의 범위에서 명시적으로 제외한다.
- 기존 generator를 Graphviz/geometry 증거를 약화하지 않고 확장할 수 있는
  경우가 아니면 README 다이어그램 작업은 선택 사항으로 남긴다.
