/// Request accepted by the invitation codec example.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvitationRequest {
    pub tenant: String,
    pub invitation_id: String,
    pub recipient_id: String,
    pub redirect_path: String,
    pub nonce: String,
    pub support_ticket: String,
}

impl InvitationRequest {
    #[must_use]
    pub fn new(
        tenant: &str,
        invitation_id: &str,
        recipient_id: &str,
        redirect_path: &str,
        nonce: &str,
        support_ticket: &str,
    ) -> Self {
        Self {
            tenant: tenant.to_owned(),
            invitation_id: invitation_id.to_owned(),
            recipient_id: recipient_id.to_owned(),
            redirect_path: redirect_path.to_owned(),
            nonce: nonce.to_owned(),
            support_ticket: support_ticket.to_owned(),
        }
    }
}

/// Encoded artifacts returned to the application boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvitationPackage {
    pub invitation_token: String,
    pub callback_state: String,
    pub external_reference: String,
}

/// Decoded claims carried by an invitation token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvitationTokenClaims {
    pub tenant: String,
    pub invitation_id: String,
    pub recipient_id: String,
}

/// Decoded callback state carried through redirect flows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallbackState {
    pub tenant: String,
    pub redirect_path: String,
    pub nonce: String,
}

/// Support-facing reference that avoids exposing raw recipient identifiers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalReference {
    pub tenant: String,
    pub support_ticket: String,
    pub invitation_id: String,
}
