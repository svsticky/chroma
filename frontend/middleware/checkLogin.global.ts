export default defineNuxtRouteMiddleware(async (to, from) => {
  if (to.path.startsWith('/auth')) {
    return
  }

  const auth = useAuth()

  const loggedIn = await auth.checkLoggedIn()

  if (!loggedIn!.auth && loggedIn!.redirect) {
    return navigateTo(loggedIn!.redirect, { external: true })
  }
})
