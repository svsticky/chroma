import {Photo} from "@/generated/entity/photo";
import {Http} from "@/http";
import {ListPhotoResponse} from "@/generated/payload/v1/photo/list";
import {CreatePhotoRequest} from "@/generated/payload/v1/photo/create";
import {DeletePhotoRequest} from "@/generated/payload/v1/photo/delete";
import {GetPhotoResponse} from "@/generated/payload/v1/photo/get";

export interface PhotoModel {
    /**
     * The ID of the photo
     */
    id: string,
    /**
     * The bytes of the photo.
     * Always in `PNG` format.
     */
    photoBytes: Uint8Array,
}

/**
 * Convert a proto Photo to a PhotoModel
 * @param photo The proto Photo to convert
 */
function protoPhotoToPhotoModel(photo: Photo): PhotoModel {
    return <PhotoModel>{
        id: photo.id,
        photoBytes: photo.photoData,
    }
}

/**
 * List photos in an album
 * @param albumId The ID of the album
 * @return The photos in the album on success. `undefined` on failure.
 */
export async function listPhotosInAlbum(albumId: string): Promise<PhotoModel[] | undefined> {
    const result = await Http.getBody<ListPhotoResponse>(`/api/v1/photo/list?album_id=${albumId}`, null, ListPhotoResponse);
    if(result instanceof Response) {
        if(result.ok) {
            return [];
        } else {
            return undefined;
        }
    }

    return result.photos.map(protoPhotoToPhotoModel);
}

/**
 * Create a photo
 * @param albumId The ID of the album
 * @param photoData The bytes of the photo. May be `PNG` or `JPEG` format.
 * @return `true` on success. `undefined` on failure.
 */
export async function createPhoto(albumId: string, photoData: Uint8Array): Promise<boolean | undefined> {
    const result = await Http.post('/api/v1/photo', new CreatePhotoRequest({
        albumId,
        photoData
    }), null);

    return result.ok ? true : undefined;
}

export async function deletePhoto(photoId: string): Promise<boolean | undefined> {
    const result = await Http.del('/api/v1/photo', new DeletePhotoRequest({
        photoId
    }), null);

    return result.ok ? true : undefined;
}

export async function getPhoto(photoId: string): Promise<PhotoModel | null | undefined> {
    const result = await Http.getBody<GetPhotoResponse>(`/api/v1/photo?id=${photoId}`, null, GetPhotoResponse);

    if(result instanceof Response) {
        if(result.status == 404) {
            return null;
        } else {
            return;
        }
    }

    return protoPhotoToPhotoModel(result.photo);
}