use crate::routes::appdata::WebData;
use crate::routes::authorization::{Authorization, AuthorizedUser};
use crate::routes::empty::Empty;
use crate::routes::error::{Error, WebResult};
use actix_multiresponse::Payload;
use dal::database::User;
use proto::UpdateUserRequest;
use std::collections::HashSet;

pub async fn update(
    data: WebData,
    auth: Authorization,
    payload: Payload<UpdateUserRequest>,
) -> WebResult<Empty> {
    if !auth.is_admin {
        return Err(Error::Forbidden);
    }

    let granted_by_id = match auth.user {
        AuthorizedUser::Koala { koala_id } => koala_id,
        AuthorizedUser::Service { .. } => {
            return Err(Error::BadRequest(
                "This endpoint does not support service accounts".into(),
            ))?
        }
    };

    let user = User::get_by_id(&data.db, payload.user_id)
        .await?
        .ok_or(Error::NotFound)?;

    // Retrieve existing scopes on the user and stash them in a HashSet
    let existing_scopes = user.get_chroma_scopes().await?;
    let existing_scopes = existing_scopes
        .into_iter()
        .map(|f| f.scope)
        .collect::<HashSet<_>>();

    // Stash all new scopes in a HashSet
    let new_scopes = payload
        .new_scopes
        .clone()
        .into_iter()
        .collect::<HashSet<_>>();

    let intersection = existing_scopes
        .intersection(&new_scopes)
        .collect::<HashSet<_>>();

    // Check that there is no intersection between the set of current scopes
    // and the set of new scopes.
    if !intersection.is_empty() {
        return Err(Error::BadRequest(
            "Some new scopes are alreadyp resent on the user.".into(),
        ));
    }

    let granted_by = User::get_by_id(&data.db, granted_by_id)
        .await?
        .ok_or(Error::NotFound)?;

    // Finally, grant the scopes
    for scope in &new_scopes {
        user.add_scope(scope, &granted_by).await?;
    }

    Ok(Empty)
}
