use crate::routes::appdata::WebData;
use crate::routes::authorization::Authorization;
use crate::routes::error::{Error, WebResult};
use crate::routes::v1::user::get_user_name;
use actix_multiresponse::Payload;
use dal::database::User;
use futures::future::join_all;
use proto::ListUserResponse;

pub async fn list(data: WebData, auth: Authorization) -> WebResult<Payload<ListUserResponse>> {
    if !auth.is_admin {
        return Err(Error::Forbidden);
    }

    let users = User::list(&data.db).await?;

    Ok(Payload(ListUserResponse {
        users: join_all(users.into_iter().map(|f| {
            let config = data.config.clone();
            async move {
                Ok(proto::User {
                    id: f.koala_id,
                    name: get_user_name(&config, &f).await?,
                })
            }
        }))
        .await
        .into_iter()
        .collect::<WebResult<Vec<_>>>()?,
    }))
}
