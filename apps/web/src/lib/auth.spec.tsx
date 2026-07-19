import { act, renderHook, waitFor } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { AuthProvider, useAuth } from "./auth";
import { auth as authApi } from "./api";

vi.mock("./api", () => ({
  auth: {
    signin: vi.fn(),
    signup: vi.fn(),
  },
}));

const wrapper = ({ children }: { children: React.ReactNode }) => (
  <AuthProvider>{children}</AuthProvider>
);

describe("AuthProvider", () => {
  beforeEach(() => {
    localStorage.clear();
    vi.clearAllMocks();
  });

  it("starts unauthenticated when localStorage has no token", async () => {
    const { result } = renderHook(() => useAuth(), { wrapper });

    await waitFor(() => expect(result.current.isLoading).toBe(false));
    expect(result.current.token).toBeNull();
    expect(result.current.isAuthenticated).toBe(false);
  });

  it("hydrates the token from localStorage on mount", async () => {
    localStorage.setItem("access_token", "stored-token");

    const { result } = renderHook(() => useAuth(), { wrapper });

    await waitFor(() => expect(result.current.isLoading).toBe(false));
    expect(result.current.token).toBe("stored-token");
    expect(result.current.isAuthenticated).toBe(true);
  });

  it("signin stores the access token and flips isAuthenticated", async () => {
    vi.mocked(authApi.signin).mockResolvedValue({
      access_token: "fresh-token",
      token_type: "Bearer",
      expires_in: 3600,
    });

    const { result } = renderHook(() => useAuth(), { wrapper });
    await waitFor(() => expect(result.current.isLoading).toBe(false));

    await act(() => result.current.signin("ada", "secret"));

    expect(authApi.signin).toHaveBeenCalledWith({ username: "ada", password: "secret" });
    expect(localStorage.getItem("access_token")).toBe("fresh-token");
    expect(result.current.token).toBe("fresh-token");
    expect(result.current.isAuthenticated).toBe(true);
  });

  it("signin failure propagates and leaves the user signed out", async () => {
    vi.mocked(authApi.signin).mockRejectedValue(new Error("bad credentials"));

    const { result } = renderHook(() => useAuth(), { wrapper });
    await waitFor(() => expect(result.current.isLoading).toBe(false));

    await expect(
      act(() => result.current.signin("ada", "wrong"))
    ).rejects.toThrow("bad credentials");

    expect(localStorage.getItem("access_token")).toBeNull();
    expect(result.current.isAuthenticated).toBe(false);
  });

  it("signup delegates to the api without touching the token", async () => {
    vi.mocked(authApi.signup).mockResolvedValue({ id: "u1" });

    const { result } = renderHook(() => useAuth(), { wrapper });
    await waitFor(() => expect(result.current.isLoading).toBe(false));

    const data = { firstname: "Ada", lastname: "L", username: "ada", password: "pw" };
    await act(() => result.current.signup(data));

    expect(authApi.signup).toHaveBeenCalledWith(data);
    expect(result.current.isAuthenticated).toBe(false);
    expect(localStorage.getItem("access_token")).toBeNull();
  });

  it("signout clears the token from state and localStorage", async () => {
    localStorage.setItem("access_token", "stored-token");

    const { result } = renderHook(() => useAuth(), { wrapper });
    await waitFor(() => expect(result.current.isAuthenticated).toBe(true));

    act(() => result.current.signout());

    expect(result.current.token).toBeNull();
    expect(result.current.isAuthenticated).toBe(false);
    expect(localStorage.getItem("access_token")).toBeNull();
  });

  it("useAuth throws when used outside an AuthProvider", () => {
    const spy = vi.spyOn(console, "error").mockImplementation(() => undefined);
    expect(() => renderHook(() => useAuth())).toThrow(
      "useAuth must be used within AuthProvider"
    );
    spy.mockRestore();
  });
});
