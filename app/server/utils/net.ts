import * as pb_1 from "google-protobuf";
import type { H3Event } from "h3";
import {
  type FetchRequest,
  type FetchOptions,
  default as net,
} from "~/api/net";

function getBaseURL(event: H3Event) {
  const runtimeConfig = useRuntimeConfig(event);
  const baseURL = runtimeConfig.public.apiBase;

  if (typeof baseURL !== "string") {
    throw new Error("runtimeConfig.public.apiBase must be a string");
  }

  return baseURL as string;
}

function getToken(event: H3Event) {
  return getCookie(event, "sessionid") || undefined;
}

export default {
  retrieve: async function <R extends pb_1.Message>(
    ty: typeof pb_1.Message,
    event: H3Event,
    request: FetchRequest,
    options?: FetchOptions,
  ): Promise<R> {
    return net.retrieve<R>(
      ty,
      request,
      getBaseURL(event),
      getToken(event),
      options,
    );
  },
  upload: function <R extends pb_1.Message>(
    ty: typeof pb_1.Message,
    event: H3Event,
    request: FetchRequest,
    options?: FetchOptions,
    onProgress?: (progress: number) => void,
  ): Promise<R> {
    return net.upload<R>(
      ty,
      request,
      getBaseURL(event),
      getToken(event),
      options,
      onProgress,
    );
  },
  send: async function (
    request: FetchRequest,
    event: H3Event,
    options?: FetchOptions,
  ): Promise<Response> {
    return net.send(request, getBaseURL(event), getToken(event), options);
  },
};
