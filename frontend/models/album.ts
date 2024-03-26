import {ListAlbumsResponse} from '~/proto/payload/v1/album/list'
import {CreateAlbumRequest, CreateAlbumResponse} from '~/proto/payload/v1/album/create'
import {AlbumWithCoverPhoto} from '~/proto/entity/album'
import {GetAlbumResponse} from '~/proto/payload/v1/album/get'
import {Quality, qualityToString} from '~/models/photo'

/**
 * Options for the listAlbums function
 */
export type ListAlbumOptions = {
  includeCoverPhotos: boolean,
  quality: Quality
}

/**
 * Lists all the albums stored on the back-end.
 *
 * @param {ListAlbumOptions} options
 * @param {boolean} options.includeCoverPhotos - Return URLs or bytes from the cover photos in the response.
 * @param {Quality} options.quality - Preferred quality of the cover photos to retrieve
 */
export async function listAlbums(options: Partial<ListAlbumOptions> = {}): Promise<AlbumWithCoverPhoto[]> {
  const opts: ListAlbumOptions = Object.assign({
    includeCoverPhotos: true,
    quality: Quality.Thumbnail
  }, options)

  const response = await retrieve<ListAlbumsResponse>(ListAlbumsResponse, `/api/v1/album/list`, {
    query: {
      include_cover_photo: opts.includeCoverPhotos,
      quality_preference: qualityToString(opts.quality)
    }
  })

  return response.albums
}

/**
 * Options for the createAlbum function
 */
export type CreateAlbumOptions = {
  isDraft: boolean
}

/**
 * Creates a new album on the server.
 *
 * @param {string} name - Name of the album
 * @param {CreateAlbumOptions} options
 * @param {boolean} options.isDraft - Save the album as a draft on the server
 */
export async function createAlbum(name: string, options: Partial<CreateAlbumOptions> = {}): Promise<string> {
  const opts: CreateAlbumOptions = Object.assign({
    isDraft: true
  }, options)

  const response = await retrieve<CreateAlbumResponse>(CreateAlbumResponse, '/api/v1/album', {
    method: 'post',
    body: new CreateAlbumRequest({
      name,
      isDraft: opts.isDraft
    }).serializeBinary()
  })

  return response.id
}

/**
 * Options for the getAlbum function
 */
export type GetAlbumOptions = {
  includePhotos: boolean,
  includeCoverPhoto: boolean
}

/**
 * Retrieves an album from the server
 *
 * @param {string} id - ID of the album to retrieve
 * @param {GetAlbumOptions} options
 * @param {boolean} options.includePhotos - Return URLs or bytes from the photos on the album in the response.
 * @param {boolean} options.includeCoverPhoto - Return URLs or bytes from the cover photo in the response.
 */
export async function getAlbum(id: string, options: Partial<GetAlbumOptions> = {}): Promise<AlbumWithCoverPhoto> {
  const opts: GetAlbumOptions = Object.assign({
    includePhotos: true,
    includeCoverPhoto: true
  }, options)

  const response = await retrieve<GetAlbumResponse>(GetAlbumResponse, `/api/v1/album`, {
    query: {
      id,
      without_photos: !opts.includePhotos,
      include_cover_photo: opts.includeCoverPhoto
    }
  })

  return response.album
}
