import * as pb_1 from "google-protobuf";
import {
  type FetchRequest,
  type FetchOptions,
  default as net,
} from "~/api/net";

function getBaseURL() {
  const runtimeConfig = useRuntimeConfig();
  const baseURL = runtimeConfig.public.apiBase;

  if (typeof baseURL !== "string") {
    throw new Error("runtimeConfig.public.apiBase must be a string");
  }

  return baseURL as string;
}

function getToken() {
  return useAuth().getSessionId() || undefined;
}

export default {
  retrieve: async function <R extends pb_1.Message>(
    ty: typeof pb_1.Message,
    request: FetchRequest,
    options?: FetchOptions,
  ): Promise<R> {
    return net.retrieve<R>(ty, request, getBaseURL(), getToken(), options);
  },
  upload: function <R extends pb_1.Message>(
    ty: typeof pb_1.Message,
    request: FetchRequest,
    options?: FetchOptions,
  ): { response: Promise<R>; progress: Ref<number> } {
    const progress: Ref<number> = ref(0);

    return {
      response: net.upload<R>(
        ty,
        request,
        getBaseURL(),
        getToken(),
        options,
        (p) => (progress.value = p),
      ),
      progress,
    };
  },
  send: async function (
    request: FetchRequest,
    options?: FetchOptions,
  ): Promise<Response> {
    return net.send(request, getBaseURL(), getToken(), options);
  },
};
