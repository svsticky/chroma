import {Http} from "@/http";
import {ListAlbumsResponse} from "@/generated/payload/v1/album/list";
import {GetAlbumResponse} from "@/generated/payload/v1/album/get";
import {CreateAlbumRequest, CreateAlbumResponse} from "@/generated/payload/v1/album/create";
import {UpdateAlbumRequest} from "@/generated/payload/v1/album/update";
import {DeleteAlbumRequest} from "@/generated/payload/v1/album/delete";
import {PhotoModel, protoPhotoToPhotoModel} from "@/views/photo/photo";
import {AlbumWithCoverPhoto} from "@/generated/entity/album";


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
    /**
     * Whether the album is a draft.
     */
    isDraft: boolean,
    /**
     * The name of the creator.
     * Null if the creator was a service token
     */
    createdBy: string | null,
    /**
     * The name of the publisher.
     * If `is_draft` is true, this value should not be shown and will be `null`.
     * If `is_draft` is false and this value is `null`, the publisher is a service token.
     */
    publishedBy: string | null,

    coverPhoto: PhotoModel | null,

    publishedAt: number | null,
    createdAt: number,
}

/**
 * Convert a proto Album to an AlbumModel
 * @param album The proto album to convert
 */
function protoAlbumToAlbumModel(album: AlbumWithCoverPhoto): AlbumModel {
    return <AlbumModel> {
        id: album.album.id,
        coverPhotoId: album.album.hasCoverPhotoId ? album.album.coverPhotoId : null,
        coverPhoto: album.hasCoverPhoto ? protoPhotoToPhotoModel(album.coverPhoto) : null,
        name: album.album.name,
        isDraft: album.album.isDraft,
        createdBy: album.album.createdBy.name,
        publishedBy: album.album.hasPublishedBy ? album.album.publishedBy.name : null,
        publishedAt: album.album.publishedAt,
        createdAt: album.album.createdAt,
    };
}

export async function setAlbumDraftStatus(album: AlbumModel, draft: boolean): Promise<boolean | undefined> {
    const result = await Http.patch('/api/v1/album', new UpdateAlbumRequest({
        id: album.id,
        setDraft: draft ? true : undefined,
        setPublished: draft ? undefined : true,
    }), null);

    return result.ok ? true : undefined;
}

/**
 * Save an edited album.
 * The following values are updated:
 * - `name`
 * - `coverPhotoId`
 * @param album The edited album
 * @return `true` on success. `undefined` on failures
 */
export async function saveEditedAlbum(album: AlbumModel): Promise<boolean | undefined> {
    const result = await Http.patch('/api/v1/album', new UpdateAlbumRequest({
        id: album.id,
        name: album.name,
        coverPhotoId: album.coverPhotoId ?? undefined, // Convert `null` to `undefined`
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
export async function listAlbums(includeCoverPhoto: boolean = true): Promise<AlbumModel[] | undefined> {
    let query = `include_cover_photo=${includeCoverPhoto}&quality_preference=W400`;


    const albums = await Http.getBody<ListAlbumsResponse>(`/api/v1/album/list?${query}`, null, ListAlbumsResponse);
    if(albums instanceof Response) {
        return undefined;
    }

    return albums.albums.map(protoAlbumToAlbumModel);
}

/**
 * Get an album by ID
 * @param id The id of the album
 * @param without_photos Do not retrieve the album's photos
 * @param includeCoverPhoto Include the cover photo in the response
 * @return If the album was found, the album. If the album was not found, `null`. `undefined` on failure.
 */
export async function getAlbum(id: string, without_photos: boolean = false, includeCoverPhoto: boolean = true): Promise<AlbumModel | null | undefined> {
    let query = `id=${id}`;
    if(without_photos) {
        query = query.concat("&without_photos=true");
    }

    if(!includeCoverPhoto) {
        query = query.concat("&include_cover_photo=false");
    }

    const response = await Http.getBody<GetAlbumResponse>(`/api/v1/album?${query}`, null, GetAlbumResponse);
    if(response instanceof Response) {
        if(response.status == 404) {
            return null;
        } else {
            return;
        }
    }

    return protoAlbumToAlbumModel(response.album);
}

/**
 * Create an album
 * @param name The name of the album
 * @param isDraft Whether the album should be created as a draft
 * @return The ID of the album on success. `undefined` on failure
 */
export async function createAlbum(name: string, isDraft: boolean): Promise<string | undefined> {
    const response = await Http.postBody<CreateAlbumResponse>('/api/v1/album', new CreateAlbumRequest({
        name,
        isDraft
    }), null, CreateAlbumResponse);

    if(response instanceof Response) {
        return;
    }

    return response.id;
}

export async function deleteAlbum(id: string): Promise<boolean | undefined> {
    const result = await Http.del('/api/v1/album', new DeleteAlbumRequest({
        id
    }), null);

    return result.ok ? true : undefined;
}