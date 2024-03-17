import * as pb_1 from 'google-protobuf'

// Todo: Overload solution looks horrible, just split into two separate functions
export async function request<R extends Response>(request: Parameters<typeof $fetch>[0], options?: Parameters<typeof $fetch>[1]): Promise<Response>;
export async function request<R extends pb_1.Message>(ty: typeof pb_1.Message, request: Parameters<typeof $fetch<Blob>>[0], options?: Parameters<typeof $fetch<Blob>>[1]): Promise<R>;

export async function request<R extends pb_1.Message | Response>(tyOrRequest: typeof pb_1.Message | Parameters<typeof $fetch>[0], requestOrOptions?: Parameters<typeof $fetch<Blob>>[0] | Parameters<typeof $fetch>[1], options?: Parameters<typeof $fetch<Blob>>[1]): Promise<R> {
  const parseResponse = typeof tyOrRequest !== 'string'

  const ty = parseResponse ? tyOrRequest as unknown as pb_1.Message : undefined
  const request = parseResponse ? requestOrOptions! as Parameters<typeof $fetch<Blob>>[0] : tyOrRequest as Parameters<typeof $fetch>[0]
  options = parseResponse ? options : requestOrOptions as Parameters<typeof $fetch>[1]

  const opts = {
    ...options,
    baseURL: 'http://localhost:8000',
    headers: {
      ...options?.headers,
      'Content-Type': 'application/protobuf',
      'Accept': 'application/protobuf',
      'Authorization': useCookie('sessionid').value || ''
    }
  }

  if (ty) {
    const response = await $fetch<Blob>(request, opts)

    // @ts-ignore
    return <R>ty!.deserializeBinary(new Uint8Array(await response.arrayBuffer())) as R
  } else {
    return await $fetch(request, opts)
  }
}

