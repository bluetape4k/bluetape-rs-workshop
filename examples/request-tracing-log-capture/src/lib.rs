use bluetape_rs_core::{ValidationError, require_not_blank};
use bluetape_rs_logging::{CorrelationId, CorrelationIdError};
use thiserror::Error;
use tracing::info;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestSummary {
    pub correlation_id: CorrelationId,
    pub route: String,
    pub status: u16,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum RequestTraceError {
    #[error("validation failed: {0}")]
    Validation(#[from] ValidationError),
    #[error("correlation id rejected: {0}")]
    Correlation(#[from] CorrelationIdError),
    #[error("invalid HTTP status `{0}`")]
    InvalidStatus(u16),
}

pub fn record_request(
    correlation_id: &str,
    route: &str,
    status: u16,
) -> Result<RequestSummary, RequestTraceError> {
    if !(100..=599).contains(&status) {
        return Err(RequestTraceError::InvalidStatus(status));
    }

    let correlation_id = CorrelationId::new(require_not_blank("correlation_id", correlation_id)?)?;
    let route = require_not_blank("route", route)?.to_owned();

    info!(
        correlation.id = correlation_id.as_str(),
        request.route = route.as_str(),
        http.status = status,
        "request completed"
    );

    Ok(RequestSummary {
        correlation_id,
        route,
        status,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use bluetape_rs_logging::{CapturedLogs, capture_subscriber, with_default};

    #[test]
    fn captures_correlation_aware_request_log() {
        let captured = CapturedLogs::new();
        let subscriber =
            capture_subscriber(captured.clone(), "info").expect("subscriber should parse");

        let summary = with_default(subscriber, || record_request("corr-042", "/orders", 202))
            .expect("request should record");

        assert_eq!(summary.correlation_id.as_str(), "corr-042");
        assert_eq!(summary.route, "/orders");
        assert_eq!(summary.status, 202);

        let logs = captured.to_lossy_string();
        assert!(logs.contains("request completed"));
        assert!(logs.contains("corr-042"));
        assert!(logs.contains("/orders"));
        assert!(logs.contains("http.status=202"));
    }

    #[test]
    fn rejects_blank_route_before_logging() {
        let err = record_request("corr-042", " ", 200).expect_err("blank route should fail");

        assert!(matches!(err, RequestTraceError::Validation(_)));
    }

    #[test]
    fn rejects_invalid_status() {
        let err = record_request("corr-042", "/orders", 99).expect_err("status should fail");

        assert_eq!(err, RequestTraceError::InvalidStatus(99));
    }
}
