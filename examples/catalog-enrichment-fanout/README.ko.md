# catalog-enrichment-fanout

이 예제는 catalog 행을 bounded async provider fan-out으로 보강합니다. 필수
provider 실패는 요청 실패가 되고, 선택 provider 실패는 각 product warning으로
반환됩니다.

## Scenario

catalog 페이지는 재고와 추천 데이터를 필요로 합니다. 예제는 요청 메타데이터를
검증하고, 행을 그룹화한 뒤 결정적으로 페이지를 만들며, `map_bounded_collect`로
provider 작업을 실행하고 명시적인 timeout/cancellation 경계로 감쌉니다.

## Representative Code

```rust
let page = enrich_catalog(request).await?;

assert_eq!(page.items()[0].category, "books");
assert!(page.items()[0].warnings.is_empty());
```

## What To Notice

- `map_bounded_collect`는 모든 provider 작업 결과를 입력 순서 기준으로
  기록합니다.
- `with_timeout_or_cancel`로 timeout과 cancellation 동작을 명시합니다.
- 필수 provider 실패는 `CatalogError::RequiredProvider`로 반환하고, 선택
  provider 실패는 product warning으로 노출합니다.

## Run

```bash
cargo test -p catalog-enrichment-fanout
```
