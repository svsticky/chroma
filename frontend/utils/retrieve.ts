import * as pb_1 from 'google-protobuf'

export default async function <R extends pb_1.Message>(ty: typeof pb_1.Message, request: Parameters<typeof $fetch<Blob>>[0], options?: Parameters<typeof $fetch<Blob>>[1]): Promise<R> {
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

  const response = await $fetch<Blob>(request, opts)

  return ty.deserializeBinary(new Uint8Array(await response.arrayBuffer())) as R
}
