//! Invitation token and callback codec boundary example.

mod domain;
mod error;
mod invitation;

pub use domain::{
    CallbackState, ExternalReference, InvitationPackage, InvitationRequest, InvitationTokenClaims,
};
pub use error::InvitationError;
pub use invitation::{
    build_invitation_codes, decode_callback_state, decode_external_reference,
    decode_invitation_token,
};

#[cfg(test)]
mod tests;
