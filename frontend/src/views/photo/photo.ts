import {Photo} from "@/generated/entity/photo";
import {Http} from "@/http";
import {ListPhotoResponse} from "@/generated/payload/v1/photo/list";

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