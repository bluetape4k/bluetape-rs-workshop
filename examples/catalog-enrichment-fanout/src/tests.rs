use std::collections::HashMap;
use std::time::Duration;

use bluetape_rs_async::AsyncControlError;

use super::*;

#[tokio::test]
async fn enriches_grouped_and_paged_catalog_rows() {
    let page = enrich_catalog(request_with(
        vec![required_provider(), optional_provider()],
        50,
    ))
    .await
    .expect("catalog page should enrich");

    assert_eq!(page.total_items(), 2);
    assert_eq!(page.items()[0].category, "books");
    assert_eq!(page.items()[0].product_id, "sku-1");
    assert_eq!(
        page.items()[0].attributes,
        vec!["inventory=available", "recommender=featured"]
    );
    assert!(page.items()[0].warnings.is_empty());
}

#[tokio::test]
async fn rejects_blank_request_metadata() {
    let mut request = request_with(vec![required_provider()], 50);
    request.correlation_id = " ".to_owned();

    let err = enrich_catalog(request)
        .await
        .expect_err("blank correlation id should fail");

    assert!(matches!(err, CatalogError::Validation(_)));
}

#[tokio::test]
async fn required_provider_failure_fails_request() {
    let mut provider = required_provider();
    provider.failure = Some("upstream 500".to_owned());

    let err = enrich_catalog(request_with(vec![provider], 50))
        .await
        .expect_err("required provider failure should fail");

    assert!(matches!(err, CatalogError::RequiredProvider(_)));
}

#[tokio::test]
async fn optional_provider_failure_is_returned_as_warning() {
    let mut optional = optional_provider();
    optional.failure = Some("no recommendation".to_owned());

    let page = enrich_catalog(request_with(vec![required_provider(), optional], 50))
        .await
        .expect("optional provider failure should not fail");

    assert_eq!(
        page.items()[0].warnings,
        vec!["optional provider recommender failed: no recommendation"]
    );
}

#[tokio::test]
async fn timeout_returns_typed_control_error() {
    let err = enrich_catalog(request_with(vec![required_provider()], 1))
        .await
        .expect_err("slow provider should time out");

    assert!(matches!(
        err,
        CatalogError::Control(AsyncControlError::TimedOut)
    ));
}

fn request_with(providers: Vec<ProviderFixture>, timeout_ms: u64) -> CatalogRequest {
    CatalogRequest {
        correlation_id: "corr-catalog-001".to_owned(),
        page_number: 0,
        page_size: 10,
        max_concurrency: 2,
        timeout: Duration::from_millis(timeout_ms),
        rows: vec![
            CatalogRow::new("sku-1", "books", "Rust Patterns"),
            CatalogRow::new("sku-2", "games", "Async Cards"),
        ],
        providers,
    }
}

fn required_provider() -> ProviderFixture {
    ProviderFixture {
        name: "inventory".to_owned(),
        required: true,
        latency: Duration::from_millis(5),
        attributes: HashMap::from([
            ("sku-1".to_owned(), "available".to_owned()),
            ("sku-2".to_owned(), "backorder".to_owned()),
        ]),
        failure: None,
    }
}

fn optional_provider() -> ProviderFixture {
    ProviderFixture {
        name: "recommender".to_owned(),
        required: false,
        latency: Duration::from_millis(5),
        attributes: HashMap::from([("sku-1".to_owned(), "featured".to_owned())]),
        failure: None,
    }
}
