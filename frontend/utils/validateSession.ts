import {AccessResponse} from '~/proto/payload/v1/access'
import {FetchError} from 'ofetch'

export default async function (sessionId: string | null) {
  try {
    const res = await $fetch<AccessResponse>('/api/v1/access', {
      baseURL: 'http://localhost:8000',
      headers: {
        'Content-Type': 'application/protobuf',
        'Accept': 'application/protobuf',
        'Authorization': sessionId || ''
      }
    })

    return {
      auth: true,
      role: res.admin ? 'admin' : 'user'
    }
  } catch (e) {
    if (e instanceof FetchError) {
      switch (e.statusCode) {
        case 200:
          return {
            auth: true,
            role: 'user'
          }
        case 401:
          return {
            auth: false,
            redirect: e.response!.headers.get('location')
          }
        default:
          console.error(`Invalid session ID`)
          return null
      }
    }
  }
}
