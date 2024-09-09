use actix_multiresponse::Payload;
use actix_web::web;
use serde::Deserialize;

use dal::database::User;
use proto::GetUserResponse;

use crate::routes::appdata::WebData;
use crate::routes::authorization::Authorization;
use crate::routes::error::{Error, WebResult};

#[derive(Debug, Deserialize)]
pub struct Query {
    id: i32,
}

pub async fn get(
    data: WebData,
    auth: Authorization,
    query: web::Query<Query>,
) -> WebResult<Payload<GetUserResponse>> {
    if !auth.is_admin {
        return Err(Error::Forbidden);
    }

    let user = User::get_by_id(&data.db, query.id)
        .await?
        .ok_or(Error::NotFound)?;

    let scopes = user.get_chroma_scopes().await?;

    Ok(Payload(GetUserResponse {
        user: Some(proto::User {
            id: user.koala_id,
            name: user.name.clone(),
        }),
        scopes: scopes
            .into_iter()
            .map(|f| proto::UserScope {
                name: f.scope,
                granted_by: f.granted_by,
                granted_at: f.granted_at,
            })
            .collect::<Vec<_>>(),
    }))
}
