import {CreatePhotoRequest, CreatePhotoResponse} from '~/proto/payload/v1/photo/create'
import {GetPhotoResponse} from '~/proto/payload/v1/photo/get'
import {type Photo, PhotoRespone, PhotoResponseType} from '~/proto/entity/photo'
import {DeletePhotoRequest} from '~/proto/payload/v1/photo/delete'
import {ListPhotoResponse} from '~/proto/payload/v1/photo/list'

// Todo: Change all parameters with a default value to an options object
// Todo: Add JSDoc
export enum Quality {
  Thumbnail,
  Preview,
  Original,
}

export class DataType {
  static Url = PhotoResponseType.URL
  static Bytes = PhotoResponseType.InResponse
}

function qualityToString(quality: Quality) {
  switch (quality) {
    case Quality.Original:
      return 'Original'
    case Quality.Preview:
      return 'W1600'
    case Quality.Thumbnail:
      return 'W400'
  }
}

/**
 * List photos of the album
 * @param albumId The ID of the album.
 * @param quality Quality of the images to retrieve
 * @return The photos of the album
 */
export async function listPhotosInAlbum(albumId: string, quality: Quality = Quality.Preview): Promise<Photo[]> {
  const response = await request<ListPhotoResponse>(ListPhotoResponse, '/api/v1/photo/list', {
    query: {
      album_id: albumId,
      quality_preference: qualityToString(quality)
    }
  })

  return response.photos
}

export async function createPhoto(albumId: string, photoData: Uint8Array): Promise<string> {
  const response = await request<CreatePhotoResponse>(CreatePhotoResponse, '/api/v1/photo', {
    method: 'post',
    body: new CreatePhotoRequest({
      albumId,
      photoData
    }).serializeBinary()
  })

  return response.photoId
}

export async function deletePhoto(id: string): Promise<boolean> {
  const result = await request<Response>('/api/v1/photo', {
    method: 'delete',
    body: new DeletePhotoRequest({
      photoId: id
    })
  })

  return result.ok
}

export async function getPhoto(id: string, quality: Quality, forceBytes = false): Promise<Photo> {
  const response = await request<GetPhotoResponse>(GetPhotoResponse, `/api/v1/photo`, {
    query: {
      id,
      quality_preference: qualityToString(quality),
      force_bytes: forceBytes
    }
  })

  return response.photo
}
