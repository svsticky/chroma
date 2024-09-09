export default eventHandler(async (event) => {
  const query = getQuery(event);

  if (!query.session_id) {
    throw createError({ statusCode: 400, message: "Session id not found" });
  }

  try {
    const loggedIn = await session.validate(event, query.session_id as string);

    if (loggedIn?.access) {
      if (typeof query.session_id === "string") {
        setCookie(event, "sessionid", query.session_id);
        setCookie(event, "role", loggedIn?.role || "");
      }
    } else if (loggedIn?.redirect) {
      return sendRedirect(event, loggedIn.redirect);
    }

    return sendRedirect(event, "/");
  } catch (e) {
    console.error("Error during login handling: ", e);

    return createError({
      statusCode: 500,
      statusMessage: "Internal server error",
    });
  }
});
