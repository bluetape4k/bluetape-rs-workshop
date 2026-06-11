use bluetape_rs_core::{ValidationError, blank_to_default, require_not_blank, require_positive};
use bluetape_rs_logging::{CorrelationId, CorrelationIdError};
use thiserror::Error;
use tracing::info;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartnerRow {
    pub tenant: String,
    pub order_id: String,
    pub customer: String,
    pub channel: String,
    pub items: Vec<PartnerItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartnerItem {
    pub sku: String,
    pub quantity: i32,
    pub note: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedOrder {
    pub tenant: String,
    pub order_id: String,
    pub customer: String,
    pub channel: String,
    pub items: Vec<NormalizedItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedItem {
    pub sku: String,
    pub quantity: u32,
    pub note: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CleanupReport {
    pub correlation_id: CorrelationId,
    pub orders: Vec<NormalizedOrder>,
    pub skipped_items: usize,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CleanupError {
    #[error("validation failed: {0}")]
    Validation(#[from] ValidationError),
    #[error("correlation id rejected: {0}")]
    Correlation(#[from] CorrelationIdError),
    #[error("order `{order_id}` does not contain any valid line items")]
    EmptyOrder { order_id: String },
}

pub fn normalize_partner_rows(
    correlation_id: &str,
    rows: &[PartnerRow],
) -> Result<CleanupReport, CleanupError> {
    let correlation_id = CorrelationId::new(require_not_blank("correlation_id", correlation_id)?)?;
    let mut skipped_items = 0;
    let mut orders = Vec::with_capacity(rows.len());

    for row in rows {
        let order_id = require_not_blank("order_id", &row.order_id)?.to_owned();
        let tenant = require_not_blank("tenant", &row.tenant)?.to_owned();
        let customer = require_not_blank("customer", &row.customer)?.to_owned();
        let channel = blank_to_default(&row.channel, "unknown").to_owned();
        let mut items = Vec::with_capacity(row.items.len());

        for item in &row.items {
            if item.sku.trim().is_empty() || item.quantity <= 0 {
                skipped_items += 1;
                continue;
            }

            items.push(NormalizedItem {
                sku: require_not_blank("sku", &item.sku)?.to_owned(),
                quantity: require_positive("quantity", item.quantity)? as u32,
                note: blank_to_default(&item.note, "none").to_owned(),
            });
        }

        if items.is_empty() {
            return Err(CleanupError::EmptyOrder { order_id });
        }

        orders.push(NormalizedOrder {
            tenant,
            order_id,
            customer,
            channel,
            items,
        });
    }

    info!(
        correlation.id = correlation_id.as_str(),
        orders = orders.len(),
        skipped_items,
        "partner order rows normalized"
    );

    Ok(CleanupReport {
        correlation_id,
        orders,
        skipped_items,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use bluetape_rs_logging::{CapturedLogs, capture_subscriber, with_default};

    #[test]
    fn normalizes_partner_rows_and_emits_cleanup_log() {
        let captured = CapturedLogs::new();
        let subscriber =
            capture_subscriber(captured.clone(), "info").expect("subscriber should parse");

        let report = with_default(subscriber, || {
            normalize_partner_rows(
                "corr-001",
                &[PartnerRow {
                    tenant: "north".to_owned(),
                    order_id: "ord-100".to_owned(),
                    customer: "alice".to_owned(),
                    channel: " ".to_owned(),
                    items: vec![
                        PartnerItem {
                            sku: "sku-1".to_owned(),
                            quantity: 2,
                            note: "fragile".to_owned(),
                        },
                        PartnerItem {
                            sku: " ".to_owned(),
                            quantity: 5,
                            note: String::new(),
                        },
                        PartnerItem {
                            sku: "sku-2".to_owned(),
                            quantity: 0,
                            note: String::new(),
                        },
                    ],
                }],
            )
        })
        .expect("partner rows should normalize");

        assert_eq!(report.correlation_id.as_str(), "corr-001");
        assert_eq!(report.skipped_items, 2);
        assert_eq!(report.orders[0].channel, "unknown");
        assert_eq!(report.orders[0].items[0].note, "fragile");

        let logs = captured.to_lossy_string();
        assert!(logs.contains("partner order rows normalized"));
        assert!(logs.contains("corr-001"));
        assert!(logs.contains("skipped_items=2"));
    }

    #[test]
    fn rejects_blank_required_fields() {
        let err = normalize_partner_rows(
            "corr-001",
            &[PartnerRow {
                tenant: String::new(),
                order_id: "ord-100".to_owned(),
                customer: "alice".to_owned(),
                channel: "web".to_owned(),
                items: vec![PartnerItem {
                    sku: "sku-1".to_owned(),
                    quantity: 1,
                    note: String::new(),
                }],
            }],
        )
        .expect_err("blank tenant should fail");

        assert!(matches!(err, CleanupError::Validation(_)));
    }

    #[test]
    fn rejects_orders_after_all_items_are_skipped() {
        let err = normalize_partner_rows(
            "corr-001",
            &[PartnerRow {
                tenant: "north".to_owned(),
                order_id: "ord-100".to_owned(),
                customer: "alice".to_owned(),
                channel: "web".to_owned(),
                items: vec![PartnerItem {
                    sku: " ".to_owned(),
                    quantity: 0,
                    note: String::new(),
                }],
            }],
        )
        .expect_err("empty normalized order should fail");

        assert_eq!(
            err,
            CleanupError::EmptyOrder {
                order_id: "ord-100".to_owned()
            }
        );
    }
}
