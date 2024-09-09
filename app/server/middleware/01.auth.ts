import { FetchError } from "ofetch";

export default defineEventHandler(async (event) => {
  if (event.path.startsWith("/auth")) {
    return;
  }

  try {
    const loginStatus = await session.validate(event);

    if (!loginStatus.access) {
      if (loginStatus.redirect) {
        return sendRedirect(event, loginStatus.redirect);
      } else {
        return createError({ statusCode: 401, statusMessage: "Unauthorized" });
      }
    }
  } catch (e) {
    if (e instanceof FetchError) {
      console.error("Failed to reach back-end server during auth", e.message);
      return createError({ statusCode: 504, statusMessage: "Gateway Timeout" });
    } else if (e instanceof Error) {
      console.error("Authorization middleware error:", e.message);
    } else {
      console.error(e);
    }

    return createError({
      statusCode: 500,
      statusMessage: "Internal server error",
    });
  }
});
