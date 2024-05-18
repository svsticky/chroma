import {CreatePhotoRequest, CreatePhotoResponse} from '~/proto/payload/v1/photo/create'
import {GetPhotoResponse} from '~/proto/payload/v1/photo/get'
import {type Photo, PhotoResponseType} from '~/proto/entity/photo'
import {DeletePhotoRequest} from '~/proto/payload/v1/photo/delete'
import {ListPhotoResponse} from '~/proto/payload/v1/photo/list'

/**
 * List of possible photo qualities
 */
export enum Quality {
  Thumbnail,
  Preview,
  Original,
}

/**
 * Helper function to convert the quality enum to the respective quality string
 *
 * @param quality Quality type to convert to a string
 */
export function qualityToString(quality: Quality) {
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
 * Convert the PhotoResponseType types to my own type
 */
export class DataType {
  static Url = PhotoResponseType.URL
  static Bytes = PhotoResponseType.InResponse
}

/**
 * Options for the listPhotosInAlbum function
 */
export type ListPhotosInAlbumOptions = {
  quality: Quality
}

/**
 *
 * @param {string} albumId - ID of the album to list the photos of
 * @param {ListPhotosInAlbumOptions} options
 * @param {Quality} options.quality - Preferred quality of the photos to retrieve
 */
export async function listPhotosInAlbum(albumId: string, options: Partial<ListPhotosInAlbumOptions> = {}): Promise<Photo[]> {
  const opts: ListPhotosInAlbumOptions = Object.assign({
    quality: Quality.Thumbnail
  }, options)

  const response = await retrieve<ListPhotoResponse>(ListPhotoResponse, '/api/v1/photo/list', {
    query: {
      album_id: albumId,
      quality_preference: qualityToString(opts.quality)
    }
  })

  return response.photos
}

/**
 * Stores a photo on the server and attaches it to an album
 *
 * @param {string} albumId - ID of the album to store the photo in
 * @param {Uint8Array} photoData - Image data to store on the server
 */
export async function createPhoto(albumId: string, photoData: Uint8Array): Promise<string> {
  const response = await retrieve<CreatePhotoResponse>(CreatePhotoResponse, '/api/v1/photo', {
    method: 'post',
    body: new CreatePhotoRequest({
      albumId,
      photoData
    }).serializeBinary()
  })

  return response.photoId
}

/**
 * Removes a photo from the server
 *
 * @param {string} id - ID of the photo to delete
 */
export async function deletePhoto(id: string): Promise<boolean> {
  const result = await send('/api/v1/photo', {
    method: 'delete',
    body: new DeletePhotoRequest({
      photoId: id
    })
  })

  return result.ok
}

/**
 * Options for the getPhoto function
 */
export type GetPhotoOptions = {
  quality: Quality,
  forceBytes: boolean
}

/**
 * Retrieves a photo from the server
 *
 * @param {string} id - ID of the photo to retrieve
 * @param {GetAlbumOptions} options
 * @param {Quality} options.quality - Preferred quality of the photo to retrieve
 * @param {boolean} options.forceBytes - Ensures the datatype of the response is always bytes
 */
export async function getPhoto(id: string, options: Partial<GetPhotoOptions> = {}): Promise<Photo> {
  const opts: GetPhotoOptions = Object.assign({
    quality: Quality.Preview,
    forceBytes: false
  }, options)

  const response = await retrieve<GetPhotoResponse>(GetPhotoResponse, `/api/v1/photo`, {
    query: {
      id,
      quality_preference: qualityToString(opts.quality),
      force_bytes: opts.forceBytes
    }
  })

  return response.photo
}
