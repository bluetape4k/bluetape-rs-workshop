use super::*;

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

#[test]
fn empty_input_returns_empty_page() {
    let page = build_order_batches(BatchRun {
        correlation_id: "corr-batch-002".to_owned(),
        page_number: 0,
        page_size: 10,
        events: Vec::new(),
    })
    .expect("empty input should still produce page metadata");

    assert_eq!(page.total_items(), 0);
    assert!(page.is_empty());
}

#[test]
fn rejects_invalid_page_size() {
    let err = build_order_batches(BatchRun {
        correlation_id: "corr-batch-003".to_owned(),
        page_number: 0,
        page_size: 0,
        events: Vec::new(),
    })
    .expect_err("zero page size should fail");

    assert!(matches!(err, BatchError::Page(_)));
}

#[test]
fn rejects_blank_tenant() {
    let err = build_order_batches(BatchRun {
        correlation_id: "corr-batch-004".to_owned(),
        page_number: 0,
        page_size: 1,
        events: vec![OrderEvent::new(" ", "web", "ord-1", "sku-1", 1)],
    })
    .expect_err("blank tenant should fail");

    assert!(matches!(err, BatchError::Validation(_)));
}
