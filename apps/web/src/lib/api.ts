import type { AuthTokens, Monitor, NotificationChannel } from "@/types";
import { createLogger } from "./logger";

const BASE = "/api/v1";
const log = createLogger("api");

function getToken(): string | null {
  return localStorage.getItem("access_token");
}

function authHeaders(): Record<string, string> {
  const token = getToken();
  return token ? { Authorization: `Bearer ${token}` } : {};
}

async function request<T>(
  path: string,
  options: RequestInit = {}
): Promise<T> {
  const method = options.method ?? "GET";
  log.debug(`${method} ${BASE}${path}`);

  let res: Response;
  try {
    res = await doFetch(path, options);
  } catch (err) {
    log.error(`${method} ${BASE}${path} network failure`, err);
    throw err;
  }

  if (!res.ok) {
    const text = await res.text().catch(() => "");
    log.error(`${method} ${BASE}${path} failed with ${res.status}`, text);
    throw new ApiError(res.status, text || res.statusText);
  }

  if (res.status === 204) return undefined as T;
  return res.json();
}

function doFetch(path: string, options: RequestInit): Promise<Response> {
  return fetch(`${BASE}${path}`, {
    headers: {
      "Content-Type": "application/json",
      ...authHeaders(),
      ...(options.headers as Record<string, string>),
    },
    credentials: "include",
    ...options,
  });
}

export class ApiError extends Error {
  constructor(
    public status: number,
    message: string
  ) {
    super(message);
    this.name = "ApiError";
  }
}

export const auth = {
  signup: (data: {
    firstname: string;
    lastname: string;
    username: string;
    password: string;
  }) => request<{ id: string }>("/signup", { method: "POST", body: JSON.stringify(data) }),

  signin: (data: { username: string; password: string }) =>
    request<AuthTokens>("/signin", { method: "POST", body: JSON.stringify(data) }),
};

export const monitors = {
  list: () => request<Monitor[]>("/monitors"),

  create: (data: {
    url: string;
    name?: string;
    interval?: number;
    timeout_ms?: number;
    is_paused?: boolean;
  }) => request<{ monitor_id: string }>("/monitors", { method: "POST", body: JSON.stringify(data) }),

  pause: (id: string) =>
    request<{ message: string }>(`/monitors/${id}/pause`, { method: "PATCH" }),

  resume: (id: string) =>
    request<{ message: string }>(`/monitors/${id}/resume`, { method: "PATCH" }),

  delete: (id: string) =>
    request<void>(`/monitors/${id}`, { method: "DELETE" }),
};

export const notifications = {
  list: () => request<NotificationChannel[]>("/notification-channels"),

  create: (data: { channel_type: "Email" | "Webhook"; value: string }) =>
    request<{ channel_id: string }>("/notification-channels", {
      method: "POST",
      body: JSON.stringify(data),
    }),

  verify: (id: string) =>
    request<{ message: string }>(`/notification-channels/${id}/verify`, {
      method: "POST",
    }),

  delete: (id: string) =>
    request<void>(`/notification-channels/${id}`, { method: "DELETE" }),
};
