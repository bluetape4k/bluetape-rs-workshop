use bluetape_rs_collections::{Page, iter};
use bluetape_rs_core::{require_not_blank, require_positive};
use bluetape_rs_logging::CorrelationId;
use tracing::info;

use crate::{BatchError, BatchRun, OrderBatch, OrderEvent, OrderLine};

/// Builds a deterministic page of grouped order batches.
pub fn build_order_batches(run: BatchRun) -> Result<Page<OrderBatch>, BatchError> {
    let correlation_id =
        CorrelationId::new(require_not_blank("correlation_id", &run.correlation_id)?)?;
    let page_size = run.page_size;
    let events = validate_events(run.events)?;

    let grouped = iter::group_by(events, |event| {
        (event.tenant.clone(), event.channel.clone())
    });
    let mut grouped: Vec<_> = grouped.into_iter().collect();
    grouped.sort_by(|(left, _), (right, _)| left.cmp(right));

    let mut batches = Vec::new();
    for ((tenant, channel), events) in grouped {
        let lines = events
            .into_iter()
            .map(|event| OrderLine {
                order_id: event.order_id,
                sku: event.sku,
                quantity: event.quantity as u32,
            })
            .collect::<Vec<_>>();

        for (batch_index, chunk) in iter::chunks(lines, page_size as usize)?.enumerate() {
            batches.push(OrderBatch {
                tenant: tenant.clone(),
                channel: channel.clone(),
                batch_index,
                orders: chunk,
            });
        }
    }

    let total_items = batches.len() as u64;
    let items = page_slice(batches, run.page_number, page_size);

    info!(
        correlation.id = correlation_id.as_str(),
        batches = total_items,
        page.number = run.page_number,
        page.size = page_size,
        "order batches built"
    );

    Ok(Page::with_meta(
        items,
        run.page_number,
        page_size,
        total_items,
    )?)
}

fn validate_events(events: Vec<OrderEvent>) -> Result<Vec<OrderEvent>, BatchError> {
    events
        .into_iter()
        .map(|event| {
            Ok(OrderEvent {
                tenant: require_not_blank("tenant", &event.tenant)?.to_owned(),
                channel: require_not_blank("channel", &event.channel)?.to_owned(),
                order_id: require_not_blank("order_id", &event.order_id)?.to_owned(),
                sku: require_not_blank("sku", &event.sku)?.to_owned(),
                quantity: require_positive("quantity", event.quantity)?,
            })
        })
        .collect()
}

fn page_slice<T>(items: Vec<T>, page_number: u64, page_size: u64) -> Vec<T> {
    let start = page_number.saturating_mul(page_size) as usize;
    let size = page_size as usize;
    items.into_iter().skip(start).take(size).collect()
}
