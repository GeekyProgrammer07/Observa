import { afterEach, beforeEach, describe, expect, it, vi, type Mock } from "vitest";
import { ApiError, auth, monitors, notifications } from "./api";

function jsonResponse(body: unknown, status = 200) {
  return {
    ok: status >= 200 && status < 300,
    status,
    statusText: "OK",
    text: async () => JSON.stringify(body),
    json: async () => body,
  } as Response;
}

function errorResponse(status: number, body = "", statusText = "Error") {
  return {
    ok: false,
    status,
    statusText,
    text: async () => body,
    json: async () => {
      throw new Error("no body");
    },
  } as unknown as Response;
}

describe("api client", () => {
  let fetchMock: Mock;

  beforeEach(() => {
    fetchMock = vi.fn().mockResolvedValue(jsonResponse({}));
    vi.stubGlobal("fetch", fetchMock);
    localStorage.clear();
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  describe("request plumbing", () => {
    it("prefixes every path with /api/v1 and sends cookies", async () => {
      await monitors.list();

      expect(fetchMock).toHaveBeenCalledTimes(1);
      const [url, init] = fetchMock.mock.calls[0];
      expect(url).toBe("/api/v1/monitors");
      expect(init.credentials).toBe("include");
    });

    it("sends Content-Type: application/json on every request", async () => {
      await monitors.list();

      const [, init] = fetchMock.mock.calls[0];
      expect(init.headers["Content-Type"]).toBe("application/json");
    });

    it("omits the Authorization header when no token is stored", async () => {
      await monitors.list();

      const [, init] = fetchMock.mock.calls[0];
      expect(init.headers.Authorization).toBeUndefined();
    });

    it("attaches a Bearer token from localStorage when present", async () => {
      localStorage.setItem("access_token", "tok-123");

      await monitors.list();

      const [, init] = fetchMock.mock.calls[0];
      expect(init.headers.Authorization).toBe("Bearer tok-123");
    });

    it("returns the parsed JSON body on success", async () => {
      const payload = [{ id: "m1" }];
      fetchMock.mockResolvedValue(jsonResponse(payload));

      await expect(monitors.list()).resolves.toEqual(payload);
    });

    it("resolves to undefined for 204 No Content responses", async () => {
      fetchMock.mockResolvedValue({
        ok: true,
        status: 204,
        statusText: "No Content",
        text: async () => "",
        json: async () => {
          throw new Error("204 has no body");
        },
      });

      await expect(monitors.delete("m1")).resolves.toBeUndefined();
    });

    it("throws ApiError carrying the status and response body on failure", async () => {
      fetchMock.mockResolvedValue(errorResponse(409, "username taken"));

      const err = await auth
        .signup({ firstname: "a", lastname: "b", username: "c", password: "d" })
        .catch((e) => e);

      expect(err).toBeInstanceOf(ApiError);
      expect(err.status).toBe(409);
      expect(err.message).toBe("username taken");
      expect(err.name).toBe("ApiError");
    });

    it("falls back to statusText when the error body is empty", async () => {
      fetchMock.mockResolvedValue(errorResponse(500, "", "Internal Server Error"));

      const err = await monitors.list().catch((e) => e);

      expect(err).toBeInstanceOf(ApiError);
      expect(err.message).toBe("Internal Server Error");
    });

    it("still throws ApiError when reading the error body itself fails", async () => {
      fetchMock.mockResolvedValue({
        ok: false,
        status: 502,
        statusText: "Bad Gateway",
        text: async () => {
          throw new Error("stream broken");
        },
      });

      const err = await monitors.list().catch((e) => e);

      expect(err).toBeInstanceOf(ApiError);
      expect(err.status).toBe(502);
      expect(err.message).toBe("Bad Gateway");
    });
  });

  describe("auth endpoints", () => {
    it("POSTs signup data to /api/v1/signup", async () => {
      const data = {
        firstname: "Ada",
        lastname: "Lovelace",
        username: "ada",
        password: "secret",
      };
      await auth.signup(data);

      const [url, init] = fetchMock.mock.calls[0];
      expect(url).toBe("/api/v1/signup");
      expect(init.method).toBe("POST");
      expect(JSON.parse(init.body)).toEqual(data);
    });

    it("POSTs credentials to /api/v1/signin", async () => {
      await auth.signin({ username: "ada", password: "secret" });

      const [url, init] = fetchMock.mock.calls[0];
      expect(url).toBe("/api/v1/signin");
      expect(init.method).toBe("POST");
      expect(JSON.parse(init.body)).toEqual({ username: "ada", password: "secret" });
    });
  });

  describe("monitor endpoints", () => {
    it("GETs the monitor list", async () => {
      await monitors.list();
      const [url, init] = fetchMock.mock.calls[0];
      expect(url).toBe("/api/v1/monitors");
      expect(init.method).toBeUndefined();
    });

    it("POSTs new monitors with the full payload", async () => {
      const data = { url: "https://example.com", name: "Example", interval: 60 };
      await monitors.create(data);

      const [url, init] = fetchMock.mock.calls[0];
      expect(url).toBe("/api/v1/monitors");
      expect(init.method).toBe("POST");
      expect(JSON.parse(init.body)).toEqual(data);
    });

    it("PATCHes /monitors/:id/pause", async () => {
      await monitors.pause("abc");
      const [url, init] = fetchMock.mock.calls[0];
      expect(url).toBe("/api/v1/monitors/abc/pause");
      expect(init.method).toBe("PATCH");
    });

    it("PATCHes /monitors/:id/resume", async () => {
      await monitors.resume("abc");
      const [url, init] = fetchMock.mock.calls[0];
      expect(url).toBe("/api/v1/monitors/abc/resume");
      expect(init.method).toBe("PATCH");
    });

    it("DELETEs /monitors/:id", async () => {
      await monitors.delete("abc");
      const [url, init] = fetchMock.mock.calls[0];
      expect(url).toBe("/api/v1/monitors/abc");
      expect(init.method).toBe("DELETE");
    });
  });

  describe("notification channel endpoints", () => {
    it("GETs the channel list", async () => {
      await notifications.list();
      expect(fetchMock.mock.calls[0][0]).toBe("/api/v1/notification-channels");
    });

    it("POSTs new channels", async () => {
      await notifications.create({ channel_type: "Email", value: "a@b.com" });

      const [url, init] = fetchMock.mock.calls[0];
      expect(url).toBe("/api/v1/notification-channels");
      expect(init.method).toBe("POST");
      expect(JSON.parse(init.body)).toEqual({ channel_type: "Email", value: "a@b.com" });
    });

    it("POSTs /notification-channels/:id/verify", async () => {
      await notifications.verify("ch1");
      const [url, init] = fetchMock.mock.calls[0];
      expect(url).toBe("/api/v1/notification-channels/ch1/verify");
      expect(init.method).toBe("POST");
    });

    it("DELETEs /notification-channels/:id", async () => {
      await notifications.delete("ch1");
      const [url, init] = fetchMock.mock.calls[0];
      expect(url).toBe("/api/v1/notification-channels/ch1");
      expect(init.method).toBe("DELETE");
    });
  });
});
