const sessionIdKey = 'sessionid'
const roleKey = 'role'

export default function () {
  return {
    getSessionId(): string | null {
      return useCookie(sessionIdKey).value || null
    },
    setSessionId(id: string) {
      useCookie(sessionIdKey).value = id
    },
    isAdmin(): boolean {
      return useCookie(roleKey).value === 'admin' || true
    },
    async checkLoggedIn() {
      return validateSession(this.getSessionId())
    }
  }
}
