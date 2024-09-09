const sessionIdKey = "sessionid";
const roleKey = "role";

export default function () {
  return {
    getSessionId(): string | null {
      return useCookie(sessionIdKey).value || null;
    },
    isAdmin(): boolean {
      return useCookie(roleKey).value === "admin";
    },
  };
}
