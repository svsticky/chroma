use crate::config::Config;
use crate::routes::error::{Error, WebResult};
use crate::routes::routable::Routable;
use actix_web::web;
use actix_web::web::ServiceConfig;
use dal::database::User;
use tap::TapFallible;
use tracing::warn;

mod get;
mod list;
mod update;

pub struct Router;

impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config.service(
            web::scope("/user")
                .route("", web::get().to(get::get))
                .route("", web::patch().to(update::update))
                .route("/list", web::get().to(list::list)),
        );
    }
}

async fn get_user_name(config: &Config, user: &User<'_>) -> WebResult<String> {
    // Update the access token if needed
    let access_token = if user.oauth_expires_at > time::OffsetDateTime::now_utc().unix_timestamp() {
        let exchange = crate::koala::get_new_access_token(&config, &user.refresh_token)
            .await
            .tap_err(|e| warn!("Failed to retrieve new access token from Koala: {e}"))
            .map_err(|e| Error::Koala(e))?;

        // TODO update user records.

        exchange.access_token
    } else {
        user.access_token.clone()
    };

    // Retrieve the user's name
    // TODO Think about storing this in the database
    let user_info = crate::koala::get_user_info(&config, access_token, user.koala_id)
        .await
        .tap_err(|e| warn!("Failed to retrieve user info from Koala: {e}"))
        .map_err(|e| Error::Koala(e))?;

    let user_name = if let Some(infix) = user_info.infix {
        format!("{} {} {}", user_info.first_name, infix, user_info.last_name)
    } else {
        format!("{} {}", user_info.first_name, user_info.last_name)
    };

    Ok(user_name)
}
