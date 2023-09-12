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

    // The user to grant to
    let grantee = User::get_by_id(&data.db, payload.user_id)
        .await?
        .ok_or(Error::NotFound)?;

    // Get the current list of scopes
    let existing_scopes = grantee.get_chroma_scopes().await?
        .into_iter()
        .map(|f| f.scope)
        .collect::<HashSet<_>>();

    // Stash all new scopes in a HashSet
    let new_scopes = payload.new_scopes
        .clone()
        .into_iter()
        .collect::<HashSet<_>>();

    // The set of scopes that should be /removed/ from the user
    let to_remove = existing_scopes
        .difference(&new_scopes)
        .collect::<HashSet<_>>();

    // The set of scopes that should be /added/ to the user
    let to_add = new_scopes
        .difference(&existing_scopes)
        .collect::<HashSet<_>>();

    // User who is granting the scopes
    let granted_by = User::get_by_id(&data.db, granted_by_id)
        .await?
        .ok_or(Error::NotFound)?;

    for scope in &to_add {
        grantee.add_scope(scope, &granted_by).await?;
    }

    for scope in &to_remove {
        grantee.remove_scope_by_name(scope).await?;
    }

    Ok(Empty)
}
