import { ListAlbumsResponse } from "~/proto/payload/v2/albums/list";
import { Album } from "~/proto/entity/album";
import {
  SearchPhotosRequest,
  SearchPhotosResponse,
} from "~/proto/payload/v2/photos/search";
import { Photo } from "~/proto/entity/photo";
import { BatchDeletePhotosRequest } from "~/proto/payload/v2/photos/batchDelete";

function objectToPaths(object: { [key: string]: any }) {
  const paths: string[] = [];
  const stack: {
    obj: { [key: string]: any };
    path: string[];
  }[] = [{ obj: object, path: [] }];

  while (stack.length > 0) {
    const { obj, path } = stack.pop()!;

    if (typeof obj === "object" && obj !== null) {
      for (const key in obj) {
        stack.push({ obj: obj[key], path: [...path, key] });
      }
    } else {
      paths.push(path.join("."));
    }
  }

  return paths;
}

export default function () {
  return {
    album: {
      list: async () =>
        (
          await net.retrieve<ListAlbumsResponse>(
            ListAlbumsResponse,
            `/api/v2/albums`,
          )
        ).albums.map((album) => album.toObject()),
      get: async (id: string) =>
        (await net.retrieve<Album>(Album, `/api/v2/albums/${id}`)).toObject(),
      create: async (name: string) =>
        (
          await net.retrieve<Album>(Album, "/api/v2/albums", {
            method: "post",
            body: Album.fromObject({
              name,
            }).serializeBinary(),
          })
        ).toObject(),
      update: async (id: string, updateValues: ReturnType<Album["toObject"]>) =>
        (
          await net.retrieve<Album>(Album, `/api/v2/albums/${id}`, {
            method: "patch",
            query: {
              update_mask: objectToPaths(updateValues),
            },
            body: Album.fromObject(updateValues).serializeBinary(),
          })
        ).toObject(),
      delete: async (id: string) =>
        await net.send(`/api/v2/albums/${id}`, { method: "delete" }),
    },
    photo: {
      batchDelete: async (ids: string[]) =>
        await net.send(`/api/v2/photos:batchDelete`, {
          method: "post",
          body: BatchDeletePhotosRequest.fromObject({
            ids,
          }).serializeBinary(),
        }),
      get: async (id: string) =>
        (await net.retrieve<Photo>(Photo, `/api/v2/photos/${id}`)).toObject(),
      delete: async (id: string) =>
        await net.send(`/api/v2/photos/${id}`, { method: "delete" }),
      search: async (albumId?: string) =>
        (
          await net.retrieve<SearchPhotosResponse>(
            SearchPhotosResponse,
            "/api/v2/photos/search",
            {
              method: "post",
              body: SearchPhotosRequest.fromObject({
                ...(albumId && { albumId }),
              }).serializeBinary(),
            },
          )
        ).photos.map((photo) => photo.toObject()),
    },
  };
}
