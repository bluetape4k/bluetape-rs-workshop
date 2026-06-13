use bluetape_rs_async::{CancellationSource, map_bounded_collect, with_timeout_or_cancel};
use bluetape_rs_collections::{Page, iter};
use bluetape_rs_core::{require_not_blank, require_positive};
use bluetape_rs_logging::CorrelationId;
use tokio::time::sleep;
use tracing::{info, warn};

use crate::{CatalogError, CatalogRequest, CatalogRow, EnrichedProduct, ProviderError};

/// Enriches catalog rows with bounded async provider fan-out.
pub async fn enrich_catalog(
    request: CatalogRequest,
) -> Result<Page<EnrichedProduct>, CatalogError> {
    let correlation_id = CorrelationId::new(require_not_blank(
        "correlation_id",
        &request.correlation_id,
    )?)?;
    let page_size = require_positive("page_size", request.page_size)?;
    let max_concurrency = require_positive("max_concurrency", request.max_concurrency)?;
    let rows = validate_rows(request.rows)?;
    let providers = request.providers;

    let grouped = iter::group_by(rows, |row| row.category.clone());
    let mut rows = grouped
        .into_values()
        .flat_map(|mut rows| {
            rows.sort_by(|left, right| left.product_id.cmp(&right.product_id));
            rows
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.category
            .cmp(&right.category)
            .then_with(|| left.product_id.cmp(&right.product_id))
    });

    let (_source, token) = CancellationSource::new();
    let report = with_timeout_or_cancel(request.timeout, token, async move {
        map_bounded_collect(rows, max_concurrency, move |row| {
            let providers = providers.clone();
            async move { enrich_row(row, providers).await }
        })
        .await
    })
    .await??;

    if let Some(failure) = report
        .failures
        .into_iter()
        .map(|failure| failure.error)
        .next()
    {
        return Err(CatalogError::RequiredProvider(failure));
    }

    let products = report
        .successes
        .into_iter()
        .map(|success| success.value)
        .collect::<Vec<_>>();
    let total_items = products.len() as u64;
    let items = page_slice(products, request.page_number, page_size);

    info!(
        correlation.id = correlation_id.as_str(),
        products = total_items,
        page.number = request.page_number,
        page.size = page_size,
        "catalog enrichment completed"
    );

    Ok(Page::with_meta(
        items,
        request.page_number,
        page_size,
        total_items,
    )?)
}

fn validate_rows(rows: Vec<CatalogRow>) -> Result<Vec<CatalogRow>, CatalogError> {
    rows.into_iter()
        .map(|row| {
            Ok(CatalogRow {
                product_id: require_not_blank("product_id", &row.product_id)?.to_owned(),
                category: require_not_blank("category", &row.category)?.to_owned(),
                title: require_not_blank("title", &row.title)?.to_owned(),
            })
        })
        .collect()
}

async fn enrich_row(
    row: CatalogRow,
    providers: Vec<crate::ProviderFixture>,
) -> Result<EnrichedProduct, ProviderError> {
    let mut attributes = Vec::new();
    let mut warnings = Vec::new();

    for provider in providers {
        sleep(provider.latency).await;

        if let Some(message) = provider.failure {
            if provider.required {
                return Err(ProviderError {
                    name: provider.name,
                    product_id: row.product_id,
                    required: true,
                    message,
                });
            }

            warn!(
                provider.name = provider.name.as_str(),
                product.id = row.product_id.as_str(),
                "optional provider failed"
            );
            warnings.push(format!(
                "optional provider {} failed: {}",
                provider.name, message
            ));
            continue;
        }

        if let Some(value) = provider.attributes.get(&row.product_id) {
            attributes.push(format!("{}={}", provider.name, value));
        }
    }

    Ok(EnrichedProduct {
        product_id: row.product_id,
        category: row.category,
        title: row.title,
        attributes,
        warnings,
    })
}

fn page_slice<T>(items: Vec<T>, page_number: u64, page_size: u64) -> Vec<T> {
    let start = page_number.saturating_mul(page_size) as usize;
    let size = page_size as usize;
    items.into_iter().skip(start).take(size).collect()
}
