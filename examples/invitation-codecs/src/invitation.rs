use bluetape_rs_codec::{
    decode_base58, decode_base64_url_unpadded, decode_utf8_text, encode_base58,
    encode_base64_url_unpadded, encode_utf8_text,
};
use bluetape_rs_core::require_not_blank;

use crate::{
    CallbackState, ExternalReference, InvitationError, InvitationPackage, InvitationRequest,
    InvitationTokenClaims,
};

const EXTERNAL_REFERENCE_PREFIX: &str = "INV-";
const PART_DELIMITER: char = '|';

/// Builds encoded invitation artifacts from validated request fields.
pub fn build_invitation_codes(
    request: InvitationRequest,
) -> Result<InvitationPackage, InvitationError> {
    let tenant = validated_part("tenant", &request.tenant)?;
    let invitation_id = validated_part("invitation_id", &request.invitation_id)?;
    let recipient_id = validated_part("recipient_id", &request.recipient_id)?;
    let redirect_path = validated_part("redirect_path", &request.redirect_path)?;
    let nonce = validated_part("nonce", &request.nonce)?;
    let support_ticket = validated_part("support_ticket", &request.support_ticket)?;

    let invitation_token = encode_urlsafe_parts([tenant, invitation_id, recipient_id]);
    let callback_state = encode_urlsafe_parts([tenant, redirect_path, nonce]);
    let external_reference_payload = join_parts([tenant, support_ticket, invitation_id]);
    let external_reference = format!(
        "{EXTERNAL_REFERENCE_PREFIX}{}",
        encode_base58(encode_utf8_text(external_reference_payload))
    );

    Ok(InvitationPackage {
        invitation_token,
        callback_state,
        external_reference,
    })
}

/// Decodes an invitation token back into typed claims.
pub fn decode_invitation_token(
    encoded: impl AsRef<str>,
) -> Result<InvitationTokenClaims, InvitationError> {
    let parts = decode_urlsafe_parts("invitation_token", encoded, 3)?;

    Ok(InvitationTokenClaims {
        tenant: parts[0].clone(),
        invitation_id: parts[1].clone(),
        recipient_id: parts[2].clone(),
    })
}

/// Decodes callback state returned by a redirect flow.
pub fn decode_callback_state(encoded: impl AsRef<str>) -> Result<CallbackState, InvitationError> {
    let parts = decode_urlsafe_parts("callback_state", encoded, 3)?;

    Ok(CallbackState {
        tenant: parts[0].clone(),
        redirect_path: parts[1].clone(),
        nonce: parts[2].clone(),
    })
}

/// Decodes a support-facing external reference.
pub fn decode_external_reference(
    encoded: impl AsRef<str>,
) -> Result<ExternalReference, InvitationError> {
    let encoded = encoded.as_ref();
    let Some(payload) = encoded.strip_prefix(EXTERNAL_REFERENCE_PREFIX) else {
        return Err(InvitationError::InvalidShape {
            artifact: "external_reference",
            expected_parts: 3,
            actual_parts: 0,
        });
    };

    let decoded = decode_utf8_text(decode_base58(payload)?)?;
    let parts = split_parts("external_reference", &decoded, 3)?;

    Ok(ExternalReference {
        tenant: parts[0].clone(),
        support_ticket: parts[1].clone(),
        invitation_id: parts[2].clone(),
    })
}

fn encode_urlsafe_parts<const N: usize>(parts: [&str; N]) -> String {
    encode_base64_url_unpadded(encode_utf8_text(join_parts(parts)))
}

fn join_parts<const N: usize>(parts: [&str; N]) -> String {
    parts.join(&PART_DELIMITER.to_string())
}

fn decode_urlsafe_parts(
    artifact: &'static str,
    encoded: impl AsRef<str>,
    expected_parts: usize,
) -> Result<Vec<String>, InvitationError> {
    let decoded = decode_utf8_text(decode_base64_url_unpadded(encoded)?)?;
    split_parts(artifact, &decoded, expected_parts)
}

fn split_parts(
    artifact: &'static str,
    decoded: &str,
    expected_parts: usize,
) -> Result<Vec<String>, InvitationError> {
    let parts = decoded.split(PART_DELIMITER).collect::<Vec<_>>();
    if parts.len() != expected_parts || parts.iter().any(|part| part.trim().is_empty()) {
        return Err(InvitationError::InvalidShape {
            artifact,
            expected_parts,
            actual_parts: parts.len(),
        });
    }

    Ok(parts.into_iter().map(ToOwned::to_owned).collect())
}

fn validated_part<'a>(field: &'static str, value: &'a str) -> Result<&'a str, InvitationError> {
    let value = require_not_blank(field, value)?;
    if value.contains(PART_DELIMITER) {
        return Err(InvitationError::ReservedDelimiter {
            field,
            delimiter: PART_DELIMITER,
        });
    }

    Ok(value)
}
