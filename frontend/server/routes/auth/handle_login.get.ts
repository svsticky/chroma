import validateSession from "~/utils/validateSession"

export default eventHandler(async event => {
  const query = getQuery(event)

  const loggedIn = await validateSession(query!.session_id as string)

  if (loggedIn!.auth) {
    if (typeof query.session_id === 'string') {
      setCookie(event, 'sessionid', query.session_id)
      setCookie(event, 'role', loggedIn!.role || '')
    }
  } else if (loggedIn!.redirect) {
    return sendRedirect(event, loggedIn!.redirect)
  }

  return sendRedirect(event, '/')
})
