use bluetape_rs_codec::encode_base64_url_unpadded;

use super::*;

#[test]
fn builds_url_safe_invitation_codes_and_decodes_them() {
    let package = build_invitation_codes(InvitationRequest::new(
        "north",
        "invite-42",
        "recipient-7",
        "/accept",
        "nonce-001",
        "SUP-1001",
    ))
    .expect("valid invitation inputs should build codes");

    assert!(!package.invitation_token.contains(['+', '/', '=']));
    assert!(!package.callback_state.contains(['+', '/', '=']));
    assert!(package.external_reference.starts_with("INV-"));
    assert!(!package.external_reference.contains("recipient-7"));

    assert_eq!(
        decode_invitation_token(&package.invitation_token).expect("token should decode"),
        InvitationTokenClaims {
            tenant: "north".to_owned(),
            invitation_id: "invite-42".to_owned(),
            recipient_id: "recipient-7".to_owned(),
        }
    );
    assert_eq!(
        decode_callback_state(&package.callback_state).expect("state should decode"),
        CallbackState {
            tenant: "north".to_owned(),
            redirect_path: "/accept".to_owned(),
            nonce: "nonce-001".to_owned(),
        }
    );
    assert_eq!(
        decode_external_reference(&package.external_reference).expect("reference should decode"),
        ExternalReference {
            tenant: "north".to_owned(),
            support_ticket: "SUP-1001".to_owned(),
            invitation_id: "invite-42".to_owned(),
        }
    );
}

#[test]
fn rejects_blank_request_fields_before_encoding() {
    let err = build_invitation_codes(InvitationRequest::new(
        " ",
        "invite-42",
        "recipient-7",
        "/accept",
        "nonce-001",
        "SUP-1001",
    ))
    .expect_err("blank tenant should fail validation");

    assert!(matches!(err, InvitationError::Validation(_)));
}

#[test]
fn rejects_reserved_delimiter_before_encoding() {
    let err = build_invitation_codes(InvitationRequest::new(
        "north|east",
        "invite-42",
        "recipient-7",
        "/accept",
        "nonce-001",
        "SUP-1001",
    ))
    .expect_err("delimiter-bearing tenant should fail before encoding");

    assert!(matches!(
        err,
        InvitationError::ReservedDelimiter {
            field: "tenant",
            delimiter: '|',
        }
    ));
}

#[test]
fn rejects_malformed_url_safe_callback_state() {
    let err = decode_callback_state("not valid base64+")
        .expect_err("invalid URL-safe Base64 should fail");

    assert!(matches!(err, InvitationError::Base64(_)));
}

#[test]
fn rejects_callback_state_with_invalid_shape() {
    let malformed = encode_base64_url_unpadded("north|/accept");
    let err = decode_callback_state(malformed).expect_err("state with missing nonce should fail");

    assert!(matches!(
        err,
        InvitationError::InvalidShape {
            artifact: "callback_state",
            ..
        }
    ));
}

#[test]
fn rejects_external_reference_with_ambiguous_base58_character() {
    let err =
        decode_external_reference("INV-0OIl").expect_err("ambiguous Base58 characters should fail");

    assert!(matches!(err, InvitationError::Base58(_)));
}
