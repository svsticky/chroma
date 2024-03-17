import {ListAlbumsResponse} from '~/proto/payload/v1/album/list'
import {CreateAlbumRequest, CreateAlbumResponse} from '~/proto/payload/v1/album/create'
import {AlbumWithCoverPhoto} from '~/proto/entity/album'
import {GetAlbumResponse} from '~/proto/payload/v1/album/get'

// Todo: Change all parameters with a default value to an options object
// Todo: Add JSDoc
export async function listAlbums(includeCoverPhoto: boolean = true): Promise<AlbumWithCoverPhoto[]> {
  const response = await request<ListAlbumsResponse>(ListAlbumsResponse, `/api/v1/album/list?include_cover_photo=true&quality_preference=W400`)

  return response.albums
}

export async function createAlbum(name: string, isDraft: boolean): Promise<string> {
  const response = await request<CreateAlbumResponse>(CreateAlbumResponse, '/api/v1/album', {
    method: 'post',
    body: new CreateAlbumRequest({
      name,
      isDraft
    }).serializeBinary()
  })

  return response.id
}

export async function getAlbum(id: string, withoutPhotos: boolean = false, includeCoverPhoto: boolean = true): Promise<AlbumWithCoverPhoto> {
  const response = await request<GetAlbumResponse>(GetAlbumResponse, `/api/v1/album`, {
    query: {
      id,
      without_photos: withoutPhotos,
      include_cover_photo: includeCoverPhoto
    }
  })

  return response.album
}
