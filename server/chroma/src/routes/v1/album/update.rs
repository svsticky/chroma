use actix_multiresponse::Payload;
use actix_web::web;

use dal::database::{Album, Photo};
use proto::UpdateAlbumRequest;

use crate::routes::appdata::{AlbumIdCache, WebData};
use crate::routes::authorization::Authorization;
use crate::routes::empty::Empty;
use crate::routes::error::{Error, WebResult};

/// Update the metadata of an existing album.
/// Currently, only the following properties can be updated:
/// - The name
/// - The cover photo
///
/// # Errors
///
/// - If the new name's length is longer than [Album::MAX_NAME_LENGTH]
/// - If the album to be updated could not be found
/// - If the provided cover photo does not exist
/// - If the provided cover photo is not part of the specified album
/// - If something went wrong
pub async fn update(
    auth: Authorization,
    data: WebData,
    album_id_cache: web::Data<AlbumIdCache>,
    payload: Payload<UpdateAlbumRequest>,
) -> WebResult<Empty> {
    if !auth.is_admin
        && !auth
            .has_scope(&data.db, "nl.svsticky.chroma.album.update")
            .await?
    {
        return Err(Error::Forbidden);
    }

    let mut album = Album::get_by_id(&data.db, &payload.id)
        .await?
        .ok_or(Error::NotFound)?;

    if !album.is_draft && !auth.is_admin {
        return Err(Error::Forbidden);
    }

    if let Some(name) = &payload.name {
        if name.len() > Album::MAX_NAME_LENGTH {
            return Err(Error::BadRequest(format!(
                "Provided value 'name' with length '{}' exceeds the maximum length of '{}'",
                name.len(),
                Album::MAX_NAME_LENGTH
            )));
        }

        album.update_name(name, &data.db).await?;
    }

    if let Some(cover_photo_id) = &payload.cover_photo_id {
        let photo = Photo::get_by_id(&data.db, &cover_photo_id)
            .await?
            .ok_or(Error::BadRequest(format!(
                "Cover photo with ID '{cover_photo_id}' does not exist"
            )))?;

        if photo.album_id.ne(&album.id) {
            return Err(Error::BadRequest(format!(
                "Cover photo with ID '{cover_photo_id}' is not in album with ID '{}'",
                album.id
            )));
        }

        album.update_cover_photo(&photo, &data.db).await?;
    }

    if let Some(draft_settings) = &payload.draft_settings {
        // Only admins may change publication settings
        if !auth.is_admin {
            return Err(Error::Forbidden);
        }

        match draft_settings {
            proto::update_album_request::DraftSettings::SetDraft(v) if *v => {
                album.set_draft(&data.db).await?;
            }
            proto::update_album_request::DraftSettings::SetPublished(v) if *v => {
                album
                    .set_published(auth.to_dal_user_type(&data.db).await?, &data.db)
                    .await?;
            }
            _ => {}
        }
    }

    album_id_cache.insert(album.id.clone(), album).await;

    Ok(Empty)
}
