use actix_multiresponse::Payload;
use actix_web::web;
use serde::Deserialize;

use dal::database::{Album, Photo};

use crate::routes::appdata::{AlbumIdCache, WebData};
use crate::routes::authorization::Authorization;
use crate::routes::error::{Error, WebResult};

#[derive(Deserialize)]
pub struct Query {
    update_mask: String,
}

/// Update the metadata of an existing albums.
/// Currently, only the following properties can be updated:
/// - The name
/// - The cover photos
///
/// # Errors
///
/// - If the new name's length is longer than [Album::MAX_NAME_LENGTH]
/// - If the albums to be updated could not be found
/// - If the provided cover photos does not exist
/// - If the provided cover photos is not part of the specified albums
/// - If something went wrong
pub async fn update(
    auth: Authorization,
    data: WebData,
    album_id_cache: web::Data<AlbumIdCache>,
    path: web::Path<String>,
    query: web::Query<Query>,
    payload: Payload<proto::Album>,
) -> WebResult<Payload<proto::Album>> {
    let id = path.into_inner();
    let paths = query
        .update_mask
        .split(',')
        .map(|p| p.trim())
        .collect::<Vec<_>>();

    if !auth.is_admin
        && !auth
            .has_scope(&data.db, "nl.svsticky.chroma.albums.update")
            .await?
    {
        return Err(Error::Forbidden);
    }

    let mut album = Album::get_by_id(&data.db, &id)
        .await?
        .ok_or(Error::NotFound)?;

    if !album.published && !auth.is_admin {
        return Err(Error::Forbidden);
    }

    if paths.contains(&"name") {
        if let Some(name) = &payload.name {
            if name.len() > Album::MAX_NAME_LENGTH {
                return Err(Error::BadRequest(format!(
                    "Provided value 'name' with length '{}' exceeds the maximum length of '{}'",
                    name.len(),
                    Album::MAX_NAME_LENGTH
                )));
            }

            album.update_name(name.clone(), &data.db).await?;
        }
    }

    if paths.contains(&"coverPhoto") {
        if !&payload.cover_photo.is_none() {
            return Err(Error::BadRequest(String::from(
                "'coverPhoto' can only be null with update mask 'coverPhoto'",
            )));
        }

        album.update_cover_photo(None, &data.db).await?;
    } else if paths.contains(&"coverPhoto.id") {
        let photo = match &payload.cover_photo {
            Some(cover_photo) => match &cover_photo.id {
                Some(id) => Photo::get(&data.db, id)
                    .await?
                    .ok_or(Error::BadRequest(format!(
                        "Photo with ID '{}' does not exist",
                        id
                    )))
                    .map(Some),
                None => Err(Error::BadRequest(String::from(
                    "Property 'coverPhoto.id' not set",
                ))),
            },
            None => Err(Error::BadRequest(String::from(
                "Property 'coverPhoto.id' not set",
            ))),
        }?;

        album.update_cover_photo(photo, &data.db).await?;
    }

    if paths.contains(&"published") {
        if let Some(published) = &payload.published {
            // Only admins may change publication settings
            if !auth.is_admin {
                return Err(Error::Forbidden);
            }

            if !published {
                album.set_draft(&data.db).await?;
            } else {
                album
                    .set_published(auth.to_dal_user_type(&data.db).await?, &data.db)
                    .await?;
            }
        }
    }

    album_id_cache.insert(album.id.clone(), album.clone()).await;

    Ok(Payload(album.into()))
}
