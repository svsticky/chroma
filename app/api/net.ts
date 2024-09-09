import * as pb_1 from "google-protobuf";

export type FetchRequest = NonNullable<Parameters<typeof $fetch<Blob>>[0]>;
export type FetchOptions = NonNullable<Parameters<typeof $fetch<Blob>>[1]>;

function createOptions(
  baseURL: string,
  token?: string,
  options?: FetchOptions,
): FetchOptions {
  return {
    ...options,
    baseURL,
    headers: {
      "Content-Type": "application/protobuf",
      ...(token && { Authorization: token }),
      ...options?.headers,
      Accept: "application/protobuf",
    },
  };
}

export default {
  retrieve: async function <R extends pb_1.Message>(
    ty: typeof pb_1.Message,
    request: FetchRequest,
    baseURL: string,
    token?: string,
    options?: FetchOptions,
  ): Promise<R> {
    const response = await $fetch<Blob>(
      request,
      createOptions(baseURL, token, options),
    );

    return ty.deserializeBinary(
      new Uint8Array(await response.arrayBuffer()),
    ) as R;
  },
  upload: async function <R extends pb_1.Message>(
    ty: typeof pb_1.Message,
    request: FetchRequest,
    baseURL: string,
    token?: string,
    options?: FetchOptions,
    onProgress?: (progress: number) => void,
  ): Promise<R> {
    return new Promise((resolve, reject) => {
      const opts = createOptions(baseURL, token, options);

      const xhr = new XMLHttpRequest();
      xhr.open(opts.method || "post", `${opts.baseURL}/${request}`, true);
      xhr.responseType = "arraybuffer";

      for (const [name, value] of Object.entries(opts.headers!)) {
        xhr.setRequestHeader(name, value);
      }

      xhr.upload.onprogress = function (event) {
        if (!event.lengthComputable || !onProgress) return;
        onProgress((event.loaded / event.total) * 100);
      };

      xhr.onload = function () {
        if (xhr.status >= 200 && xhr.status < 300) {
          resolve(ty.deserializeBinary(new Uint8Array(xhr.response)) as R);
        } else {
          reject(new Error(`Upload failed with status: ${xhr.status}`));
        }
      };

      xhr.onerror = function () {
        reject(new Error("Upload failed due to a network error"));
      };

      xhr.send(opts.body as any);
    });
  },
  send: async function (
    request: FetchRequest,
    baseURL: string,
    token?: string,
    options?: FetchOptions,
  ): Promise<Response> {
    return await $fetch(request, createOptions(baseURL, token, options));
  },
};
