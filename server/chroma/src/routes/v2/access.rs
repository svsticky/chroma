use actix_multiresponse::Payload;
use actix_web::web;
use serde::Deserialize;

use proto::AccessResponse;

use crate::routes::appdata::WebData;
use crate::routes::authorization::Authorization;
use crate::routes::error::WebResult;

#[derive(Debug, Deserialize)]
pub struct Query {
    scope: Option<String>,
}

/// Check if a session ID is still valid.
/// This endpoint should have proper ratelimiting to ensure an
/// attacker cannot fish for valid session IDs.
///
/// This endpoint itself will not return an error, though the [Authorization]
/// middleware might.
///
/// This endpoint can also be used to check if the logged in user has a certain scope, using hte `scope` query parameter.
pub async fn access(
    data: WebData,
    auth: Authorization,
    query: web::Query<Query>,
) -> WebResult<Payload<AccessResponse>> {
    // Checking if the session and tokens are valid
    // is done by the `Authorization` middleware.
    // We only need to return the information here.

    let response = if let Some(check_scope) = &query.scope {
        AccessResponse {
            admin: auth.is_admin,
            has_requested_scope: Some(auth.has_scope(&data.db, check_scope).await?),
            all_scopes: None,
        }
    } else {
        AccessResponse {
            admin: auth.is_admin,
            has_requested_scope: None,
            all_scopes: Some(auth.list_scopes(&data.db).await?),
        }
    };

    Ok(Payload(response))
}
