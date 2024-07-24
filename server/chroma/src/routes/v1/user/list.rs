use actix_multiresponse::Payload;

use dal::database::User;
use proto::ListUserResponse;

use crate::routes::appdata::WebData;
use crate::routes::authorization::Authorization;
use crate::routes::error::{Error, WebResult};

pub async fn list(data: WebData, auth: Authorization) -> WebResult<Payload<ListUserResponse>> {
    if !auth.is_admin {
        return Err(Error::Forbidden);
    }

    let users = User::list(&data.db).await?;

    Ok(Payload(ListUserResponse {
        users: users
            .into_iter()
            .map(|f| proto::User {
                id: f.koala_id,
                name: f.name,
            })
            .collect::<Vec<_>>(),
    }))
}
