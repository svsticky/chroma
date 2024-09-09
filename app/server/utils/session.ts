import { AccessResponse } from "~/proto/payload/v2/access";
import { FetchError } from "ofetch";
import { H3Event } from "h3";

type Session =
  | {
      access: true;
      role: "admin" | "user";
    }
  | {
      access: false;
      redirect: string | null;
    };

export default {
  validate: async function (event: H3Event, token?: string): Promise<Session> {
    try {
      const res = await net.retrieve<AccessResponse>(
        AccessResponse,
        event,
        "/api/v2/access",
        {
          headers: {
            ...(token && { Authorization: token }),
          },
        },
      );

      return {
        access: true,
        role: res.admin ? "admin" : "user",
      };
    } catch (e) {
      if (e instanceof FetchError) {
        switch (e.statusCode) {
          case 200:
            return {
              access: true,
              role: "user",
            };
          case 401:
            return {
              access: false,
              redirect: e.response?.headers.get("location") || null,
            };
        }
      }

      throw e;
    }
  },
};
