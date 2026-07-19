import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { MemoryRouter, Route, Routes } from "react-router-dom";
import { beforeEach, describe, expect, it, vi } from "vitest";
import LoginPage from "./Login";
import { useAuth } from "@/lib/auth";
import { ApiError } from "@/lib/api";

vi.mock("@/lib/auth", () => ({
  useAuth: vi.fn(),
}));

const signin = vi.fn();

function renderLogin() {
  return render(
    <MemoryRouter initialEntries={["/login"]}>
      <Routes>
        <Route path="/login" element={<LoginPage />} />
        <Route path="/dashboard" element={<p>dashboard landing</p>} />
      </Routes>
    </MemoryRouter>
  );
}

function fillAndSubmit(username = "ada", password = "secret") {
  fireEvent.change(screen.getByLabelText("Username"), {
    target: { value: username },
  });
  fireEvent.change(screen.getByLabelText("Password"), {
    target: { value: password },
  });
  fireEvent.click(screen.getByRole("button", { name: "Sign in" }));
}

describe("LoginPage", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(useAuth).mockReturnValue({
      token: null,
      isAuthenticated: false,
      isLoading: false,
      signin,
      signup: vi.fn(),
      signout: vi.fn(),
    });
  });

  it("renders the sign-in form", () => {
    renderLogin();

    expect(screen.getByText("Welcome back")).toBeTruthy();
    expect(screen.getByLabelText("Username")).toBeTruthy();
    expect(screen.getByLabelText("Password")).toBeTruthy();
    expect(screen.getByRole("button", { name: "Sign in" })).toBeTruthy();
  });

  it("signs in with the entered credentials and navigates to the dashboard", async () => {
    signin.mockResolvedValue(undefined);
    renderLogin();

    fillAndSubmit("ada", "secret");

    await waitFor(() => expect(screen.getByText("dashboard landing")).toBeTruthy());
    expect(signin).toHaveBeenCalledWith("ada", "secret");
  });

  it("shows a credentials error on a 401 response", async () => {
    signin.mockRejectedValue(new ApiError(401, "unauthorized"));
    renderLogin();

    fillAndSubmit();

    expect(await screen.findByText("Invalid username or password.")).toBeTruthy();
    expect(screen.queryByText("dashboard landing")).toBeNull();
  });

  it("shows a generic error for non-401 failures", async () => {
    signin.mockRejectedValue(new ApiError(500, "boom"));
    renderLogin();

    fillAndSubmit();

    expect(
      await screen.findByText("Something went wrong. Please try again.")
    ).toBeTruthy();
  });

  it("shows a generic error for network-level failures", async () => {
    signin.mockRejectedValue(new TypeError("fetch failed"));
    renderLogin();

    fillAndSubmit();

    expect(
      await screen.findByText("Something went wrong. Please try again.")
    ).toBeTruthy();
  });

  it("clears a previous error on a successful retry", async () => {
    signin.mockRejectedValueOnce(new ApiError(401, "unauthorized"));
    renderLogin();

    fillAndSubmit();
    expect(await screen.findByText("Invalid username or password.")).toBeTruthy();

    signin.mockResolvedValueOnce(undefined);
    fillAndSubmit();

    await waitFor(() => expect(screen.getByText("dashboard landing")).toBeTruthy());
  });

  it("links to the signup page", () => {
    renderLogin();
    const link = screen.getByRole("link", { name: "Sign up" });
    expect(link.getAttribute("href")).toBe("/signup");
  });
});
