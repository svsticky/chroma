import {Photo, PhotoResponseType} from "@/generated/entity/photo";
import {Http} from "@/http";
import {ListPhotoResponse} from "@/generated/payload/v1/photo/list";
import {CreatePhotoRequest} from "@/generated/payload/v1/photo/create";
import {DeletePhotoRequest} from "@/generated/payload/v1/photo/delete";
import {GetPhotoResponse} from "@/generated/payload/v1/photo/get";


export enum PhotoDataKind {
    URL,
    BYTES,
}

export class PhotoModel {
    /**
     * The ID of the photo
     */
    id: string;
    dataKind: PhotoDataKind;
    photoUrl: string | undefined;
    photoBytes: Uint8Array | undefined;

    constructor(id: string, dataKind: PhotoDataKind, photoUrl: string | undefined, photoBytes: Uint8Array | undefined) {
        this.id = id;
        this.dataKind = dataKind;
        this.photoUrl = photoUrl;
        this.photoBytes = photoBytes;
    }

    getAsSrcUrl(): string {
        switch(this.dataKind) {
            case PhotoDataKind.URL: {
                return this.photoUrl!;
            }
            case PhotoDataKind.BYTES: {
                const blob = new Blob([this.photoBytes!], {
                    type: "image/webp"
                });
                return URL.createObjectURL(blob);
            }
        }
    }
}

/**
 * Convert a proto Photo to a PhotoModel
 * @param photo The proto Photo to convert
 */
export function protoPhotoToPhotoModel(photo: Photo): PhotoModel {
    switch(photo.dataType) {
        case PhotoResponseType.URL: {
            return new PhotoModel(photo.id, PhotoDataKind.URL, photo.data.url, undefined);
        }
        case PhotoResponseType.InResponse: {
            return new PhotoModel(photo.id, PhotoDataKind.BYTES, undefined, photo.data.bytes);
        }
    }
}

/**
 * List photos in an album
 * @param albumId The ID of the album.
 * @param low_res Get the Low resolution variant of the image.
 * @return The photos in the album on success. `undefined` on failure.
 */
export async function listPhotosInAlbum(albumId: string, low_res: boolean = false): Promise<PhotoModel[] | undefined> {
    let query = `album_id=${albumId}`;
    if(low_res) {
        query = query.concat("&quality_preference=W400");
    }

    const result = await Http.getBody<ListPhotoResponse>(`/api/v1/photo/list?${query}`, null, ListPhotoResponse);
    if(result instanceof Response) {
        if(result.ok) {
            return [];
        } else {
            return undefined;
        }
    }

    return result.photos.map(protoPhotoToPhotoModel);
}

export class TooManyRequests extends Error {
    retryAfter: number;

    constructor(retryAfter: number) {
        super();
        this.retryAfter = retryAfter;
    }
}

/**
 * Create a photo
 * @param albumId The ID of the album
 * @param photoData The bytes of the photo. May be `PNG` or `JPEG` format.
 * @throws TooManyRequests If too many requests are issued
 * @return `true` on success. `undefined` on failure.
 */
export async function createPhoto(albumId: string, photoData: Uint8Array): Promise<boolean | undefined> {
    const result = await Http.post('/api/v1/photo', new CreatePhotoRequest({
        albumId,
        photoData
    }), null);

    if(result.ok) {
        return true;
    }

    if(result.status == 429) {
        throw new TooManyRequests(
            Number.parseInt(result.headers.get('retry-after') ?? "1")
        );
    }

    return undefined;
}

export async function deletePhoto(photoId: string): Promise<boolean | undefined> {
    const result = await Http.del('/api/v1/photo', new DeletePhotoRequest({
        photoId
    }), null);

    return result.ok ? true : undefined;
}

export enum Quality {
    W400,
    W1600,
    ORIGINAL,
}

export async function getPhoto(photoId: string, quality: Quality, forceBytes = false): Promise<PhotoModel | null | undefined> {

    let qualityString;
    switch(quality) {
        case Quality.ORIGINAL: {
            qualityString = "Original";
            break;
        }
        case Quality.W1600: {
            qualityString = "W1600";
            break;
        }
        case Quality.W400: {
            qualityString = "W400";
            break;
        }
    }

    const query = `id=${photoId}&quality_preference=${qualityString}&force_bytes=${forceBytes}`;
    const result = await Http.getBody<GetPhotoResponse>(`/api/v1/photo?${query}`, null, GetPhotoResponse);
    if(result instanceof Response) {
        if(result.status == 404) {
            return null;
        } else {
            return;
        }
    }

    return protoPhotoToPhotoModel(result.photo);
}