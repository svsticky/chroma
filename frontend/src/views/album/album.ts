import {Http} from "@/http";
import {ListAlbumsResponse} from "@/generated/payload/v1/album/list";
import {GetAlbumResponse} from "@/generated/payload/v1/album/get";
import {Album} from "@/generated/entity/album";
import {CreateAlbumRequest, CreateAlbumResponse} from "@/generated/payload/v1/album/create";
import {UpdateAlbumRequest} from "@/generated/payload/v1/album/update";

export interface AlbumModel {
    /**
     * The ID of the album
     */
    id: string,
    /**
     * The name of the album
     */
    name: string,
    /**
     * The ID of the cover photo, if it is set
     */
    coverPhotoId: string | null,
}

/**
 * Convert a proto Album to an AlbumModel
 * @param album The proto album to convert
 */
function protoAlbumToAlbumModel(album: Album): AlbumModel {
    return <AlbumModel> {
        id: album.id,
        coverPhotoId: album.hasCoverPhotoId ? album.coverPhotoId : null,
        name: album.name,
    };
}

/**
 * Upload a photo to an album
 * @param albumId The ID of the album
 * @param photoBytes The bytes of the photo. May be `PNG` or `JPEG` format.
 * @return `true` on success. `undefined` on failure.
 */
export async function uploadPhoto(albumId: string, photoBytes: Uint8Array): Promise<boolean | undefined> {
    // TODO;
    return true;
}

/**
 * Save an edited album.
 * The following values are updated:
 * - `name`
 * @param album The edited album
 * @return `true` on success. `undefined` on failures
 */
export async function saveEditedAlbum(album: AlbumModel): Promise<boolean | undefined> {
    const result = await Http.patch('/api/v1/album', new UpdateAlbumRequest({
        id: album.id,
        name: album.name,
    }), null);

    if(result instanceof Response && !result.ok) {
        return undefined;
    }

    return true;
}

/**
 * List available albums
 *
 * @return The albums on success. `undefined` on failure
 */
export async function listAlbums(): Promise<AlbumModel[] | undefined> {
    const albums = await Http.getBody<ListAlbumsResponse>('/api/v1/album/list', null, ListAlbumsResponse);
    if(albums instanceof Response) {
        return undefined;
    }

    return albums.albums.map(protoAlbumToAlbumModel);
}

/**
 * Get an album by ID
 * @param id The id of the album
 * @return If the album was found, the album. If the album was not found, `null`. `undefined` on failure.
 */
export async function getAlbum(id: string): Promise<AlbumModel | null | undefined> {
    const album = await Http.getBody<GetAlbumResponse>(`/api/v1/album?id=${id}`, null, GetAlbumResponse);
    if(album instanceof Response) {
        if(album.status == 404) {
            return null;
        } else {
            return;
        }
    }

    return protoAlbumToAlbumModel(album.album);
}

/**
 * Create an album
 * @param name The name of the album
 * @return The ID of the album on success. `undefined` on failure
 */
export async function createAlbum(name: string): Promise<string | undefined> {
    const response = await Http.postBody<CreateAlbumResponse>('/api/v1/album', new CreateAlbumRequest({
        name
    }), null, CreateAlbumResponse);

    if(response instanceof Response) {
        return;
    }

    return response.id;
}