use crate::routes::authorization::Authorization;
use actix_multiresponse::Payload;
use proto::AccessResponse;

/// Check if a session ID is still valid.
/// This endpoint should have proper ratelimiting to ensure an
/// attacker cannot fish for valid session IDs.
///
/// This endpoint itself will not return an error, though the [Authorization]
/// middleware might.
pub async fn access(auth: Authorization) -> Payload<AccessResponse> {
    // Checking if the session and tokens are valid
    // is done by the `Authorization` middleware.
    // We only need to return the information here.

    Payload(AccessResponse {
        admin: auth.is_admin,
    })
}
