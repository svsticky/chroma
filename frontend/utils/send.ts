export default async function (request: Parameters<typeof $fetch>[0], options?: Parameters<typeof $fetch>[1]): Promise<Response> {
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

  return await $fetch(request, opts)
}

